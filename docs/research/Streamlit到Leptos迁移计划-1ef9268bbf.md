# Streamlit到Leptos迁移计划
https://space.coze.cn/s/hcgRmu14Fkk/
## 1. 项目概述
本文档详细描述如何将基于Streamlit的交互式故事应用安全简洁地迁移到Rust的Leptos框架，并打包为Linux平台的单一执行文件。

## 2. 技术栈对比
| 功能 | Streamlit (Python) | Leptos (Rust) |
|------|-------------------|---------------|
| 状态管理 | `st.session_state` | `Signal`/`RwSignal` + Context |
| UI渲染 | 声明式API | 响应式组件 + `view!`宏 |
| 路由系统 | 无内置支持 | `leptos_router` |
| 配置文件 | `toml`库 | `toml` + `serde` |
| 打包方式 | 依赖Python环境 | 静态编译为单一二进制 |

## 3. 详细迁移步骤

### 3.1 项目初始化
1. 创建Leptos项目
```bash
cargo new leptos_story_app --bin
cd leptos_story_app
```

2. 添加依赖到`Cargo.toml`
```toml
[package]
name = "leptos_story_app"
version = "0.1.0"
edition = "2021"

[dependencies]
leptos = { version = "0.6", features = ["csr", "nightly"] }
leptos_router = { version = "0.6", features = ["csr"] }
toml = "0.8"
serde = { version = "1.0", features = ["derive"] }
regex = "1.10"
tokio = { version = "1.0", features = ["full"] }
```

3. 创建Trunk配置文件`Trunk.toml`
```toml
[build]
target = "index.html"
dist = "dist"
public_url = "/"

[serve]
port = 8080
```

### 3.2 数据模型实现
1. 定义TOML数据结构（`src/model.rs`）
```rust
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct StoryData {
    #[serde(rename = "FM_CHOICE")]
    pub fm_choice: HashMap<String, ChoiceNode>,
    
    #[serde(rename = "FM_STORY")]
    pub fm_story: HashMap<String, StoryNode>,
    
    #[serde(rename = "FM_START")]
    pub fm_start: StoryNode,
    
    #[serde(rename = "FM_NOEND")]
    pub fm_noend: StoryNode,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChoiceNode {
    pub title: String,
    pub story: String,
    pub red: String,
    pub blue: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StoryNode {
    pub title: String,
    pub story: String,
}

impl StoryData {
    pub fn load() -> Result<Self, String> {
        let content = std::fs::read_to_string("FM_STORY.toml")
            .map_err(|e| format!("无法读取故事文件: {}", e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("TOML解析错误: {}", e))
    }
}
```

### 3.3 状态管理实现
1. 定义全局应用状态（`src/state.rs`）
```rust
use leptos::*;
use std::fmt;

#[derive(Clone)]
pub struct AppState {
    pub choice_path: RwSignal<String>,
    pub current_level: RwSignal<u32>,
    pub story_data: StoryData,
}

impl fmt::Debug for AppState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppState")
            .field("choice_path", &self.choice_path.get())
            .field("current_level", &self.current_level.get())
            .finish()
    }
}

pub fn provide_app_state(cx: Scope, story_data: StoryData) {
    let choice_path = create_rw_signal(cx, String::new());
    let current_level = create_rw_signal(cx, 1);
    
    provide_context(cx, AppState {
        choice_path,
        current_level,
        story_data,
    });
}

pub fn use_app_state(cx: Scope) -> AppState {
    use_context(cx).expect("AppState not provided")
}
```

### 3.4 UI组件实现
1. 创建流式文本组件（`src/components/stream_text.rs`）
```rust
use leptos::*;
use regex::Regex;
use std::time::Duration;

#[component]
pub fn StreamText(cx: Scope, text: String) -> impl IntoView {
    let (chunks, set_chunks) = create_signal(cx, Vec::new());
    let (loading, set_loading) = create_signal(cx, true);
    
    spawn_local(async move {
        let re = Regex::new(r"[^，。！？；：\s]+[，。！？；：\s]*").unwrap();
        let chunks: Vec<&str> = re.find_iter(&text).map(|m| m.as_str()).collect();
        
        for chunk in chunks {
            set_chunks.update(|v| v.push(chunk.to_string()));
            tokio::time::sleep(Duration::from_millis(180)).await;
        }
        
        set_loading.set(false);
    });
    
    view! { cx,
        <Suspense fallback=move || view! { cx, <p class="text-gray-500">加载中...</p> }>
            <div class="prose max-w-none">
                {move || chunks.get().into_iter()
                    .map(|chunk| view! { cx, <span>{chunk}</span> })
                    .collect_view(cx)}
                {move || if !loading.get() { view! { cx, <br/> } } else { None }}
            </div>
        </Suspense>
    }
}
```

