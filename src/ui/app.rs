use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum FocusedArea {
    Tabs,
    Input,
    History,
    ExitButton,
    None,
}

#[derive(Debug)]
pub struct App {
    pub input: String,
    pub input_mode: InputMode,
    pub download_status: DownloadStatus,
    pub selected_tab: usize,
    pub downloads: Vec<Download>,
    pub base_path: PathBuf,
    pub focused_area: FocusedArea,
    pub selected_history_item: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Download {
    pub url: String,
    pub filename: String,
    #[serde(with = "timestamp_seconds")]
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub status: String,
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

impl Default for App {
    fn default() -> Self {
        let base_path = PathBuf::from("downloads");
        
        // Create downloads directory if it doesn't exist
        if !base_path.exists() {
            std::fs::create_dir_all(&base_path).expect("Failed to create downloads directory");
        }

        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            download_status: DownloadStatus::None,
            selected_tab: 0,
            downloads: Vec::new(),
            base_path,
            focused_area: FocusedArea::None,
            selected_history_item: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enter_edit_mode(&mut self) {
        self.input_mode = InputMode::Editing;
        self.focused_area = FocusedArea::Input;
    }

    pub fn exit_edit_mode(&mut self) {
        self.input_mode = InputMode::Normal;
        self.focused_area = FocusedArea::None;
    }

    pub fn toggle_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % 2;
        self.focused_area = FocusedArea::Tabs;
    }

    pub fn add_download(&mut self, url: String, filename: String) {
        let download = Download {
            url,
            filename,
            timestamp: chrono::Local::now(),
            status: "Completed".to_string(),
        };
        self.downloads.push(download);
    }

    pub fn handle_mouse_click(&mut self, x: u16, y: u16, area: FocusedArea) {
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
            FocusedArea::History => {
                if self.selected_tab == 1 {  // Only if we're in the history tab
                    self.focused_area = FocusedArea::History;
                    // Calculate which history item was clicked based on y position
                    // Assuming history items start at y=5 (after tabs and input)
                    if y >= 5 {
                        let index = (y - 5) as usize;
                        if index < self.downloads.len() {
                            self.selected_history_item = Some(index);
                        }
                    }
                }
            }
            FocusedArea::ExitButton => {
                self.focused_area = FocusedArea::ExitButton;
            }
            FocusedArea::None => {
                self.focused_area = FocusedArea::None;
                self.selected_history_item = None;
            }
        }
    }
} 