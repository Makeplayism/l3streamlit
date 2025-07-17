# chk-self - 项目配置自检指令

这个内部指令用于完成项目配置的系统性检查和修正，确保开发环境的稳定性和最佳实践。基于 `/doctor` 命令功能增强，整合了实际项目诊断经验。

## 使用方法

```bash
# 在项目根目录运行
/chk-self
```

## 检查项目和解决方案

### 0. 快速总览检查

**目的**: 快速了解项目整体健康状况
**检查命令**:
```bash
cd lovable/
echo "🔍 项目健康检查总览"
echo "======================"
npm audit --summary
npm run type-check 2>/dev/null && echo "✅ TypeScript: 无错误" || echo "❌ TypeScript: 发现错误"
npm run lint --silent 2>/dev/null && echo "✅ ESLint: 通过" || echo "⚠️ ESLint: 发现警告/错误"
npm run build >/dev/null 2>&1 && echo "✅ 构建: 成功" || echo "❌ 构建: 失败"
echo "======================"
```

### 1. 包管理器冲突检查

**问题**: 检查是否存在多个包管理器锁文件
**检查命令**:
```bash
ls -la | grep -E "(package-lock|bun\.lock|yarn\.lock|pnpm-lock)"
```

**解决方案**:
- 选择一个包管理器（推荐 npm）
- 删除其他锁文件
- 示例：`rm bun.lockb yarn.lock pnpm-lock.yaml`

### 2. 安全漏洞检查

**问题**: 检查依赖包的安全漏洞
**检查命令**:
```bash
npm audit
```

**常见问题及解决方案**:

**2024年7月实际发现的漏洞**:
- `esbuild <=0.24.2`: 开发服务器可能接受任意请求（中等严重性）
- `vite 0.11.0 - 6.1.6`: 依赖于易受攻击的 esbuild 版本
- `lovable-tagger`: 依赖于易受攻击的 vite 版本

**解决方案**:
```bash
# 1. 尝试自动修复
npm audit fix

# 2. 如果自动修复无效，手动更新关键依赖
npm update vite@latest esbuild@latest

# 3. 对于开发依赖的中等风险漏洞
# 如果影响仅限于开发环境，可以选择暂时接受风险
# 但应定期检查更新

# 4. 生产环境部署前验证
npm audit --production
```

### 3. TypeScript 配置检查

**问题**: 检查 TypeScript 配置是否足够严格
**检查文件**:
- `tsconfig.json`
- `tsconfig.app.json`
- `tsconfig.node.json`

**关键配置项**:
```json
{
  "compilerOptions": {
    "strict": true,
    "noImplicitAny": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "strictNullChecks": true,
    "noFallthroughCasesInSwitch": true
  }
}
```

**解决方案**:
- 启用严格模式
- 确保所有配置文件的一致性
- 添加类型检查脚本到 `package.json`

### 4. 构建配置检查

**问题**: 检查构建脚本和配置
**检查命令**:
```bash
npm run build
npm run type-check
npm run lint
```

**解决方案**:
- 确保所有脚本都能正常运行
- 添加缺失的脚本（如 `type-check`）
- 修复构建错误

### 5. 代码质量检查

**问题**: 检查 ESLint 配置和代码质量
**检查命令**:
```bash
npm run lint
```

**常见 ESLint 警告及解决方案**:

**Fast Refresh 组件导出警告**（2024年7月实际发现）:
- 文件: `badge.tsx`, `button.tsx`, `form.tsx`, `navigation-menu.tsx`, `sidebar.tsx`, `sonner.tsx`, `toggle.tsx`
- 问题: 组件文件同时导出了组件和常量/函数，影响 React Fast Refresh
- 警告: `Fast refresh only works when a file only exports components`

**解决方案**:
```bash
# 方案1: 创建单独的常量文件
# 将常量和工具函数移动到单独的 .ts 文件中

# 方案2: 忽略特定文件的警告（适用于 UI 库组件）
# 在文件顶部添加：
# /* eslint-disable react-refresh/only-export-components */

# 方案3: 更新 ESLint 配置排除 UI 组件目录
# 在 eslint.config.js 中添加：
# { ignores: ["src/components/ui/**/*.tsx"] }
```

**其他代码质量检查**:
- 修复 linting 错误
- 更新 ESLint 配置
- 添加 pre-commit hooks（可选）

### 6. 依赖版本检查

**问题**: 检查过期的依赖包
**检查命令**:
```bash
npm outdated
```

**2024年7月实际发现的过期依赖** (共65个过期包):

**关键框架更新**:
- `React: 18.3.1 -> 19.1.0` (主要版本更新，需谨慎)
- `Vite: 5.4.19 -> 7.0.4` (主要版本更新，需谨慎)
- `TypeScript: 5.6.3 -> 5.8.3` (小版本更新，相对安全)

