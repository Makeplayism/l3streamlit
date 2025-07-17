# frontend-dev

前端开发常用命令集合

## 使用方法
输入 `/frontend-dev` 查看前端开发相关命令

## 环境准备
```bash
# 确保使用正确的 Node.js 版本 (LTS v22.16.0)
nvm use
```

## 依赖管理
```bash
# 安装依赖 (使用 pnpm 提高性能)
pnpm install

# 更新依赖
pnpm update

# 查看过期依赖
pnpm outdated
```

## 开发命令
```bash
# 开发服务器
pnpm dev

# 生产构建
pnpm build

# 预览生产构建
pnpm preview
```

## 代码质量
```bash
# 类型检查
pnpm type-check

# 代码检查
pnpm lint

# 代码格式化
pnpm format
```

## 测试命令
```bash
# 运行单元测试
pnpm test

# 运行端到端测试
pnpm test:e2e

# 运行测试覆盖率
pnpm test:coverage
```

## 常用端口
- 开发服务器: http://localhost:5173
- 预览服务器: http://localhost:4173