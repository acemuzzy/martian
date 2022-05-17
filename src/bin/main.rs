//! Module for the main binary entry point.  
//!
//! Note this (currently) assumes a hard-coded input file at input.txt.
use std::path::PathBuf;

const FILENAME: &str = r"input.txt";

/// Entry point
fn main() {
    let path = PathBuf::from(FILENAME);
    let martian = martian::martian::Martian::from_file(&path);
    println!("{:?}", martian);
}
