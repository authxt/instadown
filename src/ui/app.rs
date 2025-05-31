use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use chrono::Local;
use crate::downloader::InstagramDownloader;

#[derive(Default, PartialEq)]
pub enum InputMode {
    Normal,
    #[default]
    Editing,
}

#[derive(Default, PartialEq)]
pub enum FocusedArea {
    #[default]
    Input,
    Tabs,
    ExitButton,
}

pub enum DownloadStatus {
    None,
    InProgress,
    Downloading {
        progress: f32,
        speed: String,
        eta: String,
        size: String,
    },
    Complete,
    Error(String),
}

impl Default for DownloadStatus {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Download {
    pub url: String,
    pub filename: String,
    #[serde(with = "timestamp_seconds")]
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub status: String,
}

pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub focused_area: FocusedArea,
    pub selected_tab: usize,
    pub selected_history_item: Option<usize>,
    pub downloads: Vec<Download>,
    pub download_status: DownloadStatus,
    pub downloader: InstagramDownloader,
}

mod timestamp_seconds {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(date.timestamp())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let timestamp = i64::deserialize(deserializer)?;
        Ok(Local.timestamp_opt(timestamp, 0).unwrap())
    }
}

impl App {
    pub fn new(downloader: InstagramDownloader) -> Self {
        // Create downloads directory if it doesn't exist
        let downloads_dir = PathBuf::from("downloads");
        if !downloads_dir.exists() {
            std::fs::create_dir_all(&downloads_dir).expect("Failed to create downloads directory");
        }

        Self {
            input: String::new(),
            input_mode: InputMode::default(),
            focused_area: FocusedArea::default(),
            selected_tab: 0,
            selected_history_item: None,
            downloads: Vec::new(),
            download_status: DownloadStatus::default(),
            downloader,
        }
    }

    pub fn enter_edit_mode(&mut self) {
        self.input_mode = InputMode::Editing;
        self.focused_area = FocusedArea::Input;
    }

    pub fn exit_edit_mode(&mut self) {
        self.input_mode = InputMode::Normal;
        self.focused_area = FocusedArea::Input;
    }

    pub fn toggle_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % 2;
        self.focused_area = FocusedArea::Tabs;
    }

    pub fn submit_url(&mut self) {
        if !self.input.is_empty() {
            self.download_status = DownloadStatus::InProgress;
            let url = self.input.clone();
            
            match self.downloader.download(&url, |status| {
                self.download_status = status;
            }) {
                Ok(filename) => {
                    self.add_download(url, filename);
                    self.input.clear();
                    self.input_mode = InputMode::Normal;
                    self.download_status = DownloadStatus::Complete;
                }
                Err(e) => {
                    self.download_status = DownloadStatus::Error(e.to_string());
                }
            }
        }
    }

    fn add_download(&mut self, url: String, filename: String) {
        let download = Download {
            url,
            filename,
            timestamp: Local::now(),
            status: "Completed".to_string(),
        };
        self.downloads.push(download);
    }

    pub fn handle_mouse_click(&mut self, x: u16, _y: u16, area: FocusedArea) {
        match area {
            FocusedArea::Tabs => {
                self.focused_area = FocusedArea::Tabs;
                // Assuming tabs are at the top and each tab is roughly half the width
                if x < (termsize::get().map(|s| s.cols).unwrap_or(80) / 2) {
                    self.selected_tab = 0;
                } else {
                    self.selected_tab = 1;
                }
            }
            FocusedArea::Input => {
                self.focused_area = FocusedArea::Input;
                self.enter_edit_mode();
            }
            FocusedArea::ExitButton => {
                self.focused_area = FocusedArea::ExitButton;
            }
        }
    }
} 