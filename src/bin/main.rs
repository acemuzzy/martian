//! Module for the main binary entry point.  
use std::path::PathBuf;

/// Entry point
fn main() {
    // Parse arguments - we expect a single value for the file to run
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("You must provide a single filename parameter");
        return;
    }

    // Attempt to execute the file
    let path = PathBuf::from(&args[1]);
    let output = martian::run_file(&path);

    // Log output (or error message)
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