2. 创建主应用组件（`src/app.rs`）
```rust
use leptos::*;
use leptos_router::*;
use super::{model::*, state::*, components::stream_text::StreamText};

#[derive(Routable, Clone)]
pub enum AppRoutes {
    #[route("/")]
    Home,
    #[route("/story/:path")]
    Story { path: String },
    #[fallback]
    NotFound,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // 加载故事数据
    let story_data = match StoryData::load() {
        Ok(data) => data,
        Err(e) => {
            return view! { cx, <div class="text-red-500 p-4">错误加载故事数据: {e}</div> }
        }
    };
    
    // 提供全局状态
    provide_app_state(cx, story_data);
    
    view! { cx,
        <Router>
            <main class="container mx-auto p-4 max-w-6xl">
                <Routes>
                    <Route path="/" view=HomePage />
                    <Route path="/story/:path" view=StoryPage />
                    <Route path="/*any" view=NotFoundPage />
                </Routes>
            </main>
        </Router>
    }
}

// 主页组件实现...
// 故事页面组件实现...
// 未找到页面组件实现...
```

### 3.5 路由与导航实现
1. 实现故事导航逻辑（`src/navigation.rs`）
```rust
use leptos::*;
use leptos_router::use_navigate;
use super::state::use_app_state;

pub fn navigate_to_choice(cx: Scope, choice: &str) {
    let app_state = use_app_state(cx);
    let navigate = use_navigate(cx);
    
    // 更新路径
    app_state.choice_path.update(|path| path.push_str(choice));
    // 更新层级
    app_state.current_level.update(|level| *level += 1);
    
    // 导航到新路径
    let new_path = app_state.choice_path.get();
    navigate(&format!("/story/{}", new_path), Default::default());
}

pub fn reset_story(cx: Scope) {
    let app_state = use_app_state(cx);
    let navigate = use_navigate(cx);
    
    app_state.choice_path.set(String::new());
    app_state.current_level.set(1);
    navigate("/", Default::default());
}
```

### 3.6 样式实现
1. 创建Tailwind配置文件（`tailwind.config.js`）
```javascript
module.exports = {
  content: ["./src/**/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [],
}
```

2. 添加全局样式（`src/style.css`）
```css
@tailwind base;
@tailwind components;
@tailwind utilities;

/* 自定义Streamlit风格样式 */
.stRadio > div {
  @apply flex space-x-4;
}

.stRadio > div > label {
  @apply px-4 py-2 rounded-md font-bold cursor-pointer transition-all;
}

.stRadio > div > label:nth-child(1) {
  @apply bg-gradient-to-r from-red-500 to-red-400 text-white;
}

.stRadio > div > label:nth-child(2) {
  @apply bg-gradient-to-r from-blue-500 to-blue-400 text-white;
}
```

### 3.7 打包配置
1. 添加构建脚本（`build.rs`）
```rust
fn main() {
    // 复制TOML文件到输出目录
    let out_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::copy("FM_STORY.toml", format!("{}/FM_STORY.toml", out_dir))
        .expect("Failed to copy FM_STORY.toml");
}
```

2. 创建构建脚本（`build.sh`）
```bash
#!/bin/bash
set -e

# 安装Musl目标
rustup target add x86_64-unknown-linux-musl

# 构建应用
cargo build --release --target x86_64-unknown-linux-musl

# 复制二进制文件
cp target/x86_64-unknown-linux-musl/release/leptos_story_app .

echo "构建完成: $(pwd)/leptos_story_app"
```

## 4. 单元测试方案

