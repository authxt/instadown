use anyhow::{Result, Context};
use std::process::{Command, Stdio};
use std::fs;

use crate::ui::app::DownloadStatus;
use crate::core::Config;
use super::utils::DownloadUtils;

pub struct InstagramDownloader {
    config: Config,
}

impl InstagramDownloader {
    pub fn new(config: Config) -> Result<Self> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&config.output_dir)
            .context("Failed to create output directory")?;
        
        // Check if yt-dlp is available
        DownloadUtils::check_yt_dlp()?;

        Ok(Self { config })
    }

    pub fn download(&self, url: &str, status_callback: impl FnMut(DownloadStatus)) -> Result<String> {
        let output_template = self.config.output_dir
            .join("%(title)s_%(upload_date)s_%(id)s.%(ext)s")
            .to_string_lossy()
            .to_string();

        let child = Command::new("yt-dlp")
            .arg(url)
            .arg("-o")
            .arg(&output_template)
            .arg("--newline")  // Force progress on new lines
            .arg("--no-check-certificates")  // Skip SSL verification
            .arg("--user-agent")
            .arg("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .arg("--progress-template")
            .arg("download:[%(progress.downloaded_bytes)s/%(progress.total_bytes)s][%(progress.speed)s][ETA:%(progress.eta)s]")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to start yt-dlp")?;

        DownloadUtils::handle_download_process(child, status_callback, output_template)
    }
} 