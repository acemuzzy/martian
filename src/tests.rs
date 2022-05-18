//! Unit test functions

#[cfg(test)]
mod tests {

    use crate::direction::Direction;
    use crate::martian::Martian;
    use crate::movement::Movement;
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

    /// Test we can move a martian (first OK example)
    #[test]
    fn test_movement() {
        let mut m: Martian = Martian::from_strings(vec![
            "5 3".to_string(),
            "1 1 E".to_string(),
            "RFRFRFRF".to_string(),
        ]);

        m.attempt_movements();

        assert_eq!(format!("{}", m), "1 1 E");
    }

    /// Test movement out of bounds fails
    #[test]
    #[should_panic]
    fn test_movement_oob() {
        let mut m: Martian = Martian::from_strings(vec![
            "5 3".to_string(),
            "3 2 N".to_string(),
            "FRRFLLFFRRFLL".to_string(),
        ]);

        m.attempt_movements();
    }
}
