//! Direction module

use crate::Vec2;
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
    /// Rotate the direction anti-clockwise
    pub fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }

    /// Rotate the direction clockwise
    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    /// Get a forrward movement vector, based of the current direction
    pub fn forward_vector(&self) -> Vec2 {
        match self {
            Direction::North => Vec2::new(0, 1),
            Direction::East => Vec2::new(1, 0),
            Direction::South => Vec2::new(0, -1),
            Direction::West => Vec2::new(-1, 0),
        }
    }
}
