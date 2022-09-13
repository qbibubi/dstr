#[macro_use]

mod args;

use args::Cli;
use args::Commands;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    if let Some(project_name) = cli.project_name.as_deref() {
        println!("Project name: {}", project_name);
    }

    if let Some(config_path) = cli.project_config.as_deref() {
        println!("Config: {}", config_path.display());
    }

    match cli.debug {
        0 => println!("Debug OFF"),
        1 => println!("Debug ON"),
        _ => println!("It is ON. You can stop trying"),
    }

    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }
}
