use clap::{command, Parser};

#[derive(Parser)]
#[command(version = "0.1.0", about = "Simple ls like command", long_about = None)]
pub struct Cli {
    files: Option<Vec<String>>,
}

impl Cli {
    pub fn get_paths(&self) -> Vec<String> {
        match &self.files {
            Some(paths) => paths.to_owned(),
            None => vec!["./".to_string()],
        }
    }
}