**UI 库更新**:
- `@radix-ui/*`: 多个组件有小版本更新
- `Tailwind CSS: 3.4.17 -> 4.1.11` (主要版本更新，需谨慎)

**解决方案**:
```bash
# 1. 安全的小版本更新
npm update @radix-ui/react-* @types/* eslint typescript

# 2. 主要版本更新需要谨慎测试
# React 19 和 Vite 7 包含破坏性变更，建议单独处理

# 3. 测试驱动的更新流程
npm run test      # 运行测试（如果有）
npm run build     # 验证构建
npm run lint      # 检查代码质量

# 4. 生产验证
npm run preview   # 预览构建结果
```

### 7. 项目结构检查

**问题**: 检查项目结构是否符合规范
**检查项目**:
- 根目录整洁性
- 文件夹结构
- 命名约定

**解决方案**:
- 重新组织文件结构
- 移除不必要的文件
- 遵循项目规范

### 8. 网站内容验证检查 🆕

**问题**: 检查已部署网站的内容完整性和功能正常性
**基于**: `simple-test.sh` 脚本实现

**检查命令**:
```bash
# 网站可访问性检查
curl -s -o /dev/null -w "%{http_code}" http://web3mh.101.so:11182/ | grep -q "200"

# 关键内容检查
page_content=$(curl -s http://web3mh.101.so:11182/)
echo "$page_content" | grep -q "校园VC" && echo "✅ 网站标题正常"
echo "$page_content" | grep -q "产品展示" && echo "✅ 产品展示区域存在" 
echo "$page_content" | grep -q "活动现场照片" && echo "✅ 照片墙区域存在"
echo "$page_content" | grep -q "home-.*\.webp" && echo "✅ WebP图片正确加载"
echo "$page_content" | grep -q "京ICP备" && echo "✅ ICP备案信息存在"
```

**解决方案**:
- 检查构建输出是否完整复制到 `public/` 目录
- 验证静态资源路径配置
- 确认备案信息包含在页脚组件中
- 检查图片优化和 WebP 格式转换

### 9. Git 仓库状态检查 🆕

**问题**: 检查 Git 仓库的清洁状态和分支同步
**检查命令**:
```bash
git status
git log --oneline -5   # 查看最近5次提交
git branch -v          # 查看分支状态
```

**常见问题**:
- 未跟踪的文件：`simple-test.sh`, `website-screenshot.png`, `website-test.js`
- 删除的文件：`.claude/chk-self.md` (已移动到 `commands/` 目录)
- 分支领先远程仓库

**解决方案**:
```bash
# 添加有用的文件
git add simple-test.sh website-test.js
git add .claude/commands/chk-self.md

# 清理不需要的文件
rm website-screenshot.png  # 临时截图文件

# 提交并推送
git commit -m "📦 NEW: 添加网站测试脚本和自检指令"
git push origin main
```

## 完整自检流程

**执行环境**: 项目根目录 `/opt/src/campus-vc-genesis/`

### Phase 1: 快速诊断 (2分钟)

```bash
# 0. 快速总览
cd lovable/
echo "🔍 项目健康检查总览"
echo "======================"
npm audit --summary 2>/dev/null
npm run type-check 2>/dev/null && echo "✅ TypeScript: 无错误" || echo "❌ TypeScript: 发现错误"
npm run lint --silent 2>/dev/null && echo "✅ ESLint: 通过" || echo "⚠️ ESLint: 发现警告"
npm run build >/dev/null 2>&1 && echo "✅ 构建: 成功" || echo "❌ 构建: 失败"
echo "======================"

# 1. Git 状态检查
cd ..
git status --porcelain | head -10
```

### Phase 2: 详细检查 (5-10分钟)

```bash
cd lovable/

# 2. 包管理器冲突检查
ls -la | grep -E "(package-lock|bun\.lock|yarn\.lock|pnpm-lock)"

# 3. 安全漏洞详细检查
npm audit
# 注意: esbuild/vite 漏洞为开发环境中等风险

# 4. TypeScript 详细检查
npm run type-check

# 5. ESLint 详细检查  
npm run lint
# 注意: 7个 Fast Refresh 警告为 UI 组件库正常现象

# 6. 构建完整测试
npm run build

# 7. 依赖版本检查
npm outdated | head -20
# 注意: 65个过期包，React/Vite 主版本更新需谨慎
```

### Phase 3: 网站验证 (3分钟)

```bash
cd ..

# 8. 网站内容验证 
if command -v curl >/dev/null; then
  echo "🌐 网站内容检查..."
  ./simple-test.sh 2>/dev/null || echo "⚠️ 网站检查脚本未运行"
else
  echo "⚠️ curl 不可用，跳过网站检查"
fi

# 9. Git 仓库清理
git status
git log --oneline -3
```

