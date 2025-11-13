# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

**JP** is a command-line Japanese learning tool written in Rust. It helps users learn Japanese through romaji lookup (with large ASCII art flashcards), Chinese-to-Japanese translation using a local dictionary, and LLM-powered translation for phrases not in the dictionary.

The project also includes a web-based version (`web/`) that provides similar functionality through a browser interface.

## Common Commands

### Building and Running

```bash
# Build the release version
cargo build --release

# Run the CLI tool directly with cargo (default positional TEXT)
cargo run -- <TEXT>

# Examples:
cargo run -- chi           # Lookup romaji
cargo run -- 你好          # Translate Chinese

# Run the compiled binary (default positional TEXT)
.\\target\\release\\jp.exe <TEXT>

# Optional explicit subcommand form also works
cargo run -- lookup <TEXT>
.\\target\\release\\jp.exe lookup <TEXT>
```

### Testing

```bash
# Run tests (if any are added in the future)
cargo test

# Run with verbose output
cargo test -- --nocaptures
```

### Development

```bash
# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

### ASCII Art Generation

When you need to regenerate or modify the ASCII art for Japanese kana:

```bash
# Generate ASCII art from Japanese fonts
cd scripts
python generate_ascii_art.py

# Update main.rs with the generated art
python update_main_rs.py

# Return to project root and rebuild
cd ..
cargo build --release
```

### Web Version

The web version is a standalone HTML/CSS/JavaScript application:

```bash
# Serve the web UI using the Rust binary (recommended)
# Default: http://127.0.0.1:8080
cargo run -- serve

# Specify host and port
cargo run -- serve --host 0.0.0.0 --port 8080

# Or run the compiled binary
.\target\release\jp.exe serve --port 8080

# Alternatively, serve via Python/Node (legacy)
cd web
python -m http.server 8000
# or
npx http-server
# or just open index.html in a browser
```

## Architecture

### Rust CLI Application (`src/main.rs`)

The main application is a single-file Rust CLI tool with three main components:

1. **ASCII Art Rendering System**
   - `get_ascii_art()`: Returns hiragana ASCII art (15 lines × 50 chars each)
   - `get_katakana_ascii_art()`: Returns katakana ASCII art
   - Each kana character has pre-rendered ASCII art generated from real Japanese fonts (MS Gothic/MS Mincho)
   - ASCII art uses character density mapping: ` .',:;-=+*#%@` to represent different grayscale levels

2. **Dictionary System**
   - `init_romaji_map()`: Creates HashMap of romaji → JapaneseChar structs
   - `init_chinese_map()`: Creates HashMap of Chinese → Japanese translations
   - Each romaji entry includes: hiragana, katakana, romaji, and 3 example words
   - Chinese dictionary contains ~60 entries covering: greetings, numbers, family, colors, seasons, days of week

3. **LLM Translation**
   - `translate_with_llm()`: Async function that calls OpenAI-compatible APIs
   - Activated only when Chinese input is not found in local dictionary
   - Uses environment variables: `OPENAI_API_KEY`, `OPENAI_API_BASE`, `OPENAI_MODEL`
   - Formats responses with Japanese kanji + hiragana reading + romaji

### Data Flow

```
User Input (CLI args)
    ↓
Parse with clap
    ↓
Check if romaji in local map → Display ASCII art flashcard
    ↓ (not found)
Check if Chinese in local map → Display translation
    ↓ (not found, has Chinese chars)
Call LLM API → Display AI-generated translation
    ↓ (not found, no Chinese chars)
Show "not found" error message
```

### Python Scripts (`scripts/`)

1. **generate_ascii_art.py**
   - Uses PIL/Pillow to render Japanese characters at 1000×1000px, 800pt font size
   - Converts rendered images to ASCII art (50×15 characters)
   - Generates both hiragana and katakana versions
   - Outputs to `hiragana_ascii_art.txt` and `katakana_ascii_art.txt`

2. **update_main_rs.py**
   - Reads generated ASCII art text files
   - Uses regex to find and replace the match arms in `get_ascii_art()` and `get_katakana_ascii_art()`
   - Automatically updates `src/main.rs` with new ASCII art

### Web Application (`web/`)

- Standalone HTML/CSS/JS application (no build step required)
- `script.js` contains equivalent dictionary data as Rust HashMaps
- Implements real-time search suggestions
- No LLM integration yet (future feature)
- Can be opened directly in browser or served via local HTTP server

## Key Data Structures

### JapaneseChar

```rust
struct JapaneseChar {
    romaji: String,      // e.g., "chi"
    hiragana: String,    // e.g., "ち"
    katakana: String,    // e.g., "チ"
    examples: Vec<String>, // 3 example words
}
```

### OpenAI API Integration

```rust
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

struct ChatResponse {
    choices: Vec<ChatChoice>,
}
```

## Environment Variables

When working with LLM features, these environment variables are required:

- `OPENAI_API_KEY` (required): API key for OpenAI or compatible service
- `OPENAI_API_BASE` (optional): Base URL, defaults to `https://api.openai.com/v1`
- `OPENAI_MODEL` (optional): Model name, defaults to `gpt-3.5-turbo`

## Dependencies

Key Rust crates:
- `clap`: CLI argument parsing with derive macros
- `reqwest`: HTTP client for LLM API calls (with blocking and json features)
- `serde`, `serde_json`: JSON serialization for API communication
- `tokio`: Async runtime (full features)
- `unicode-width`: Proper width calculation for CJK characters in output formatting

Python dependencies for scripts:
- `Pillow` (PIL): Image generation and manipulation for ASCII art

## File Organization

- Main application code is in `src/main.rs` (~2200 lines)
- ASCII art data is embedded directly in source code (not loaded from external files)
- Dictionary data is hard-coded in initialization functions
- Web version is completely separate with no code sharing

## Development Workflow

1. When modifying ASCII art appearance:
   - Edit parameters in `scripts/generate_ascii_art.py` (width, height, character set)
   - Run `generate_ascii_art.py` to create new art files
   - Run `update_main_rs.py` to inject into source code
   - Rebuild with `cargo build --release`

2. When adding new dictionary entries:
   - Edit `init_romaji_map()` or `init_chinese_map()` directly in `src/main.rs`
   - Rebuild application
   - Consider updating web version's `script.js` as well

3. When modifying LLM behavior:
   - Edit the prompt in `translate_with_llm()` function
   - Test with various inputs that aren't in the local dictionary
   - Ensure proper error handling for API failures
