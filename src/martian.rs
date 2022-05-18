//! Main martian module, with code to handle its instantiation and movement

use crate::direction::Direction;
use crate::map::Map;
use crate::movement::Movement;
use crate::Vec2;

use regex::Regex;
use std::fmt;

/// The martian structure itself
#[derive(Debug, PartialEq)]
pub struct Martian<'a> {
    map: &'a Map,
    location: Vec2,
    direction: Direction,
    instructions: Vec<Movement>,
}

/// String form of the martian (as used e.g. in successful output)
impl<'a> fmt::Display for Martian<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.location.x, self.location.y, self.direction
        )
    }
}

impl<'a> Martian<'a> {
    /// Create a new martian instance, with explicit parameters
    pub fn new(
        map: &'a Map,
        location: Vec2,
        direction: Direction,
        instructions: Vec<Movement>,
    ) -> Self {
        Martian {
            map,
            location,
            direction,
            instructions,
        }
    }

    /// Create a new martian instances, from a set of strings
    /// (Currently) does no error checking, so mainline only
    pub fn from_strings(map: &'a Map, input: Vec<&str>) -> Self {
        // Parse out the starting position / direction (two co-ordinates, plus a character)
        let start_re = Regex::new(r"^(\d*) (\d*) (\w)$").unwrap();
        let start_cap = start_re.captures_iter(input[0]).next().unwrap();
        let start = Vec2::new(start_cap[1].parse().unwrap(), start_cap[2].parse().unwrap());

        let direction: Direction = match &start_cap[3] {
            "N" => Direction::North,
            "S" => Direction::South,
            "E" => Direction::East,
            "W" => Direction::West,
            d => panic!("Invaid direction {}", d),
        };

        // Loop through the instructions
        let instructions = input[1]
            .chars()
            .map(|c| match &c {
                'L' => Movement::Left,
                'R' => Movement::Right,
                'F' => Movement::Forward,
                m => panic!("Invalid movement {}", m),
            })
            .collect();

        Martian {
            map,
            location: start,
            direction,
            instructions,
        }
    }

    /// Attempt to move the martian
    pub fn attempt_movements(&mut self) -> Result<String, String> {
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

                    // We've fallen off - work out where we were, and bail
                    if self.out_of_bounds() {
                        self.move_backwards();
                        return Err(format!("{} LOST", &self));
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

    /// Check if out of bounds
    pub fn out_of_bounds(&self) -> bool {
        (self.location.y > self.map.bounds.y)
            || (self.location.y < 0)
            || (self.location.x > self.map.bounds.x)
            || (self.location.x < 0)
    }
}
