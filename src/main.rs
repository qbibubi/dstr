#[macro_use]

mod args;

use args::DstrArgs;
use clap::Parser;

fn main() {
    let _args = DstrArgs::parse();
}
