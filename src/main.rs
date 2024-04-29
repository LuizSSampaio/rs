mod cli;
mod files;

use clap::Parser;

use crate::cli::*;
use crate::files::*;

fn main() {
    let cli = Cli::parse();
    let files = match Files::new(cli.get_paths()) {
        Ok(files) => files,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };
    files.print_dirs();
}
