use std::path::PathBuf;

pub struct Config {
    pub output_dir: PathBuf,
}

impl Config {
    pub fn new(output_dir: PathBuf) -> Self {
        Self { output_dir }
    }
} 