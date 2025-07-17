# setup-gh-aliases

设置 GitHub CLI 的 Emoji-Log 提交别名

## 使用方法
输入 `/setup-gh-aliases` 查看并设置所有 gh 别名

## 快速设置
执行以下命令一键设置所有别名：
```bash
# 下载并执行设置脚本
curl -fsSL https://raw.githubusercontent.com/VCCampus/web3mh/main/setup-gh-aliases.sh | bash

# 或者手动执行
wget -O setup-gh-aliases.sh https://raw.githubusercontent.com/VCCampus/web3mh/main/setup-gh-aliases.sh
chmod +x setup-gh-aliases.sh && ./setup-gh-aliases.sh
```

## 手动设置命令

### 核心提交命令
```bash
# 📦 NEW: 添加全新功能
gh alias set --shell new 'git add . && git commit -m "📦 NEW: $1" && git push'

# 👌 IMPROVE: 改进或重构代码
gh alias set --shell imp 'git add . && git commit -m "👌 IMPROVE: $1" && git push'

# 🐛 FIX: 修复bug
gh alias set --shell fix 'git add . && git commit -m "🐛 FIX: $1" && git push'

# 📖 DOC: 文档更新
gh alias set --shell doc 'git add . && git commit -m "📖 DOC: $1" && git push'

# 🚀 RELEASE: 版本发布
gh alias set --shell rlz 'git add . && git commit -m "🚀 RELEASE: $1" && git push'

# ✅ TEST: 测试相关
gh alias set --shell tst 'git add . && git commit -m "✅ TEST: $1" && git push'
```

### 扩展提交命令
```bash
# 🔒 SECURITY: 安全修复
gh alias set --shell sec 'git add . && git commit -m "🔒 SECURITY: $1" && git push'

# ‼️ BREAKING: 破坏性变更
gh alias set --shell brk 'git add . && git commit -m "‼️ BREAKING: $1" && git push'

# ⚡ PERFORMANCE: 性能优化
gh alias set --shell perf 'git add . && git commit -m "⚡ PERFORMANCE: $1" && git push'

# 🎨 STYLE: 代码格式化
gh alias set --shell style 'git add . && git commit -m "🎨 STYLE: $1" && git push'

# ♻️ REFACTOR: 代码重构
gh alias set --shell refactor 'git add . && git commit -m "♻️ REFACTOR: $1" && git push'

# 🔧 CONFIG: 配置更改（避免与 gh config 冲突）
gh alias set --shell gconfig 'git add . && git commit -m "🔧 CONFIG: $1" && git push'

# 🏗️ BUILD: 构建系统
gh alias set --shell build 'git add . && git commit -m "🏗️ BUILD: $1" && git push'

# 👷 CI: CI/CD 相关（避免与潜在命令冲突）
gh alias set --shell gci 'git add . && git commit -m "👷 CI: $1" && git push'

# ⬆️ DEPS: 依赖更新
gh alias set --shell dep 'git add . && git commit -m "⬆️ DEPS: $1" && git push'

# ⏪ REVERT: 回滚代码
gh alias set --shell revert 'git add . && git commit -m "⏪ REVERT: $1" && git push'
```

## 使用示例
```bash
# 添加新功能
gh new "用户注册功能"

# 修复bug（自动关联 Issue）
gh fix "登录验证问题 Fixes #123"

# 改进代码
gh imp "优化数据库查询性能"

# 更新文档
gh doc "更新 API 文档"

# 发布版本
gh rlz "v1.0.0"
```

## 验证设置
```bash
# 查看所有已设置的别名
gh alias list

# 测试某个别名（不会实际提交）
gh alias delete test --confirm >/dev/null 2>&1
gh alias set --shell test 'echo "将执行: git add . && git commit -m \"👌 IMPROVE: $1\" && git push"'
gh test "测试提交消息"
```

## Issue 关联规则
- `#123` - 引用 Issue
- `Fixes #123` - 自动关闭 Issue
- `Refs #123` - 仅引用不关闭

## 参考
基于 [ahmadawais/Emoji-Log](https://github.com/ahmadawais/Emoji-Log/) 规范