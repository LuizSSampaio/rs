use console::style;
use core::fmt;
use std::{error::Error, fs, path::PathBuf};

pub struct Files {
    paths: Vec<String>,
}

impl Files {
    pub fn new(paths: Vec<String>) -> Result<Self, FilesError> {
        if !Self::check_paths(&paths) {
            return Err(FilesError::NotFoundOrNotDir);
        }
        Ok(Self { paths })
    }

    fn check_paths(paths: &Vec<String>) -> bool {
        for path in paths {
            if !PathBuf::from(path).is_dir() {
                return false;
            }
        }
        true
    }

    pub fn print_dirs(&self) {
        if self.paths.len() > 1 {
            for path in &self.paths {
                println!("{}:", path);
                Self::print_files(path);
                println!();
            }
        } else {
            Self::print_files(self.paths.first().unwrap());
        }
    }

    fn print_files(path: &String) {
        let entries = fs::read_dir(path).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let file_name = entry.file_name();
            let file_name_string = file_name.to_str().unwrap();

            if entry.metadata().unwrap().is_dir() {
                println!("{}", style(file_name_string).blue());
            } else {
                println!("{}", file_name_string);
            }
        }
    }
}

#[derive(Debug)]
pub enum FilesError {
    NotFoundOrNotDir,
}

impl fmt::Display for FilesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FilesError::NotFoundOrNotDir => write!(f, "Not a dir or not found"),
        }
    }
}

impl Error for FilesError {}
