# CLAUDE.md - AI Assistant Guide

This file provides comprehensive guidance for AI assistants (like Claude) working with this repository.

## Project Overview

**JP (flashcard-rs)** is a Japanese learning application written in Rust with both CLI and desktop GUI versions. It helps users learn Japanese through:

1. **Romaji Lookup**: Display hiragana/katakana with large ASCII art flashcards (15 lines × 50 chars)
2. **Chinese Translation**: Local dictionary with ~60 common Chinese-to-Japanese translations
3. **AI Translation**: LLM-powered translation via OpenAI-compatible APIs for phrases not in the local dictionary
4. **Desktop App**: Tauri-based GUI with timed review notifications and learning progress tracking

### Project Structure

```
flashcard-rs/
├── src/
│   ├── main.rs              # CLI application entry point (~2200 lines)
│   └── desktop.rs           # Tauri desktop app entry point
├── desktop-ui/              # Frontend for desktop app (HTML/CSS/JS)
│   ├── index.html           # Main UI structure
│   ├── styles.css           # Styling with card flip animations
│   └── script.js            # Frontend logic and dictionary data
├── scripts/                 # Development tools
│   ├── generate_ascii_art.py    # Generate kana ASCII art from fonts
│   ├── update_main_rs.py        # Auto-update main.rs with ASCII art
│   ├── generate_icon.py         # Icon generation utility
│   ├── hiragana_ascii_art.txt   # Generated hiragana art
│   └── katakana_ascii_art.txt   # Generated katakana art
├── icons/                   # Application icons (various sizes)
├── Cargo.toml               # Project configuration with feature flags
├── tauri.conf.json          # Tauri desktop app configuration
├── build.rs                 # Conditional Tauri build script
└── README.md                # User-facing documentation (Chinese)
```

## Architecture

### Two Binaries

The project compiles to two separate binaries controlled by cargo features:

1. **jp** (CLI): `src/main.rs`
   - Command-line flashcard lookup tool
   - Web server mode for serving desktop-ui
   - No Tauri dependencies

2. **jp-desktop** (GUI): `src/desktop.rs`
   - Requires `--features desktop`
   - Tauri-based desktop application
   - System tray integration
   - Auto-start capability
   - Window state persistence
   - Notification system

### Feature Flags

```toml
[features]
default = []
desktop = [
    "tauri",
    "tauri-build",
    "tauri-plugin-autostart",
    "tauri-plugin-store",
    "tauri-plugin-window-state",
    "tauri-plugin-notification",
    "tauri-plugin-dialog",
    "rand"
]
```

### CLI Application (`src/main.rs`)

**Key Components:**

1. **Command Structure (clap)**
   ```rust
   struct Cli {
       command: Option<Commands>,
       text: Option<String>,  // Default positional argument
   }

   enum Commands {
       Serve { port, host },  // Web server mode
       Lookup { text },       // Explicit lookup (default)
   }
   ```

2. **ASCII Art System**
   - `get_ascii_art(character)`: Returns hiragana ASCII art
   - `get_katakana_ascii_art(character)`: Returns katakana ASCII art
   - Pre-rendered from MS Gothic/MS Mincho fonts at 1000×1000px, 800pt
   - Character mapping: `` ` .',:;-=+*#%@█`` for grayscale levels

3. **Dictionary System**
   - `init_romaji_map()`: HashMap<String, JapaneseChar> with 46 entries
   - `init_chinese_map()`: HashMap with ~60 entries (greetings, numbers, family, colors, seasons, weekdays)

   ```rust
   struct JapaneseChar {
       romaji: String,
       hiragana: String,
       katakana: String,
       examples: Vec<String>,  // Always 3 examples
   }
   ```

4. **LLM Integration**
   - `translate_with_llm(text)`: Async OpenAI-compatible API calls
   - Only triggered when Chinese input not found in local dictionary
   - Environment variables:
     - `OPENAI_API_KEY` (required)
     - `OPENAI_API_BASE` (optional, default: `https://api.openai.com/v1`)
     - `OPENAI_MODEL` (optional, default: `gpt-3.5-turbo`)

5. **Web Server**
   - Uses Axum + Tower-HTTP
   - Serves `desktop-ui/` directory
   - Default: `http://127.0.0.1:8080`

### Desktop Application (`src/desktop.rs`)

**Key Features:**

1. **Settings Management**
   ```rust
   struct Settings {
       interval: u64,              // Minutes between review notifications
       autostart: bool,            // System startup behavior
       card_type: String,          // "romaji", "chinese", or "mixed"
       close_behavior: Option<String>, // "minimize", "exit", or None (ask)
   }
   ```

