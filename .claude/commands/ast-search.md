# ast-search

AST-Grep 结构化搜索使用指南

## 使用方法
输入 `/ast-search` 查看 AST 搜索使用方法

## 搜索约定
- **简单字符串匹配** -> Grep
- **结构化/跨语言模式匹配** -> AST-Grep

## 推荐使用缩写指令: sg

### 基本用法
```bash
# 搜索 TypeScript 中的函数调用
sg --pattern '$PROP && $PROP()' --lang ts TypeScript/src

# 搜索并替换
sg -p '$PROP && $PROP()' \
   --rewrite '$PROP?.()' \
   --interactive \
   -l ts \
   TypeScript/src
```

### 常用参数
- `-p, --pattern`: 搜索模式
- `-l, --lang`: 指定语言 (rust, ts, js, py, 等)
- `--rewrite`: 替换模式
- `--interactive`: 交互式确认
- `-c, --config`: 指定配置文件

## 常用模式示例

### Rust 模式
```bash
# 搜索函数定义
sg -p 'fn $NAME($$$) { $$$ }' -l rust src/

# 搜索结构体
sg -p 'struct $NAME { $$$ }' -l rust src/

# 搜索 impl 块
sg -p 'impl $TYPE { $$$ }' -l rust src/
```

### TypeScript/JavaScript 模式
```bash
# 搜索函数声明
sg -p 'function $NAME($$$) { $$$ }' -l ts src/

# 搜索 React 组件
sg -p 'const $NAME = () => { $$$ }' -l tsx src/

# 搜索类定义
sg -p 'class $NAME { $$$ }' -l ts src/
```

### 通用模式
```bash
# 搜索变量声明
sg -p 'let $VAR = $VALUE' -l ts src/

# 搜索函数调用
sg -p '$FUNC($$$)' -l ts src/

# 搜索属性访问
sg -p '$OBJ.$PROP' -l ts src/
```

## 命令列表
```
Usage: sg [OPTIONS] <COMMAND>

Commands:
  run          运行一次性搜索或重写 (默认命令)
  scan         按配置扫描和重写代码
  test         测试 ast-grep 规则
  new          创建新的 ast-grep 项目或项目
  lsp          启动语言服务器
  completions  生成 shell 自动完成脚本
  docs         生成当前配置的规则文档
  help         打印帮助信息

Options:
  -c, --config <CONFIG_FILE>  ast-grep 根配置路径，默认为 sgconfig.yml
  -h, --help                  打印帮助
  -V, --version               打印版本
```

## 配置文件
可以创建 `sgconfig.yml` 来保存常用规则：
```yaml
rules:
  - id: no-console-log
    message: "Remove console.log statements"
    rule:
      pattern: "console.log($$$)"
    fix: ""
```

## 参考资料
- [AST-Grep 官方文档](https://ast-grep.github.io/)
- [规则参考](https://ast-grep.github.io/cheatsheet/rule.html)
- [中文指南](https://deepwiki.com/ast-grep/ast-grep)