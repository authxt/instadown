# Instadown

A command-line Instagram video downloader written in Rust.

## Features

- Download videos from Instagram posts and reels
- Progress bar with download status
- Automatic downloads folder creation
- Organized downloads with timestamps
- Error handling and user-friendly messages

## Installation

Make sure you have Rust installed on your system. Then:

```bash
# Clone the repository
git clone https://github.com/yourusername/instadown.git
cd instadown

# Build the project
cargo build --release

# The binary will be available in target/release/instadown
```

## Usage

```bash
# Basic usage (downloads to 'downloads' directory)
instadown --url "https://www.instagram.com/p/POST_ID/"

# Specify custom output directory
instadown --url "https://www.instagram.com/p/POST_ID/" --output ~/my-videos/

# Show help
instadown --help
```

## File Organization

- By default, all videos are saved in the `downloads` directory
- Each video is saved with a timestamp in the format: `instagram_video_YYYYMMDD_HHMMSS.mp4`
- You can specify a custom download location using the `--output` option

## Notes

- This tool only works with public Instagram posts
- Make sure you have the necessary permissions to download the content
- The tool requires an active internet connection
- Videos are automatically organized by timestamp to prevent overwrites

## License

MIT License 