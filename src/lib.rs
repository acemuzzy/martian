//! Library module with various utility functions
pub mod martian;
mod tests;

use std::fs;
use std::path::PathBuf;

/// A simple 2-vector
#[derive(Debug, PartialEq)]
pub struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    /// Create a new 2-vector
    pub fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }
}

/// Read the set of lines from a file.
/// (Currently) panics if the file does not exist.
pub fn get_file_content(filename: &PathBuf) -> Vec<String> {
    let string_form: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    string_form.lines().map(|s| s.to_string()).collect()
}