2. **State Management**
   ```rust
   struct AppState {
       settings: Arc<Mutex<Settings>>,
       timer_running: Arc<Mutex<bool>>,
       timer_generation: Arc<Mutex<u64>>,
       window_hidden: Arc<Mutex<bool>>,
   }
   ```

3. **Tauri Commands** (exposed to frontend)
   - `get_settings()`: Retrieve current settings
   - `save_settings(settings)`: Persist settings to disk
   - `get_random_card(card_type)`: Get flashcard based on type
   - `get_random_romaji_card()`: Get romaji flashcard
   - `get_random_chinese_card()`: Get Chinese word flashcard
   - `choose_close_behavior(behavior)`: Handle window close action
   - `show_window()`: Show hidden window from tray
   - `get_stats()`: Get learning statistics
   - `record_result(correct)`: Record answer result

4. **Window Management**
   - Auto-saves window size/position with `tauri-plugin-window-state`
   - System tray with "Show"/"Quit" menu
   - Configurable close behavior (minimize to tray vs exit)
   - Confirmation dialog on quit

5. **Timer System**
   - Spawns async task on app startup
   - Checks `timer_running` flag and `interval` setting
   - Shows notification and window at configured intervals
   - Generation counter prevents orphaned tasks

### Frontend (`desktop-ui/`)

**Technology Stack:**
- Vanilla JavaScript (no framework)
- CSS with 3D flip card animations
- Tauri API via `window.__TAURI__.core.invoke`

**Key Features:**
1. **Flashcard Display**
   - Front: Question (romaji or Chinese)
   - Back: Answer (hiragana/katakana or Japanese translation)
   - Click to flip with 3D animation

2. **Learning Feedback**
   - "Answer Incorrect" button (keyboard: 1)
   - "Answer Correct" button (keyboard: 2)
   - Records results for statistics

3. **Settings Modal**
   - Interval adjustment (5-60 minutes)
   - Autostart toggle
   - Card type selection
   - Close behavior preference

4. **Dictionary Data**
   - Duplicated from `src/main.rs` (keep in sync!)
   - `romajiDict`: Same 46 entries as Rust
   - `chineseDict`: Same ~60 entries as Rust

## Development Workflows

### Building

```bash
# CLI only
cargo build --release

# Desktop app (requires Tauri)
cargo build --release --bin jp-desktop --features desktop

# Both
cargo build --release --bins --features desktop
```

### Running

```bash
# CLI - lookup mode
cargo run -- chi              # Romaji lookup
cargo run -- 你好             # Chinese translation

# CLI - web server mode
cargo run -- serve --port 8080

# Desktop app
cargo run --bin jp-desktop --features desktop
```

### Installing

```bash
# Install to ~/.cargo/bin/
cargo install --path . --bins --features desktop

# CLI only
cargo install --path . --bin jp

# Desktop only
cargo install --path . --bin jp-desktop --features desktop
```

### Tauri Bundling

```bash
# Install Tauri CLI first
cargo install tauri-cli --version "^2.0.0"

# Build installer (Windows NSIS)
cargo tauri build --features desktop

# Output: src-tauri/target/release/bundle/nsis/JP Desktop_0.1.0_x64-setup.exe
```

### ASCII Art Regeneration

When modifying ASCII art appearance:

```bash
cd scripts

# 1. Generate new ASCII art from fonts
python generate_ascii_art.py
# Creates/updates: hiragana_ascii_art.txt, katakana_ascii_art.txt

# 2. Auto-inject into src/main.rs
python update_main_rs.py
# Uses regex to replace match arms in get_ascii_art() functions

cd ..

# 3. Rebuild
cargo build --release
```

