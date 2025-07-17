# /publish-website - sxl 网站发布指令

## 概述

自动化sxl网站发布流程，包括前端编译、清空目标目录、复制静态文件等完整步骤。

## 使用方法

直接输入 `/publish-website` 即可执行完整的发布流程。

## 发布流程

该指令会自动执行以下步骤：

### 1. 前端编译（自动清空目标目录）
```bash
cd /opt/src/sxl/src/frontend && npm run build
```
*注：Vite 配置 `outDir: '../public'` 和 `emptyOutDir: true` 会自动清空并输出到目标目录*

### 2. 验证发布结果
```bash
ls -la /opt/src/sxl/public/
```

### 3. 检查关键文件
```bash
# 检查主页面
test -f /opt/src/sxl/public/index.html && echo "✅ index.html 存在" || echo "❌ index.html 缺失"

# 检查静态资源
test -d /opt/src/sxl/public/assets && echo "✅ assets 目录存在" || echo "❌ assets 目录缺失"

# 检查数据文件
test -d /opt/src/sxl/public/data && echo "✅ data 目录存在" || echo "❌ data 目录缺失"

# 检查图片资源
test -d /opt/src/sxl/public/images && echo "✅ images 目录存在" || echo "❌ images 目录缺失"
```

### 4. Playwright 网站验证（可选但推荐）
```bash
cd /opt/src/sxl && node scripts/verify-website.js
```

**验证内容包括：**
- 🌐 页面能否正常加载 (http://Web3mh.101.so:11181/)
- 🎨 CSS 样式是否正确应用
- 📱 响应式设计在移动端的表现
- 🔍 关键页面元素是否存在：
  - 导航栏和 品牌标识
  - 核心价值主张："架起Web3学习鸿沟的桥梁"
  - 主要CTA按钮："立即找导师"
  - 损失规避区域
  - 收益矩阵区域
  - FAQ区域
  - 页脚信息
- ⚡ JavaScript 运行状态
- 📦 静态资源加载情况

## 注意事项

- **自动化流程**：Vite 配置已优化，编译时自动清空目标目录并输出
- **静态资源**：`src/frontend/public/` 中的文件会自动复制到输出目录
- **配置文件**：关键配置在 `vite.config.ts` 中，`outDir: '../public'` 指定输出目录
- **网络验证**：Playwright 验证需要确保目标网站 `http://sxl.101.so:11180/` 可访问
- **依赖环境**：确保已安装 Playwright 和 Chromium 浏览器
- **权限检查**：确保有足够权限进行文件操作

## 使用场景

- 前端代码更新后需要发布到生产环境
- 修改样式、组件或页面内容后的发布
- 定期的网站内容更新发布
- **新增**：验证线上网站是否正确反映最新更改
- **新增**：自动化质量保证和回归测试

## 故障排查

### 编译失败
- 检查 frontend 目录中的依赖是否完整
- 运行 `npm install` 安装缺失的依赖
- 检查 TypeScript 类型错误

### 文件复制失败
- 检查源目录和目标目录的权限
- 确认磁盘空间充足
- 验证文件路径是否正确

### 验证发布结果
- 检查 `/opt/src/sxl/public/index.html` 是否存在
- 验证 assets 目录中的静态资源是否完整
- 确认 data 目录中的 JSON 文件是否正确复制

### Playwright 验证失败

#### 网络连接问题
```bash
# 检查网站是否可访问
curl -I http://sxl.101.so:11180/
```

#### 浏览器启动失败
```bash
# 检查 Playwright 浏览器是否已安装
npx playwright install chromium
```

#### 页面加载超时
- 检查网站服务器状态
- 确认 Nginx 配置正确
- 验证防火墙和端口访问

#### 元素定位失败
- 检查页面是否正确渲染最新内容
- 验证 CSS 选择器是否正确
- 确认页面元素是否按预期存在

#### 常见错误排查
```bash
# 手动运行验证脚本查看详细错误
cd /opt/src/sxl
DEBUG=pw:api node scripts/verify-website.js

# 检查 Playwright 依赖
npm list playwright

# 重新安装 Playwright 浏览器
npx playwright install
```

## 关联文件

- `/opt/src/sxl/src/frontend/` - 前端源代码目录
- `/opt/src/sxl/src/frontend/public/` - 静态资源目录（会自动复制）
- `/opt/src/sxl/src/frontend/vite.config.ts` - Vite 构建配置
- `/opt/src/sxl/public/` - 最终发布目录（编译输出）
- `/opt/src/sxl/scripts/verify-website.js` - Playwright 网站验证脚本
- `/opt/src/sxl/package.json` - Playwright 依赖配置

## 快速执行

**完整发布流程（推荐）：**
```bash
cd /opt/src/sxl/src/frontend && npm run build && node /opt/src/sxl/scripts/verify-website.js
```

**仅编译发布：**
```bash
cd /opt/src/sxl/src/frontend && npm run build
```

**仅验证网站：**
```bash
node /opt/src/sxl/scripts/verify-website.js
```

**调试模式验证：**
```bash
cd /opt/src/sxl && DEBUG=pw:api node scripts/verify-website.js
```

## 版本记录

- v1.0 - 初始版本，支持基本的编译和发布流程
- v1.1 - 添加 Playwright 自动化验证功能，检验线上网站加载情况