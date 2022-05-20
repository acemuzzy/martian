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

    /// Instantiate a `Map` based of string input
    pub fn from_string(input: &str) -> Result<Self, String> {
        // Parse out the bounds (two co-ordinates)
        //
        // Use unwrap on the regex construction & subsequent parsing for simplicity's sake:
        // - the former is tested in all the UTs etc, and can't subsequently fail at runtime
        // - the latter must match given the form of the regex
        //
        // Do handle failure of the regex matching itself, however.
        let bounds_re = Regex::new(r"^(\d*) (\d*)$").unwrap();
        let bounds_cap = bounds_re
            .captures_iter(input)
            .next()
            .ok_or_else(|| "Failed to capture map co-ordinates".to_string())?;
        let bounds = Vec2::new(
            bounds_cap[1].parse().unwrap(),
            bounds_cap[2].parse().unwrap(),
        );

        Ok(Self::new(bounds))
    }
}
