use anyhow::{Result, Context};
use std::process::{Command, Child};
use std::io::{BufRead, BufReader};
use regex::Regex;

use crate::ui::app::DownloadStatus;

pub struct DownloadUtils;

impl DownloadUtils {
    pub fn check_yt_dlp() -> Result<()> {
        let yt_dlp_check = Command::new("yt-dlp")
            .arg("--version")
            .output();
            
        if yt_dlp_check.is_err() {
            return Err(anyhow::anyhow!("yt-dlp is not installed or not in PATH. Please install yt-dlp first."));
        }
        Ok(())
    }

    pub fn handle_download_process(
        mut child: Child,
        mut status_callback: impl FnMut(DownloadStatus),
        output_template: String,
    ) -> Result<String> {
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();
        
        // Create regex patterns for progress parsing
        let progress_re = Regex::new(r"(\d+\.\d+)%").unwrap();
        let speed_re = Regex::new(r"\[([^\]]+/s)\]").unwrap();
        let eta_re = Regex::new(r"ETA:([^\]]+)\]").unwrap();
        let size_re = Regex::new(r"\[(\d+\.\d+[KMG]iB/\d+\.\d+[KMG]iB)\]").unwrap();

        let mut success = true;

        // Read stdout for progress
        let stdout_reader = BufReader::new(stdout);
        for line in stdout_reader.lines() {
            if let Ok(line) = line {
                // Parse progress information
                if line.starts_with("download:") {
                    let progress = progress_re
                        .captures(&line)
                        .and_then(|cap| cap[1].parse::<f32>().ok())
                        .unwrap_or(0.0);

                    let speed = speed_re
                        .captures(&line)
                        .map(|cap| cap[1].to_string())
                        .unwrap_or_else(|| "0B/s".to_string());

                    let eta = eta_re
                        .captures(&line)
                        .map(|cap| cap[1].to_string())
                        .unwrap_or_else(|| "00:00".to_string());

                    let size = size_re
                        .captures(&line)
                        .map(|cap| cap[1].to_string())
                        .unwrap_or_else(|| "0B/0B".to_string());

                    status_callback(DownloadStatus::Downloading {
                        progress: progress / 100.0,
                        speed,
                        eta,
                        size,
                    });
                }
            }
        }

        // Read stderr for errors
        let stderr_reader = BufReader::new(stderr);
        let mut error_message = String::new();
        for line in stderr_reader.lines() {
            if let Ok(line) = line {
                if !line.trim().is_empty() {
                    error_message.push_str(&line);
                    error_message.push('\n');
                    success = false;
                }
            }
        }

        // Wait for the process to complete
        let status = child.wait().context("Failed to wait for yt-dlp")?;

        if success && status.success() {
            Ok(output_template)
        } else {
            let error_message = error_message.trim().to_string();
            Err(anyhow::anyhow!("{}", error_message))
        }
    }
} 