**Important:**
- ASCII art is embedded in source code, not loaded at runtime
- Characters are 15 lines tall × 50 chars wide
- Uses density mapping: `` ` .',:;-=+*#%@█``

### Adding Dictionary Entries

**For Romaji (must update BOTH):**

1. Edit `src/main.rs::init_romaji_map()`:
   ```rust
   map.insert("NEW", JapaneseChar {
       romaji: "NEW".to_string(),
       hiragana: "ひらがな".to_string(),
       katakana: "カタカナ".to_string(),
       examples: vec![
           "example1".to_string(),
           "example2".to_string(),
           "example3".to_string(),
       ],
   });
   ```

2. Edit `desktop-ui/script.js::romajiDict`:
   ```javascript
   'NEW': {
       romaji: 'NEW',
       hiragana: 'ひらがな',
       katakana: 'カタカナ',
       examples: ['example1', 'example2', 'example3']
   }
   ```

3. Generate ASCII art for new characters (if adding new kana)

**For Chinese Words (must update BOTH):**

1. Edit `src/main.rs::init_chinese_map()`:
   ```rust
   map.insert("新词", ChineseWord {
       chinese: "新词".to_string(),
       japanese: "新しい言葉".to_string(),
       reading: "あたらしいことば/atarashii kotoba".to_string(),
   });
   ```

2. Edit `desktop-ui/script.js::chineseDict`:
   ```javascript
   '新词': {
       chinese: '新词',
       japanese: '新しい言葉',
       reading: 'あたらしいことば/atarashii kotoba'
   }
   ```

### Modifying LLM Behavior

Edit `src/main.rs::translate_with_llm()`:
- Modify the system/user prompt
- Adjust temperature (currently 0.3)
- Change response parsing logic

**Note:** Desktop app doesn't use LLM (no network requests from GUI).

### Testing

```bash
# Run tests (minimal test coverage currently)
cargo test

# Clippy linting
cargo clippy --all-targets --all-features

# Format check
cargo fmt --check
```

## Code Conventions

### Rust Style

1. **Formatting**: Use `cargo fmt` (rustfmt defaults)
2. **Linting**: Follow `cargo clippy` suggestions
3. **Naming**:
   - Snake_case for functions/variables
   - PascalCase for types/structs
   - SCREAMING_SNAKE_CASE for constants

4. **Error Handling**:
   - Tauri commands return `Result<T, String>`
   - Use `.map_err(|e| e.to_string())` for error conversion
   - Log errors before returning: `eprintln!("Error: {}", e)`

5. **Async**:
   - CLI uses Tokio runtime
   - Desktop uses Tauri's async runtime
   - Timer spawns `tokio::spawn` task

### Frontend Style

1. **No Framework**: Vanilla JavaScript for simplicity
2. **Tauri API**: Always check `window.__TAURI__` availability
3. **Error Handling**: Display user-friendly alerts
4. **Keyboard Shortcuts**: Document in UI with `<span class="shortcut">`

### Git Workflow

**Commit Message Convention:**
```
<type>(<scope>): <description>