### 4.1 数据模型测试
创建`tests/model_test.rs`：
```rust
use leptos_story_app::model::StoryData;

#[test]
fn test_story_data_loading() {
    // 测试TOML文件加载
    let story_data = StoryData::load().expect("Failed to load story data");
    
    // 验证基本结构
    assert!(!story_data.fm_choice.is_empty());
    assert!(!story_data.fm_story.is_empty());
    assert!(!story_data.fm_start.title.is_empty());
    assert!(!story_data.fm_start.story.is_empty());
}

#[test]
fn test_choice_node_structure() {
    let story_data = StoryData::load().expect("Failed to load story data");
    let choice_node = story_data.fm_choice.get("0").expect("Level 0 choice not found");
    
    assert!(!choice_node.title.is_empty());
    assert!(!choice_node.story.is_empty());
    assert!(!choice_node.red.is_empty());
    assert!(!choice_node.blue.is_empty());
}
```

### 4.2 状态管理测试
创建`tests/state_test.rs`：
```rust
use leptos::*;
use leptos_story_app::{state::*, model::StoryData};

#[test]
fn test_app_state_management() {
    create_scope_immediate(|cx| {
        // 创建测试数据
        let story_data = StoryData {
            fm_choice: Default::default(),
            fm_story: Default::default(),
            fm_start: Default::default(),
            fm_noend: Default::default(),
        };
        
        // 提供状态
        provide_app_state(cx, story_data);
        let app_state = use_app_state(cx);
        
        // 测试初始状态
        assert_eq!(app_state.choice_path.get(), "");
        assert_eq!(app_state.current_level.get(), 1);
        
        // 更新状态
        app_state.choice_path.set("R".to_string());
        app_state.current_level.set(2);
        
        // 验证更新
        assert_eq!(app_state.choice_path.get(), "R");
        assert_eq!(app_state.current_level.get(), 2);
    });
}
```

### 4.3 组件测试
创建`tests/component_test.rs`：
```rust
use leptos::*;
use leptos_test::*;
use leptos_story_app::components::stream_text::StreamText;

#[test]
fn test_stream_text_component() {
    create_scope_immediate(|cx| {
        let text = "测试流式文本输出。这是一个测试。".to_string();
        let comp = view! { cx, <StreamText text=text /> };
        
        // 渲染组件
        let rendered = render_to_string(cx, comp);
        
        // 验证基本结构
        assert!(rendered.contains("加载中..."));
    });
}
```

### 4.4 集成测试
创建`tests/integration_test.rs`：
```rust
use leptos::*;
use leptos_router::*;
use leptos_test::*;
use leptos_story_app::App;

#[test]
fn test_app_initial_render() {
    create_scope_immediate(|cx| {
        // 模拟路由上下文
        let app = view! { cx,
            <MemoryRouter initial_entries=vec!["/".to_string()]>
                <App />
            </MemoryRouter>
        };
        
        // 渲染应用
        let rendered = render_to_string(cx, app);
        
        // 验证应用渲染
        assert!(rendered.contains("未来之门 - Life 3.0"));
    });
}
```

## 5. 安全考量
1. **输入验证**：对所有用户输入进行严格验证，特别是故事路径参数
2. **文件权限**：确保TOML文件读取权限适当，避免敏感信息泄露
3. **依赖审计**：定期使用`cargo audit`检查依赖安全漏洞
4. **静态链接**：使用Musl libc静态链接避免动态依赖问题
5. **错误处理**：实现优雅的错误处理，避免暴露内部实现细节

## 6. 部署与运行
1. 构建单一可执行文件
```bash
chmod +x build.sh
./build.sh
```

2. 运行应用
```bash
./leptos_story_app
```

3. 访问应用
打开浏览器访问 http://localhost:8080

## 7. 迁移风险与应对措施
| 风险 | 应对措施 |
|------|---------|
| TOML结构不兼容 | 编写兼容层处理不同版本的TOML结构 |
| 性能问题 | 使用Leptos的性能优化功能，如`memo`和`create_resource` |
| 浏览器兼容性 | 针对目标浏览器进行测试，添加必要的polyfill |
| 功能迁移不完整 | 编写详细的测试用例，确保所有功能正常工作 |

## 8. 结论
本迁移计划提供了将Streamlit应用迁移到Leptos框架的详细步骤，包括项目设置、数据模型、状态管理、UI组件、路由导航、样式实现、打包配置和测试方案。通过遵循此计划，可以安全、简洁地完成迁移，并获得一个高性能、可维护的Rust单一执行文件应用。