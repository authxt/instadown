# Instadown 📥

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![yt-dlp](https://img.shields.io/badge/yt--dlp-latest-red.svg)](https://github.com/yt-dlp/yt-dlp)
[![Ratatui](https://img.shields.io/badge/TUI-Ratatui-purple.svg)](https://github.com/ratatui-org/ratatui)
[![Crossterm](https://img.shields.io/badge/Terminal-Crossterm-blue.svg)](https://github.com/crossterm-rs/crossterm)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](https://makeapullrequest.com)

A beautiful Terminal User Interface (TUI) application for downloading Instagram videos, built with Rust.


## Features ✨

- 🎨 Beautiful TUI with tabs and interactive elements
- 📥 Download Instagram videos with progress tracking
- 📋 Download history with timestamps
- 🖱️ Mouse support for easy navigation
- ⌨️ Keyboard shortcuts for power users
- 🎯 Real-time download progress with speed, ETA, and file size
- 💾 Automatic download organization

## Requirements 📋

- Rust (latest stable)
- yt-dlp (for video downloading)
- A terminal that supports TUI applications

### Installing yt-dlp

#### On Nix:
```bash
nix-env -iA nixpkgs.yt-dlp
```

#### On other systems:
- **Linux**: `sudo apt install yt-dlp` or `sudo pacman -S yt-dlp`
- **macOS**: `brew install yt-dlp`
- **Windows**: `choco install yt-dlp`

Or follow the [official yt-dlp installation guide](https://github.com/yt-dlp/yt-dlp#installation).

## Installation 🚀

1. Clone the repository:
```bash
git clone https://github.com/authxt/instadown.git
cd instadown
```

2. Build and run with Cargo:
```bash
cargo build --release
cargo run --release
```

## Usage 💡

1. Launch the application:
```bash
cargo run --release
```

2. Navigate the interface:
   - Press `i` to enter URL input mode
   - Paste an Instagram video URL
   - Press `Enter` to start downloading
   - Use `Tab` to switch between Download and History tabs
   - Click the Exit button or press `q` to quit

### Keyboard Shortcuts ⌨️

- `i` - Enter URL input mode
- `Esc` - Exit input mode
- `Tab` - Switch tabs
- `Enter` - Submit URL/Confirm action
- `q` or `Q` - Quit application

### Mouse Controls 🖱️

- Click tabs to switch between views
- Click the input field to enter text
- Click the exit button to quit

## Download Location 📂

Downloaded videos are saved in the `downloads` directory with the following format:
```
downloads/
  └── title_uploaddate_id.ext
```

## Development 🛠️

This project uses:
- [ratatui](https://github.com/ratatui-org/ratatui) for the terminal interface
- [crossterm](https://github.com/crossterm-rs/crossterm) for terminal manipulation
- [yt-dlp](https://github.com/yt-dlp/yt-dlp) for video downloading

### Building from Source

```bash
# Clone the repository
git clone https://github.com/authxt/instadown.git
cd instadown

# Development build
cargo build

# Run tests
cargo test

# Run with debug output
RUST_LOG=debug cargo run
```

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request. 