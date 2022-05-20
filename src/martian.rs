//! Main martian module, with code to handle its instantiation and movement

use crate::direction::Direction;
use crate::map::Map;
use crate::movement::Movement;
use crate::Vec2;

use regex::Regex;
use std::fmt;

/// The martian structure itself
#[derive(Debug, PartialEq)]
pub struct Martian {
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
    pub fn new(location: Vec2, direction: Direction, instructions: Vec<Movement>) -> Self {
        Martian {
            location,
            direction,
            instructions,
        }
    }

    /// Create a new martian instances, from a set of strings
    pub fn from_strings(input: Vec<&str>) -> Result<Self, String> {
        // Parse out the starting position / direction (two co-ordinates, plus a character)
        //
        // Use unwrap on the regex construction & subsequent parsing for simplicity's sake:
        // - the former is tested in all the UTs etc, and can't subsequently fail at runtime
        // - the latter must match given the form of the regex
        //
        // Do handle failure of the regex matching itself, however.
        let start_re = Regex::new(r"^(\d*) (\d*) (\w)$").unwrap();
        let start_cap = start_re
            .captures_iter(input[0])
            .next()
            .ok_or_else(|| "Failed to capture martian start details".to_string())?;

        let start = Vec2::new(start_cap[1].parse().unwrap(), start_cap[2].parse().unwrap());
        let direction = Direction::from_char(start_cap[3].chars().next().unwrap())?;

        // Loop through the instructions
        let instructions: Result<Vec<Movement>, String> =
            input[1].chars().map(Movement::from_char).collect();
        let instructions = match instructions {
            Ok(i) => i,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(Martian {
            location: start,
            direction,
            instructions,
        })
    }

    /// Attempt to move the martian, by iterating through its instructions
    pub fn attempt_movements(&mut self, map: &mut Map) -> Result<String, String> {
        for instruction in self.instructions.clone() {
            match instruction {
                Movement::Left => {
                    self.direction = self.direction.rotate_left();
                }
                Movement::Right => {
                    self.direction = self.direction.rotate_right();
                }
                Movement::Forward => {
                    self.move_forwards();

                    // We would have fallen off!
                    //
                    // Rewind to where we were, and then either:
                    // - mark a new scent and bail, or,
                    // - use the scent and just ignore the movement by continuing
                    if self.out_of_bounds(map) {
                        self.move_backwards();

                        if !map.scents.contains(&self.location) {
                            map.scents.push(self.location.clone());
                            return Err(format!("{} LOST", &self));
                        }
                    }
                }
            }
        }

        // We've finished!
        Ok(format!("{}", &self))
    }

    /// Move the martian one space in the direction it's facing
    pub fn move_forwards(&mut self) {
        self.location += self.direction.forward_vector();
    }

    /// Move the martian one space in the opposite direction to how it's facing
    pub fn move_backwards(&mut self) {
        self.location += self.direction.forward_vector() * -1;
    }

    /// Check if out of bounds, by comparing to the map
    pub fn out_of_bounds(&self, map: &Map) -> bool {
        (self.location.y > map.bounds.y)
            || (self.location.y < 0)
            || (self.location.x > map.bounds.x)
            || (self.location.x < 0)
    }
}
