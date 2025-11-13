# JP - 日语学习命令行工具

一个用 Rust 编写的命令行工具，帮助你学习日语。支持罗马音查询（带大字 ASCII art 单词卡效果）、中文翻译，以及基于 LLM 的智能翻译功能。

## 功能特性

### 1. 罗马音查询（单词卡效果）
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

### 使用 cargo install（推荐）

使用 `cargo install --path .` 安装到本地 Cargo bin 目录：

```bash
# 在项目根目录执行
cargo install --locked --path .

# 安装完成后可执行文件在：
# Windows: %USERPROFILE%\.cargo\bin\
# Linux/macOS: $HOME/.cargo/bin/

# 确保该目录已在 PATH 中，然后直接运行
jp chi
```

### 从源码编译

```bash
# 克隆或进入项目目录
cd jp

# 编译 release 版本
cargo build --release

# 可执行文件位于
./target/release/jp.exe
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
├── src/
│   └── main.rs            # 主程序源码
├── scripts/               # 开发工具脚本
│   ├── generate_ascii_art.py      # 生成假名ASCII art
│   ├── update_main_rs.py          # 自动更新main.rs
│   ├── hiragana_ascii_art.txt     # 生成的平假名ASCII art
│   └── katakana_ascii_art.txt     # 生成的片假名ASCII art
├── target/
│   └── release/
│       └── jp.exe         # 编译后的可执行文件
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