### Phase 4: 问题修复 (按需执行)

```bash
# 仅在发现问题时执行相应修复命令

# 安全修复 (谨慎执行)
cd lovable/
npm audit fix

# ESLint 警告修复 (可选)
# 添加 UI 组件忽略规则到 eslint.config.js

# 依赖更新 (分阶段)
npm update @types/* typescript eslint  # 安全更新
# npm update @radix-ui/react-*          # UI 库更新  
# npm update vite@6.x.x                 # 主要版本谨慎更新

# Git 清理
cd ..
git add .claude/commands/chk-self.md simple-test.sh website-test.js
# rm website-screenshot.png  # 删除临时文件
```

## 输出格式

检查完成后，输出以下格式的报告：

### 2024年7月17日实际检查结果示例：

```
🔍 项目健康检查总览
======================
✅ TypeScript: 无错误
⚠️ ESLint: 发现警告 (7个 Fast Refresh 警告)
✅ 构建: 成功
⚠️ 安全漏洞: 发现 3个中等风险 (开发环境)
⚠️ 依赖版本: 发现 65个过期包
✅ 网站内容: 正常运行
⚠️ Git 状态: 未跟踪文件需清理
======================

📊 详细诊断摘要:
- 包管理器: ✅ npm (无冲突)
- 安全性: ⚠️ esbuild/vite 开发环境漏洞
- 代码质量: ⚠️ UI组件Fast Refresh警告
- 构建系统: ✅ 正常 (10.24s)
- 网站功能: ✅ 备案信息完整，图片优化正常
- 版本管理: ⚠️ 需要清理和同步
```

### 标准输出格式模板：

```
✅ 包管理器配置: 正常 / ❌ 发现冲突
✅ 安全漏洞: 无风险 / ⚠️ 中等风险 / ❌ 高风险
✅ TypeScript 配置: 无错误 / ❌ 发现错误
✅ 构建脚本: 正常 / ❌ 构建失败
⚠️ 代码质量: 发现 X 个警告 / ✅ 通过检查
⚠️ 依赖版本: 发现 X 个过期包 / ✅ 版本最新
✅ 项目结构: 符合规范 / ❌ 发现问题需要修正
✅ 网站内容: 正常运行 / ❌ 功能异常
⚠️ Git 仓库: 需要清理 / ✅ 状态干净
```

## 注意事项

1. **备份重要文件**: 在进行配置修改前，确保重要文件已备份
2. **渐进式修改**: 特别是 TypeScript 严格模式，可以逐步启用
3. **测试验证**: 每次修改后都要运行测试确保功能正常
4. **文档更新**: 修改配置后及时更新相关文档

## 自动化建议

可以考虑将这些检查集成到：
- GitHub Actions CI/CD 流程
- Pre-commit hooks
- 开发环境启动脚本
- 定期的健康检查任务

## 版本历史

- **v1.0.0** (初版): 初始版本，包含基础配置检查
- **v1.1.0**: 添加 TypeScript 严格模式检查  
- **v1.2.0**: 添加项目结构规范检查
- **v2.0.0** (2024-07-17): 🆕 重大更新
  - 整合 `/doctor` 命令功能和实际诊断经验
  - 添加快速总览检查 (Phase 1)
  - 新增网站内容验证检查 (基于 simple-test.sh)
  - 新增 Git 仓库状态检查
  - 添加实际问题案例和解决方案:
    - esbuild/vite 安全漏洞处理
    - Fast Refresh ESLint 警告解决
    - 65个过期依赖包管理策略
  - 分阶段检查流程 (快速诊断 → 详细检查 → 网站验证 → 问题修复)
  - 实际检查结果示例和标准输出格式
  - 支持自动化和 CI/CD 集成建议

### 更新说明

**v2.0.0** 是基于真实 `/doctor` 命令使用场景的全面重构：

1. **实战导向**: 所有检查项目都基于2024年7月17日的实际诊断结果
2. **分阶段执行**: 从2分钟快速检查到完整的10分钟详细诊断
3. **风险分级**: 区分高/中/低风险问题，提供差异化处理策略
4. **自动化友好**: 支持脚本化执行和 CI/CD 集成
5. **文档驱动**: 详细的问题描述、解决方案和最佳实践

### 升级指南

从 v1.x 升级到 v2.0：
```bash
# 1. 更新命令文件
cp .claude/commands/chk-self.md .claude/commands/chk-self.md.backup
# 使用新版本替换

# 2. 运行新的分阶段检查
/chk-self  # 自动使用新版本流程

# 3. 根据实际结果调整项目配置
```