Examples:
feat(desktop): add close behavior setting in UI
fix(desktop): suppress autostart warning when not enabled
style(desktop-ui): update styles.css for card height
```

**Types:** feat, fix, docs, style, refactor, test, chore

**Scopes:** desktop, cli, desktop-ui, scripts

**Branch Naming:**
- Feature: `claude/<description>-<session-id>`
- Main development: `main`

## Common Tasks for AI Assistants

### 1. Adding New Flashcard Content

**Checklist:**
- [ ] Add to `src/main.rs` dictionary functions
- [ ] Add to `desktop-ui/script.js` dictionaries
- [ ] Generate ASCII art if new kana characters
- [ ] Test in both CLI and desktop app
- [ ] Update README.md examples if significant

### 2. Modifying UI/UX

**Locations:**
- Desktop window config: `tauri.conf.json`
- Layout: `desktop-ui/index.html`
- Styles: `desktop-ui/styles.css`
- Behavior: `desktop-ui/script.js`
- Backend commands: `src/desktop.rs`

**After changes:**
- Test in development: `cargo run --bin jp-desktop --features desktop`
- Check window state persistence (close/reopen)
- Test tray minimize/restore flow

### 3. Fixing Bugs

**Desktop App Issues:**
1. Check Tauri console: DevTools (F12 in development)
2. Check Rust logs: `stderr` output from `eprintln!`
3. Verify settings persistence: `settings.json` in app data dir
4. Test autostart: `tauri-plugin-autostart` warnings are non-fatal

**CLI Issues:**
1. Test both modes: direct text and explicit `lookup` subcommand
2. Verify environment variables for LLM features
3. Check Unicode width calculations (affects formatting)

### 4. Updating Dependencies

**Be careful with:**
- Tauri: Major version changes require config updates
- Tauri plugins: Must match Tauri version (v2.x)
- Tokio: Features must include "full" for all async needs

**After updates:**
```bash
cargo update
cargo build --release --all-features
cargo clippy --all-targets --all-features
```

### 5. Performance Considerations

**CLI:**
- Dictionary lookups are O(1) HashMap operations
- ASCII art is static data (no rendering at runtime)
- LLM calls are async but block on response

**Desktop:**
- Timer runs in separate task (doesn't block UI)
- Settings saved to disk on every change (could batch if needed)
- Notifications are non-blocking

### 6. Platform-Specific Behavior

**Windows:**
- Autostart uses Registry
- Window subsystem directive: `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`
- NSIS installer bundles all dependencies

**macOS:**
- Autostart uses LaunchAgent
- `MacosLauncher` in autostart plugin

**Linux:**
- Autostart uses .desktop file
- May need additional system tray dependencies

## Environment Setup

### Development Requirements

**Rust:**
```bash
# Install Rust (if not present)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Required for desktop builds
rustup target add x86_64-pc-windows-msvc  # Windows
rustup target add x86_64-apple-darwin     # macOS
rustup target add x86_64-unknown-linux-gnu # Linux
```

**Tauri Prerequisites:**
- Node.js (for Tauri CLI, though this project has no JS build step)
- WebView2 (Windows) / WebKit (Linux) / System WebView (macOS)
- See: https://tauri.app/v2/guides/prerequisites/

**Python (for scripts):**
```bash
pip install Pillow  # For ASCII art generation
```

### Runtime Requirements

**CLI LLM Features:**
```bash
export OPENAI_API_KEY="sk-..."
export OPENAI_API_BASE="https://api.openai.com/v1"  # Optional
export OPENAI_MODEL="gpt-3.5-turbo"                  # Optional
```

**Desktop App:**
- No special environment variables needed
- Settings stored in platform-specific app data directory

## Data Synchronization Points

**CRITICAL:** These data structures must be kept in sync:

1. **Romaji Dictionary:**
   - `src/main.rs::init_romaji_map()`
   - `desktop-ui/script.js::romajiDict`

2. **Chinese Dictionary:**
   - `src/main.rs::init_chinese_map()`
   - `desktop-ui/script.js::chineseDict`

3. **ASCII Art:**
   - `src/main.rs::get_ascii_art()`
   - `src/main.rs::get_katakana_ascii_art()`
   - `scripts/hiragana_ascii_art.txt`
   - `scripts/katakana_ascii_art.txt`

**When updating any of these:**
1. Update Rust code
2. Update JavaScript code (if affects desktop-ui)
3. Regenerate ASCII art (if new characters)
4. Test both CLI and desktop app
5. Update README.md if user-facing changes

## Troubleshooting

### "Failed to enable autostart"

**Cause:** Platform-specific autostart setup can fail (permissions, missing directories)

**Solution:** This is non-fatal. The warning is logged but settings still save. Users can manually enable autostart if needed.

**Code:** `src/desktop.rs:99-115` - errors are logged but don't propagate

### ASCII Art Looks Wrong

**Possible causes:**
1. Terminal/console doesn't support required characters
2. Font doesn't render CJK properly
3. Line wrapping in narrow terminals

**Solutions:**
1. Use monospace terminal with Unicode support
2. Ensure terminal width ≥ 50 chars
3. Check font supports Japanese characters

### Desktop App Won't Start

**Checklist:**
1. Built with `--features desktop`?
2. Tauri dependencies installed? (`tauri`, all plugins)
3. System WebView available?
4. Check `tauri.conf.json` paths (especially `frontendDist`)

### Settings Not Persisting

**Debug:**
1. Check store file location (printed on first save error)
2. Verify write permissions
3. Check for JSON serialization errors in logs

### Timer Not Working

**Debug:**
1. Check `timer_running` state in `AppState`
2. Verify `interval` setting is > 0
3. Look for panics in spawned task (check stderr)
4. Ensure timer generation increments correctly

## Related Documentation

- **README.md**: User-facing documentation (Chinese)
- **WARP.md**: Guide for WARP terminal AI (legacy, less comprehensive)
- **Cargo.toml**: Dependencies and feature configuration
- **tauri.conf.json**: Desktop app configuration

## Quick Reference

### File Sizes (Approximate)
- `src/main.rs`: ~2200 lines (includes embedded ASCII art)
- `src/desktop.rs`: ~400 lines
- `desktop-ui/script.js`: ~500 lines

### Key Line References
- Romaji dictionary: `src/main.rs:~800-1000`
- Chinese dictionary: `src/main.rs:~1000-1100`
- ASCII art matchers: `src/main.rs:~50-800`
- Tauri commands: `src/desktop.rs:66-200`
- Timer logic: `src/desktop.rs:250-300`

### Important Constants
- ASCII art size: 15 lines × 50 chars
- Default interval: 10 minutes
- Default window: 800×900 pixels
- Min window: 700×800 pixels
- Default card type: "mixed"
- LLM temperature: 0.3

---

**Last Updated:** 2025-01-14 (based on commit da17091)

**Maintainer Notes:**
- Keep dictionaries in sync between Rust and JavaScript
- Test both binaries after significant changes
- Update WARP.md if major architecture changes
- Consider adding integration tests for dictionary sync
