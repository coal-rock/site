use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[arg(short = 'p', long = "path", default_value = ".")]
    pub project_path: PathBuf,

    #[arg(short = 'c', long = "config", default_value = "config.toml")]
    pub config_path: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Serve {},
}
