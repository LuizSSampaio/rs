mod cli;

use clap::Parser;

use crate::cli::*;

fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli.get_paths());
}
