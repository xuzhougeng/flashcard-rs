# JP Desktop - 日文学习桌面应用

基于 Tauri 构建的日文学习桌面应用，每隔一段时间自动弹出学习卡片。

## 功能特性

- 🎴 **自动弹出学习卡片**：可配置时间间隔（5-60分钟）
- 🔄 **混合学习模式**：支持假名和中文词汇混合学习
- 🎯 **系统托盘**：最小化到托盘，可暂停/恢复提醒
- ⚙️ **可配置**：自定义时间间隔、卡片类型
- 🚀 **开机自启**：可选择开机自动启动

## 构建说明

### 前置要求

1. **Rust 工具链**：已安装（项目已有）
2. **WebView2**（Windows）：Windows 10/11 自带或需要安装
   - 下载：https://developer.microsoft.com/microsoft-edge/webview2/

### 图标准备

在构建前，需要准备应用图标：

```bash
# 创建 icons 目录
mkdir icons

# 需要以下图标文件：
# - icon.png (1024x1024 PNG，应用主图标)
# - icon.ico (Windows 图标)
# - tray-icon.png (32x32 PNG，托盘图标)
```

如果暂时没有图标，可以使用任意 PNG 图片占位。

### 构建命令

```bash
# 开发模式（带调试窗口）
cargo run --bin jp-desktop --features desktop

# 发布版本（优化构建）
cargo build --bin jp-desktop --features desktop --release

# 生成安装包
cargo tauri build --bin jp-desktop --features desktop
```

## 使用说明

### 启动应用

```bash
# 直接运行
cargo run --bin jp-desktop --features desktop

# 或运行编译后的可执行文件
.\target\release\jp-desktop.exe
```

### 主要功能

1. **查看卡片**：
   - 应用启动后会立即显示一张随机卡片
   - 每隔设定的时间间隔，窗口会自动弹出显示新卡片

2. **手动刷新**：
   - 点击"🔄 下一张卡片"按钮可手动切换卡片

3. **设置配置**：
   - 点击"⚙️ 设置"按钮打开设置面板
   - 可配置：提醒间隔、开机自启、卡片类型

4. **系统托盘**：
   - 右键点击托盘图标可以：
     - 显示卡片
     - 暂停/恢复提醒
     - 打开设置
     - 退出应用

### 卡片类型

- **混合（假名+中文词）**：随机显示假名或中文词翻译
- **仅假名**：只显示日文假名学习卡
- **仅中文词**：只显示中日词汇翻译

## 技术栈

- **后端**：Rust + Tauri 2.0
- **前端**：HTML + CSS + Vanilla JavaScript
- **插件**：
  - `tauri-plugin-autostart`：开机自启功能
  - `tauri-plugin-store`：设置持久化

## 项目结构

```
jp/
├── src/
│   ├── main.rs          # CLI 版本主程序
│   └── desktop.rs       # 桌面版主程序
├── desktop-ui/          # 桌面应用前端
│   ├── index.html
│   ├── styles.css
│   └── script.js
├── icons/               # 应用图标
├── tauri.conf.json      # Tauri 配置
├── build.rs             # 构建脚本
└── Cargo.toml           # 依赖配置
```

## 已知问题

1. 首次构建需要下载较多依赖，请耐心等待
2. Windows Defender 可能误报，需要添加信任
3. 图标文件需要手动准备

## 开发计划

- [ ] 添加学习统计功能
- [ ] 支持自定义词库
- [ ] 添加语音朗读
- [ ] 跨平台支持（macOS、Linux）
