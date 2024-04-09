use std::process;

use clap::Parser;

use minigrep::{run, Config};

fn main() {
    let config = Config::parse();    

    if let Err(e) = run(&config) {
        match &config {
            Config { no_messages: true, .. } => (),
            _ => eprintln!("Application error: {e}"),
        }
        process::exit(1);
    }
}
