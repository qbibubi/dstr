use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Project name
    #[clap(value_parser)]
    pub project_name: Option<String>,

    /// Set custom config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub project_config: Option<PathBuf>,

    /// Enable debug information
    #[clap(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Test {
        #[clap(short, long, action)]
        list: bool,
    },
}
