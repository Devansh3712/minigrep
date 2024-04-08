use std::process;

use clap::Parser;

use minigrep::{run, Config};

fn main() {
    let config = Config::parse();    

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
