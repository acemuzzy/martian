//! Unit test functions

#[cfg(test)]
mod tests {

    use crate::direction::Direction;
    use crate::map::Map;
    use crate::martian::Martian;
    use crate::movement::Movement;
    use crate::Vec2;

    /// Test we can parse a martian from a simple set of strings
    #[test]
    fn test_parsing() {
        let map = Map::new(Vec2::new(1, 2));
        let m: Martian = Martian::from_strings(&map, vec!["3 4 E", "LRF"]);

        assert_eq!(
            m,
            Martian::new(
                &Map::new(Vec2::new(1, 2)),
                Vec2::new(3, 4),
                Direction::East,
                vec![Movement::Left, Movement::Right, Movement::Forward]
            )
        );
    }

    /// Test we can parse a martian from a fuller set of strings
    #[test]
    fn test_parsing_multidigit() {
        let map = Map::from_string("11 22");
        let m: Martian = Martian::from_strings(&map, vec!["33 44 W", "FFFFFF"]);

        assert_eq!(
            m,
            Martian::new(
                &Map::new(Vec2::new(11, 22)),
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
        let map = Map::new(Vec2::new(5, 3));
        let mut m: Martian = Martian::from_strings(&map, vec!["1 1 E", "RFRFRFRF"]);

        let res = m.attempt_movements();

        assert_eq!(res.unwrap(), "1 1 E");
    }

    /// Test movement out of bounds fails
    #[test]
    fn test_movement_oob() {
        let map = Map::new(Vec2::new(5, 3));
        let mut m: Martian = Martian::from_strings(&map, vec!["3 2 N", "FRRFLLFFRRFLL"]);

        let res = m.attempt_movements();
        if let Err(msg) = res {
            assert_eq!(msg, "3 3 N LOST");
        } else {
            panic!("Failed to get lost!")
        }
    }
}
