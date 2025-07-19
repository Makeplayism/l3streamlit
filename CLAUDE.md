# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is **l3streamlit** - a Chinese interactive story-based game inspired by the "Life 3.0" book. The game presents players with binary choices about humanity's future relationship with AI, creating branching narratives based on their decisions.

**🚀 MIGRATION STATUS**: Successfully migrated from Streamlit (Python) to Leptos (Rust) - see `/src/` directory for the new implementation.

## Dual Architecture

### Legacy Implementation (Streamlit)
- **Location**: `/app/` directory
- **Status**: Legacy/Reference implementation
- **Technology**: Python + Streamlit
- **Main Application**: `/app/main.py` - Entry point containing the Streamlit UI and game logic
- **Story Navigator**: `StoryNavigator` class handles story data parsing and path navigation
- **Configuration**: `/app/mod/settings.py` - Centralized configuration using python-box

### Current Implementation (Leptos/Rust) 🆕
- **Location**: `/src/` directory  
- **Status**: Migration complete, ready for validation
- **Technology**: Rust + Leptos + Axum + WASM
- **Architecture**: Modular component-based design (15 modules)
- **Story Data**: Shared `/docs/FM_STORY.toml` with legacy version
- **Compilation**: ✅ `cargo check` passed successfully

#### Rust Project Structure
```
/src/
├── models/           # 数据模型层
│   ├── story.rs      # 故事数据结构 (StoryData, StoryContent)
│   ├── choice.rs     # 选择类型定义 (ChoiceType, Choice)
│   └── game_state.rs # 游戏状态管理 (GameState)
├── components/       # Leptos UI 组件
│   ├── app.rs        # 主应用根组件
│   ├── story_display.rs    # 故事展示组件 (含流式文本)
│   ├── choice_buttons.rs   # 红/蓝选择按钮组件
│   ├── story_tree.rs       # ASCII 路径树组件
│   └── control_panel.rs    # 游戏控制面板
├── services/         # 业务逻辑服务
│   ├── story_loader.rs     # TOML 故事数据加载器
│   └── path_navigator.rs   # 故事路径导航器
├── utils/           # 工具函数库
│   ├── text_streaming.rs   # 智能文本流式输出
│   └── ascii_tree.rs       # ASCII 树生成器
├── tests/           # 测试模块
│   ├── story_tests.rs      # 故事数据测试
│   ├── game_state_tests.rs # 游戏状态测试
│   └── integration_tests.rs # 集成测试
├── Cargo.toml       # Rust 项目配置
└── main.rs          # 应用入口点
```

### Data Structure

The game uses a hierarchical choice system:
- **6 levels** of binary choices (Red/Blue)
- **126 possible story combinations** (2^6 paths)
- Each choice path generates unique story content
- Stories are stored in TOML format with nested sections

### Key Features

- **Interactive Story Tree**: ASCII visualization of choice paths
- **Session State Management**: Tracks player progress through `st.session_state`
- **Streaming Text**: Smart text streaming for dramatic effect
- **Path Visualization**: Shows current position in story tree
- **Multiple Endings**: Based on accumulated choices

## Development Commands

### Legacy Streamlit Version
```bash
# Install dependencies
pip install -r requirements.txt

# Run the Streamlit app
streamlit run app/main.py
```

### Current Rust/Leptos Version 🆕
```bash
# Navigate to Rust project
cd src/

# Check project (verify dependencies and syntax)
cargo check

# Run tests
cargo test

# Build for development
cargo build

# Build for production (optimized)
cargo build --release

# Run the server (default: http://localhost:3000)
cargo run

# Run with custom port
LEPTOS_SITE_ADDR="0.0.0.0:18052" cargo run
```

### Development Setup
```bash
# Prerequisites
rustc --version  # Ensure Rust 1.88+ is installed
cargo --version  # Cargo package manager

# The app expects story data at:
# /opt/src/streamlit/l3/docs/FM_STORY.toml

# Key features:
# - SSR + CSR support via Leptos
# - WASM compilation for frontend
# - Axum web server backend
# - Hot reload in development mode
```

