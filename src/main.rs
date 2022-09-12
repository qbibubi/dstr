#![allow(unused)]

use clap::Parser;
use std::fs;

struct Args {
    // pattern to look for
    pattern: String,

    // path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");
    let args = Args::parse();
}
