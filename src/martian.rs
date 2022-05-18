//! Main martian module, with code to handle its instantiation and movement

use crate::direction::Direction;
use crate::movement::Movement;
use crate::{get_file_content, Vec2};

use regex::Regex;
use std::fmt;
use std::path::PathBuf;

/// The martian structure itself
#[derive(Debug, PartialEq)]
pub struct Martian {
    bounds: Vec2,
    location: Vec2,
    direction: Direction,
    instructions: Vec<Movement>,
}

/// String form of the martian (as used e.g. in successful output)
impl fmt::Display for Martian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.location.x, self.location.y, self.direction
        )
    }
}

impl Martian {
    /// Create a new martian instance, with explicit parameters
    pub fn new(
        bounds: Vec2,
        location: Vec2,
        direction: Direction,
        instructions: Vec<Movement>,
    ) -> Self {
        Martian {
            bounds,
            location,
            direction,
            instructions,
        }
    }

    /// Create a new martian instances, from a set of strings
    /// (Currently) does no error checking, so mainline only
    pub fn from_strings(input: Vec<String>) -> Self {
        // Parse out the bounds (two co-ordinates)
        let bounds_re = Regex::new(r"^(\d*) (\d*)$").unwrap();
        let bounds_cap = bounds_re.captures_iter(&input[0]).next().unwrap();
        let bounds = Vec2::new(
            bounds_cap[1].parse().unwrap(),
            bounds_cap[2].parse().unwrap(),
        );

        // Parse out the starting position / direction (two co-ordinates, plus a character)
        let start_re = Regex::new(r"^(\d*) (\d*) (\w)$").unwrap();
        let start_cap = start_re.captures_iter(&input[1]).next().unwrap();
        let start = Vec2::new(start_cap[1].parse().unwrap(), start_cap[2].parse().unwrap());

        let direction: Direction = match &start_cap[3] {
            "N" => Direction::North,
            "S" => Direction::South,
            "E" => Direction::East,
            "W" => Direction::West,
            d => panic!("Invaid direction {}", d),
        };

        // Loop through the instructions
        let instructions = input[2]
            .chars()
            .map(|c| match &c {
                'L' => Movement::Left,
                'R' => Movement::Right,
                'F' => Movement::Forward,
                m => panic!("Invalid movement {}", m),
            })
            .collect();

        Martian {
            bounds,
            location: start,
            direction,
            instructions,
        }
    }

    /// Creates a new martian object from the contents of a given file
    /// (Currently) panics if that file doesn't exist, or if the instructions are invalid
    pub fn from_file(filename: &PathBuf) -> Self {
        let input = get_file_content(filename);
        Martian::from_strings(input)
    }

    /// Attempt to move the martian
    pub fn attempt_movements(&mut self) {
        for instruction in &self.instructions {
            match instruction {
                Movement::Left => {
                    self.direction = self.direction.rotate_left();
                }
                Movement::Right => {
                    self.direction = self.direction.rotate_right();
                }
                Movement::Forward => {
                    match self.direction {
                        Direction::North => self.location.y += 1,
                        Direction::East => self.location.x += 1,
                        Direction::South => self.location.y -= 1,
                        Direction::West => self.location.x -= 1,
                    }

                    if self.out_of_bounds() {
                        panic!("LOST");
                    }
                }
            }
        }
    }

    pub fn out_of_bounds(&self) -> bool {
        (self.location.y > self.bounds.y)
            || (self.location.y < 0)
            || (self.location.x > self.bounds.x)
            || (self.location.x < 0)
    }
}
