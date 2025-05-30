# Instadown 📥

[![License](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/authxt/instadown)
[![Version](https://img.shields.io/badge/version-1.0.0-blue)](https://github.com/authxt/instadown/releases)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)

A robust command-line Instagram video downloader written in Rust that provides organized video management with automatic file naming and comprehensive download tracking capabilities.


## ✨ Features

- 🎥 Download videos from Instagram posts and reels
- 📊 Real-time progress bar with download status
- 📁 Automatic downloads folder organization
- 🕒 Smart file naming with timestamps
- ⚡ Fast and efficient downloads
- 🛡️ Built-in error handling
- 🔄 Automatic retry on failed downloads
- 📱 Support for mobile and desktop URLs

## 🚀 Quick Start

Make sure you have Rust installed on your system:

```bash
# Clone the repository
git clone https://github.com/authxt/instadown.git
cd instadown

# Build the project
cargo build --release

# Run the binary
./target/release/instadown --url "https://www.instagram.com/p/POST_ID/"
```

## 📖 Usage

```bash
# Basic usage
instadown --url "https://www.instagram.com/p/POST_ID/"

# Specify custom output directory
instadown --url "https://www.instagram.com/p/POST_ID/" --output ~/my-videos/

# Show help
instadown --help
```

## 📂 File Organization

Downloads are automatically organized for easy management:

```
downloads/
├── instagram_video_20240301_123456.mp4
├── instagram_video_20240301_123789.mp4
└── ...
```

- 📁 Default storage in `downloads` directory
- 🏷️ Timestamp-based naming: `instagram_video_YYYYMMDD_HHMMSS.mp4`
- 🎯 Custom output location support via `--output`
- 🔄 Automatic duplicate prevention

## ⚙️ Configuration

The tool can be configured through command-line arguments:

| Option | Description |
|--------|-------------|
| `--url` | Instagram post/reel URL (required) |
| `--output` | Custom output directory (optional) |
| `--format` | Video format (default: mp4) |
| `--quiet` | Disable progress bar |

## 📝 Notes

- ✅ Works with public Instagram posts
- ⚠️ Requires appropriate permissions for content
- 🌐 Needs active internet connection
- 🔒 Respects Instagram's terms of service


## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. 