use anyhow::{Result, Context};
use std::path::PathBuf;
use std::process::Command;
use std::fs;

pub struct Downloader {
    output_dir: PathBuf,
}

impl Downloader {
    pub fn new(output_dir: PathBuf) -> Result<Self> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(&output_dir)
            .context("Failed to create output directory")?;
        
        // Check if yt-dlp is available
        let yt_dlp_check = Command::new("yt-dlp")
            .arg("--version")
            .output();
            
        if yt_dlp_check.is_err() {
            return Err(anyhow::anyhow!("yt-dlp is not installed or not in PATH. Please install yt-dlp first."));
        }

        Ok(Self { output_dir })
    }

    pub fn download(&self, url: &str) -> Result<String> {
        let output_template = self.output_dir
            .join("%(title)s_%(upload_date)s_%(id)s.%(ext)s")
            .to_string_lossy()
            .to_string();

        let output = Command::new("yt-dlp")
            .arg(url)
            .arg("-o")
            .arg(&output_template)
            .arg("--no-warnings")
            .arg("--progress")
            .output()
            .context("Failed to execute yt-dlp")?;

        if output.status.success() {
            Ok(output_template)
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            Err(anyhow::anyhow!("Failed to download video: {}", error))
        }
    }
} 