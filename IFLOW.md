# ze-minecraft-launcher - 项目上下文文档

## 项目概述

**ze-minecraft-launcher** 是一个使用 Rust 编程语言开发的 Minecraft 启动器项目。该项目基于 `egui/eframe` 框架构建，提供了图形用户界面（GUI）原型，具备中文显示支持。项目已集成 `lyceris` 库，为实际的 Minecraft 游戏启动功能奠定基础。

## 项目类型
- **编程语言**: Rust (2024 edition)
- **GUI 框架**: egui/eframe
- **Minecraft 启动库**: lyceris
- **构建系统**: Cargo (Rust 的包管理和构建工具)
- **项目状态**: 开发中 - GUI 原型与核心功能集成阶段

## 技术栈
- **Rust 版本**: 2024 edition
- **GUI 框架**: eframe 0.33.3 (基于 egui)
- **Minecraft 启动**: lyceris 1.1.3
- **字体管理**: fontdb 0.23.0
- **依赖管理**: Cargo.toml
- **构建工具**: Cargo

## 项目结构
```
ze-minecraft-launcher/
├── Cargo.toml          # Rust 项目配置文件
├── Cargo.lock          # 依赖锁定文件
├── README.md           # 项目说明文档
├── src/
│   └── main.rs         # GUI 应用程序主入口文件
├── target/             # 构建输出目录
└── .idea/              # IntelliJ IDEA 配置文件
```

## 架构说明

应用程序采用 eframe 作为 GUI 框架，结合 lyceris 库提供 Minecraft 启动功能：

1. **主程序入口** (`main` 函数):
   - 初始化 eframe 应用程序
   - 加载系统字体并配置中文字体支持
   - 启动 GUI 事件循环

2. **字体加载系统**:
   - 使用 `fontdb` 扫描系统字体
   - 自动识别常见中文字体（微软雅黑、思源黑体等）
   - 将字体数据注入 egui 渲染系统

3. **Minecraft 启动核心**:
   - 集成 `lyceris` 库 (v1.1.3)
   - 提供 Minecraft 游戏启动、版本管理、认证等核心功能
   - 计划与 GUI 界面深度集成

4. **应用程序结构** (`MyApp`):
   - 实现 `eframe::App` trait
   - 管理 UI 状态和事件处理
   - 提供基本的启动器界面组件
   - 预留 `lyceris` 集成接口

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
# 启动 GUI 应用程序
cargo run

# 运行测试（暂无测试文件）
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
- Visual Studio Code 配合 rust-analyzer 扩展也可获得良好体验

## 项目状态

### 当前实现
- 基于 egui/eframe 的图形用户界面
- 系统字体自动加载，支持中文显示
- 基本 UI 组件：标题、标签、按钮、分隔线
- 字体加载逻辑，支持常见中文字体：
  - Microsoft YaHei (Windows)
  - SimHei, SimSun (Windows)
  - PingFang SC (macOS)
  - Source Han Sans SC, Noto Sans CJK SC (Linux)
  - WenQuanYi Micro Hei (Linux)
- 应用程序框架结构 (`MyApp` 结构体和 `eframe::App` trait 实现)
- 基本的交互逻辑（按钮点击事件）
- **新增** `lyceris` 1.1.3 依赖集成，为 Minecraft 启动功能提供基础

### 待实现功能（按优先级）
1. **集成 lyceris 库** - 将 lyceris 功能集成到 GUI 中
2. **Minecraft 版本管理** - 使用 lyceris 实现版本列表、下载和切换
3. **用户认证**（Mojang/Microsoft）- 集成 lyceris 认证功能
4. **游戏启动逻辑** - 实现实际的游戏启动流程
5. **Java 运行时管理** - JRE 版本检测和配置
6. **游戏文件下载和验证** - 整合 lyceris 的资源管理
7. **游戏更新检查功能** - 自动检查游戏更新
8. **设置界面和配置管理** - 用户偏好设置
9. **游戏日志查看器** - 实时查看游戏日志
10. **多账户支持** - 多个 Minecraft 账户管理
11. **模组/资源包管理** - 扩展功能支持

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
- 集成测试放在 `tests/` 目录中（待创建）
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
4. **GUI 窗口无法启动**: 检查系统图形驱动和显示服务器
5. **中文显示为方块**: 确保系统安装了中文字体（如微软雅黑、思源黑体等）
6. **lyceris 集成问题**: 检查 lyceris 库的文档和兼容性

### 调试技巧
- 使用 `cargo run --verbose` 查看详细构建信息
- 使用 `RUST_BACKTRACE=1 cargo run` 查看完整的错误堆栈
- 检查控制台输出中的字体加载信息
- 在 `main.rs` 中添加 `println!` 语句调试字体加载过程
- 参考 lyceris 库的示例代码进行集成调试

## 相关资源
- [Rust 官方文档](https://www.rust-lang.org/learn)
- [Cargo 手册](https://doc.rust-lang.org/cargo/)
- [eframe 文档](https://docs.rs/eframe) - egui 应用程序框架
- [egui 文档](https://www.egui.rs/) - 即时模式 GUI 库
- [fontdb 文档](https://docs.rs/fontdb) - 字体数据库管理
- [lyceris 文档](https://docs.rs/lyceris) - Minecraft 启动库
- [lyceris GitHub 仓库](https://github.com/heroiclabs/lyceris) - 源代码和示例
- [Minecraft 启动器开发指南](https://wiki.vg/Launching_the_game)

---

*本文档最后更新于: 2025年12月28日*
*项目版本: 0.1.0*
*基于 eframe 0.33.3, fontdb 0.23.0 和 lyceris 1.1.3*