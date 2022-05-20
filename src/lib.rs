//! Library module with various utility functions
pub mod direction;
pub mod map;
pub mod martian;
pub mod movement;
mod tests;

use crate::map::Map;
use crate::martian::Martian;

use std::fs;
use std::ops;
use std::path::PathBuf;

/// A simple 2-vector
#[derive(Debug, PartialEq, Clone)]
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

/// Operator overload for +
impl ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

/// Operator overload for +=
impl ops::Mul<i32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

/// Read the set of lines from a file.
/// (Currently) panics if the file does not exist.
pub fn get_file_content(filename: &PathBuf) -> Result<Vec<String>, String> {
    let string_form: String = fs::read_to_string(filename)
        .map_err(|_| "Error loading file - check it exists and is readable".to_string())?;
    Ok(string_form.lines().map(|s| s.to_string()).collect())
}

/// Run a full input file.
///
/// This involves opening the file, loading the bounds & martians, and then executing them in series.
pub fn run_file(filename: &PathBuf) -> Result<Vec<String>, String> {
    let file_content = get_file_content(filename)?;
    let mut file_content_iter = file_content.iter().filter(|x| !x.is_empty());
    let mut map = Map::from_string(
        file_content_iter
            .next()
            .ok_or_else(|| "File has no valid rows!".to_string())?,
    )?;
    let mut martians = vec![];

    while let Some(s1) = file_content_iter.next() {
        let s2 = file_content_iter
            .next()
            .ok_or_else(|| "Martian input line is unpaired!".to_string())?;
        martians.push(Martian::from_strings(vec![s1, s2])?);
    }

    let mut output = vec![];
    for mut martian in martians {
        match martian.attempt_movements(&mut map) {
            Ok(s) | Err(s) => {
                output.push(s);
            }
        }
    }

    Ok(output)
}
