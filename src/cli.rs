use clap::{command, Parser};

#[derive(Parser)]
#[command(version = "0.1.0", about = "Simple ls like command", long_about = None)]
pub struct Cli {
    files: Option<Vec<String>>,
}

impl Cli {
    // TODO: Remove unwrap
    pub fn get_paths(&self) -> Vec<&str> {
        match &self.files {
            Some(files) => {
                let mut paths: Vec<&str> = vec![];
                for path in files {
                    paths.push(path);
                }
                paths
            }
            None => vec!["./"],
        }
    }
}
