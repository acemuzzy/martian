//! Integration test functions

extern crate martian;

use martian::direction::Direction;
use martian::martian::Martian;
use martian::movement::Movement;
use martian::Vec2;

use std::path::PathBuf;

/// Test we can accurately load a martian from a file
#[test]
fn test_from_file() {
    let m: Martian = Martian::from_file(&PathBuf::from("input.txt"));

    assert_eq!(
        m,
        Martian::new(
            Vec2::new(5, 3),
            Vec2::new(1, 1),
            Direction::East,
            vec![
                Movement::Right,
                Movement::Forward,
                Movement::Right,
                Movement::Forward,
                Movement::Right,
                Movement::Forward,
                Movement::Right,
                Movement::Forward,
            ]
        )
    );
}
