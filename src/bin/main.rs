//! Module for the main binary entry point.  
//!
//! Note this (currently) assumes a hard-coded input file at input.txt.
use std::path::PathBuf;

/// Entry point
fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("You must provide a single filename parameter");
        return;
    }
    let path = PathBuf::from(&args[1]);
    let output = martian::run_file(&path);
    match output {
        Ok(output) => {
            for line in output {
                println!("{}", line);
            }
        }
        Err(err) => {
            println!("Failed with error: {}", err);
        }
    }
}
