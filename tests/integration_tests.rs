//! Integration test functions

extern crate martian;

use martian::run_file;

use std::path::PathBuf;

/// Test we can accurately load a martian from a file
#[test]
fn test_from_file() {
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("input.txt");
    
    let output = run_file(&path);
    assert_eq!(
        output, 
        vec!["1 1 E", "3 3 N LOST", "2 3 S"]
    );
}
