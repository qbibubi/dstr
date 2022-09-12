use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct DstrArgs {
    /// Project name
    pub project_arg: String,
    /// Project language
    pub project_lang: String,
}
