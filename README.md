# JP - 日语学习命令行工具

一个用 Rust 编写的命令行工具，帮助你学习日语。支持罗马音查询（带大字 ASCII art 单词卡效果）、中文翻译，以及基于 LLM 的智能翻译功能。

## 功能特性

### 桌面应用（jp-desktop）

- 图形化界面展示假名学习卡片
- 可设置定时弹窗复习（默认10分钟）
- 支持开机自启动
- 自动记忆窗口大小和位置
- 可选择学习模式：罗马音/中文/混合

### 命令行工具（jp）

#### 1. 罗马音查询（单词卡效果）
输入罗马音，显示对应的平假名、片假名（使用从真实日文字体生成的 ASCII art 大字效果，占据约15行×50字符）和例词。

```bash
jp chi
```
**特点：**
- 假名使用真实日文字体（MS Gothic/MS Mincho）渲染后转换成 ASCII art
- 每个假名占据约 **15 行高度 × 50 字符宽度**，大字效果清晰易读
- 使用 1000×1000 像素高分辨率渲染，字体大小 800pt
- ASCII art 使用丰富的字符集（` .':;-=+*#%@█`）真实还原假名的笔画和形状
- 同时显示平假名和片假名
- 包含3个例词帮助记忆

### 2. 中文翻译（本地字典）
输入中文，查询本地字典中的日语翻译。

```bash
jp 你好
```

输出：
```
╔═══════════════════════════════════════════════
║ Chinese (中文): 你好
║ Japanese (日文): こんにちは (konnichiwa)
╚═══════════════════════════════════════════════
```

### 3. AI 智能翻译（LLM）
当本地字典中找不到对应的中文翻译时，自动调用 OpenAI compatible API 进行翻译。

```bash
jp 我喜欢编程
```

输出：
```
╔═══════════════════════════════════════════════
║ 🔍 本地字典未找到，正在使用 LLM 翻译...
╠═══════════════════════════════════════════════
║ Chinese (中文): 我喜欢编程
║ Japanese (日文): 私はプログラミングが好きです (わたしはプログラミングがすきです/watashi wa puroguramingu ga suki desu)
║
║ 💡 提示：这是由 AI 生成的翻译
╚═══════════════════════════════════════════════
```

## 安装

本项目包含两个程序：
- **jp**：命令行工具（CLI）
- **jp-desktop**：桌面应用（GUI），带定时弹窗复习功能

### 使用 cargo install（推荐）

```bash
# 在项目根目录执行

# 方式1：同时安装 CLI 和桌面应用
cargo install --path . --bins --features desktop

# 方式2：只安装 CLI
cargo install --path . --bin jp

# 方式3：只安装桌面应用
cargo install --path . --bin jp-desktop --features desktop

# 安装完成后可执行文件在：
# Windows: %USERPROFILE%\.cargo\bin\
# Linux/macOS: $HOME/.cargo/bin/

# 确保该目录已在 PATH 中，然后直接运行
jp chi          # 命令行工具
jp-desktop      # 桌面应用
```

### 从源码编译

```bash
# 克隆或进入项目目录
cd jp

# 编译 CLI 版本
cargo build --release

# 编译桌面应用
cargo build --release --bin jp-desktop --features desktop

# 可执行文件位于
./target/release/jp.exe          # CLI
./target/release/jp-desktop.exe  # 桌面应用
```

### 构建 Windows 安装包

桌面应用支持打包成 Windows 安装程序（NSIS）：

```bash
# 需要先安装 Tauri CLI
cargo install tauri-cli --version "^2.0.0"

# 构建安装包
cargo tauri build --features desktop

# 安装包位于（根据 Tauri 版本可能有所不同）：
# src-tauri/target/release/bundle/nsis/JP Desktop_0.1.0_x64-setup.exe
```

### 添加到 PATH（可选）

将 `target/release/jp.exe` 复制到你的 PATH 目录中，或者将该目录添加到 PATH 环境变量。

## 配置 LLM 功能

要使用 AI 智能翻译功能，需要设置以下环境变量：

### 必需的环境变量

```bash
# Windows (PowerShell)
$env:OPENAI_API_KEY = "your-api-key-here"

# Windows (CMD)
set OPENAI_API_KEY=your-api-key-here

# Linux/macOS
export OPENAI_API_KEY=your-api-key-here
```

### 可选的环境变量

```bash
# 自定义 API 地址（用于兼容 OpenAI 的 API）
export OPENAI_API_BASE=https://api.openai.com/v1

# 自定义模型
export OPENAI_MODEL=gpt-3.5-turbo
```

### 支持的 API 提供商

由于使用 OpenAI compatible API，本工具支持：

- OpenAI 官方 API
- Azure OpenAI
- 国内各种兼容 OpenAI 的 API（如：智谱、通义千问、Kimi 等）
- 自部署的兼容服务（如 LocalAI、Ollama 等）

## 本地字典包含的内容

### 罗马音（五十音图）
- 完整的平假名和片假名对照
- 每个假名包含 3 个常用例词

### 中文词汇
- **问候语**：你好、谢谢、对不起、再见等
- **数字**：一到十
- **家庭成员**：父亲、母亲、哥哥、姐姐等
- **颜色**：红色、蓝色、白色、黑色等
- **季节**：春天、夏天、秋天、冬天
- **星期**：星期一到星期日
- **常用词汇**：学习、朋友、家、学校等

## 使用示例

### 桌面应用

```bash
# 直接启动桌面应用
jp-desktop

# 或者从源码运行
cargo run --release --bin jp-desktop --features desktop
```

### 命令行工具

```bash
# 查询罗马音
jp ka
jp chi
jp tsu

# 查询常用中文词
jp 谢谢
jp 学习
jp 春天

# 查询复杂句子（需要配置 LLM）
jp 今天天气真好
jp 我正在学习日语
```

## 项目结构

```
jp/
├── Cargo.toml              # 项目配置和依赖
├── tauri.conf.json        # Tauri 桌面应用配置
├── src/
│   ├── main.rs            # CLI 主程序
│   └── desktop.rs         # 桌面应用主程序
├── desktop-ui/            # 桌面应用前端页面
│   ├── index.html
│   ├── script.js
│   └── styles.css
├── scripts/               # 开发工具脚本
│   ├── generate_ascii_art.py      # 生成假名ASCII art
│   ├── update_main_rs.py          # 自动更新main.rs
│   ├── hiragana_ascii_art.txt     # 生成的平假名ASCII art
│   └── katakana_ascii_art.txt     # 生成的片假名ASCII art
├── target/
│   └── release/
│       ├── jp.exe         # CLI 编译后的可执行文件
│       └── jp-desktop.exe # 桌面应用可执行文件
└── README.md              # 本文件
```

## 开发指南

如果需要修改ASCII art的显示效果(比如改变大小、字符集等):

1. 修改 `scripts/generate_ascii_art.py` 中的参数
2. 运行生成脚本:
   ```bash
   cd scripts
   python generate_ascii_art.py
   ```
3. 自动更新main.rs:
   ```bash
   python update_main_rs.py
   ```
4. 重新编译:
   ```bash
   cd ..
   cargo build --release
   ```

## 技术栈

- **语言**: Rust
- **命令行解析**: clap
- **HTTP 客户端**: reqwest
- **JSON 序列化**: serde, serde_json
- **异步运行时**: tokio

## 许可证

MIT License

## 贡献

欢迎提交 Issue 和 Pull Request！
