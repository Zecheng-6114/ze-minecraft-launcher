# ze-minecraft-launcher - 项目上下文文档

## 项目概述

**ze-minecraft-launcher** 是一个使用 Rust 编程语言开发的 Minecraft 启动器项目。这是一个初始阶段的项目，目前包含基本的 Rust 项目结构和 "Hello, world!" 示例代码。

## 项目类型
- **编程语言**: Rust
- **构建系统**: Cargo (Rust 的包管理和构建工具)
- **项目状态**: 初始开发阶段

## 技术栈
- **Rust 版本**: 2024 edition
- **依赖管理**: Cargo.toml
- **构建工具**: Cargo

## 项目结构
```
ze-minecraft-launcher/
├── Cargo.toml          # Rust 项目配置文件
├── Cargo.lock          # 依赖锁定文件
├── README.md           # 项目说明文档
├── src/
│   └── main.rs         # 主程序入口文件
├── target/             # 构建输出目录
└── .idea/              # IntelliJ IDEA 配置文件
```

## 构建和运行

### 构建项目
```bash
# 调试构建
cargo build

# 发布构建
cargo build --release
```

### 运行项目
```bash
# 直接运行
cargo run

# 运行测试
cargo test
```

### 代码检查
```bash
# 代码格式化
cargo fmt

# 代码检查
cargo clippy
```

## 开发环境设置

### 前提条件
1. 安装 Rust 工具链：
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. 验证安装：
   ```bash
   rustc --version
   cargo --version
   ```

### IDE 支持
- 项目包含 IntelliJ IDEA 配置文件，支持 Rust 插件
- 推荐使用 RustRover 或 IntelliJ IDEA with Rust plugin

## 项目状态

### 当前实现
- 基础 Rust 项目结构
- 简单的 "Hello, world!" 程序
- 基本的 Cargo 配置

### 待实现功能
根据项目名称推测，需要实现以下功能：
1. Minecraft 版本管理
2. 游戏文件下载和验证
3. 用户认证（Mojang/Microsoft）
4. Java 运行时管理
5. 游戏启动参数配置
6. 用户界面（CLI 或 GUI）

## 开发约定

### 代码风格
- 遵循 Rust 官方编码规范
- 使用 `cargo fmt` 保持代码格式一致
- 使用 `cargo clippy` 进行代码质量检查

### 依赖管理
- 所有依赖在 `Cargo.toml` 中声明
- 使用语义化版本控制
- 定期更新依赖版本

### 测试
- 单元测试放在与被测试代码相同的文件中
- 集成测试放在 `tests/` 目录中
- 使用 Rust 内置的测试框架

## 贡献指南

1. Fork 项目仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 故障排除

### 常见问题
1. **构建失败**: 确保 Rust 工具链已正确安装
2. **依赖下载失败**: 检查网络连接，或使用国内镜像源
3. **IDE 无法识别项目**: 重新导入项目或更新 Rust 插件

### 调试技巧
- 使用 `cargo run --verbose` 查看详细构建信息
- 使用 `RUST_BACKTRACE=1 cargo run` 查看完整的错误堆栈

## 相关资源
- [Rust 官方文档](https://www.rust-lang.org/learn)
- [Cargo 手册](https://doc.rust-lang.org/cargo/)
- [Minecraft 启动器开发指南](https://wiki.vg/Launching_the_game)

---

*本文档最后更新于: 2025年12月27日*
*项目版本: 0.1.0*