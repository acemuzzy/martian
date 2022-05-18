//! Map module.
//! Tracks the bounds of the map, and scent locations

use crate::Vec2;
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct Map {
    pub bounds: Vec2,
    pub scents: Vec<Vec2>,
}

impl Map {
    /// Constuctor
    pub fn new(bounds: Vec2) -> Self {
        Map {
            bounds,
            scents: vec![],
        }
    }

    /// Create a new martian instances, from a set of strings
    /// (Currently) does no error checking, so mainline only
    pub fn from_string(input: &str) -> Self {
        // Parse out the bounds (two co-ordinates)
        let bounds_re = Regex::new(r"^(\d*) (\d*)$").unwrap();
        let bounds_cap = bounds_re.captures_iter(input).next().unwrap();
        let bounds = Vec2::new(
            bounds_cap[1].parse().unwrap(),
            bounds_cap[2].parse().unwrap(),
        );

        Self::new(bounds)
    }
}
