//! Direction module

use std::fmt;

/// Directions in which the martian may face
#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// String form of the martian (as used e.g. in successful output)
impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "N",
                Direction::South => "S",
                Direction::East => "E",
                Direction::West => "W",
            }
        )
    }
}

impl Direction {
    pub fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}
