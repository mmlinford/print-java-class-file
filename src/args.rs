use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    pub input_file_path: PathBuf,
}

pub fn parse_args() -> Args {
    Args::parse()
}
