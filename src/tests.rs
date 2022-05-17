//! Unit test functions

#[cfg(test)]
mod tests {

    use crate::martian::{Direction, Martian, Movement};
    use crate::Vec2;

    /// Test we can parse a martian from a simple set of strings
    #[test]
    fn test_parsing() {
        let m: Martian = Martian::from_strings(vec![
            "1 2".to_string(),
            "3 4 E".to_string(),
            "LRF".to_string(),
        ]);

        assert_eq!(
            m,
            Martian::new(
                Vec2::new(1, 2),
                Vec2::new(3, 4),
                Direction::East,
                vec![Movement::Left, Movement::Right, Movement::Forward]
            )
        );
    }

    /// Test we can parse a martian from a fuller set of strings
    #[test]
    fn test_parsing_multidigit() {
        let m: Martian = Martian::from_strings(vec![
            "11 22".to_string(),
            "33 44 W".to_string(),
            "FFFFFF".to_string(),
        ]);

        assert_eq!(
            m,
            Martian::new(
                Vec2::new(11, 22),
                Vec2::new(33, 44),
                Direction::West,
                vec![
                    Movement::Forward,
                    Movement::Forward,
                    Movement::Forward,
                    Movement::Forward,
                    Movement::Forward,
                    Movement::Forward
                ]
            )
        );
    }
}
