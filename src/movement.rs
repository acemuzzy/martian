//! Movement module

/// Movements the martian may perform
#[derive(Debug, PartialEq, Clone)]
pub enum Movement {
    Forward,
    Left,
    Right,
}

impl Movement {
    /// Parse
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            'L' => Ok(Movement::Left),
            'R' => Ok(Movement::Right),
            'F' => Ok(Movement::Forward),
            m => Err(format!("Invalid movement {}", m)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Movement;

    /// Test symmetry
    #[test]
    fn test_valid() {
        for c in vec!['L', 'R', 'F'] {
            assert!(Movement::from_char(c).is_ok());
        }
    }

    /// Test symmetry
    #[test]
    fn test_invalid() {
        for c in vec!['l', 'X', '1'] {
            assert!(Movement::from_char(c).is_err());
        }
    }
}