## Configuration Files

### Legacy Configuration
- **requirements.txt**: Python dependencies (streamlit, python-box, loguru, toml)

### Current Rust Configuration
- **src/Cargo.toml**: Rust dependencies and project metadata
  - `leptos`: Web framework with SSR/CSR support
  - `axum`: Web server backend
  - `tokio`: Async runtime
  - `serde`: Serialization framework
  - `toml`: TOML parsing library
  - `playwright`: E2E testing framework

### Shared Data Files
- **docs/FM_STORY.toml**: Story content with sections:
  - `FM_CHOICE.0-6`: Choice prompts for each level (6 levels)
  - `FM_STORY.*`: Story branches for each path combination (126 paths)
  - `FM_START`: Initial story setup
  - `FM_NOEND`: Final ending content

## Story Content Structure

Story paths are encoded as strings of 'R' (Red) and 'B' (Blue) choices:
- `"R"` - First level red choice
- `"RB"` - Red then blue choice
- `"RBRBRB"` - Complete 6-level path

Each path in FM_STORY contains:
- `title`: Chapter title
- `story`: Narrative content (supports markdown)

## Technical Notes

### Legacy Streamlit Implementation
- Uses `@st.cache_data` for story data loading
- Session state manages: `choice_path`, `current_level`
- Custom CSS styling for choice buttons (red/blue gradients)
- Supports both English and Chinese content
- Logging configured with custom format and colors

### Current Rust Implementation 🆕
- **Reactive State Management**: Leptos signals replace `st.session_state`
- **Component Architecture**: Modular UI components with props
- **Type Safety**: Rust's type system prevents runtime errors
- **Performance**: Compiled binary with WASM frontend
- **Testing**: Unit tests + Integration tests + E2E with Playwright
- **Memory Safety**: Rust's ownership system prevents common bugs

## Development Environment

### Legacy Environment
- **Python 3.x** with Streamlit framework
- **TOML** for configuration and story data
- **Loguru** for structured logging
- **python-box** for configuration management

### Current Environment 🆕
- **Rust 1.88+** with Cargo package manager
- **Leptos 0.6** web framework
- **Axum 0.7** web server
- **WASM** for frontend compilation
- **Tokio** async runtime
- **DevOps**: Nginx configuration and deployment scripts in `/devops/`

## Project Status

### Migration Status (January 2025)
- ✅ **Legacy Version**: v25.7.17.1542 - Fully functional Streamlit implementation
- 🚀 **Current Version**: v0.1.0 - Rust/Leptos implementation complete
  - All core functionality migrated
  - 15 modules with comprehensive test coverage
  - ✅ Compilation verified (`cargo check` passed)
  - Ready for functional testing and deployment
  - Performance optimizations pending

### Next Phase Development Plan
1. **验证阶段** (Validation Phase)
   - [x] Compilation verification
   - [ ] Functional testing vs legacy version
   - [ ] Performance benchmarking
   
2. **优化阶段** (Optimization Phase) 
   - [ ] CSS styling to match Streamlit UI
   - [ ] Error handling and logging
   - [ ] Production deployment configuration
   
3. **增强阶段** (Enhancement Phase)
   - [ ] Progressive Web App features
   - [ ] User session persistence
   - [ ] Multi-language support

### Current Development Focus
**Primary**: Rust/Leptos implementation in `/src/` - Ready for functional testing
**Secondary**: Legacy maintenance and reference
**Documentation**: Comprehensive development guidance in `/docs/reports/l3streamlit-项目开发指导报告-2025-01-19.md`

### Enhanced Development Tools
- **Claude Code Integration**: 15 specialized commands for systematic development
- **Kiro Workflow**: Complete spec→design→task→execute development pipeline  
- **Analysis Tools**: `/think-ultra`, `/think-harder` for complex problem solving
- **Quality Assurance**: `/reflection` for continuous improvement
- **Knowledge Management**: `/eureka` for breakthrough documentation