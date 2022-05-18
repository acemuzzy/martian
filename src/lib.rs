//! Library module with various utility functions
pub mod direction;
pub mod martian;
pub mod movement;
mod tests;

use std::fs;
use std::ops;
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

impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

/// Read the set of lines from a file.
/// (Currently) panics if the file does not exist.
pub fn get_file_content(filename: &PathBuf) -> Vec<String> {
    let string_form: String =
        fs::read_to_string(filename).expect("Something went wrong reading the file");
    string_form.lines().map(|s| s.to_string()).collect()
}
