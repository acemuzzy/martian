//! Integration test functions.
//!
//! These load various test cases from files in the same directory, and check they each work as expected.

extern crate martian;

use martian::run_file;

use std::path::PathBuf;

/// Confirm the sample file works as it ought
#[test]
fn test_sample_file() {
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("input.txt");

    let output = run_file(&path);
    assert_eq!(output.unwrap(), vec!["1 1 E", "3 3 N LOST", "2 3 S"]);
}

/// Test for missing files
#[test]
fn test_invalid_file() {
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("missing.txt");

    let output = run_file(&path);
    assert!(output.is_err());
}

/// Test for empty files
#[test]
fn test_empty_file() {
    let mut path = PathBuf::new();
    path.push("tests");
    path.push("empty.txt");

    let output = run_file(&path);
    assert!(output.is_err());
}

/// Test for invalid files, with varying types of error in
#[test]
fn test_invalid_files() {
    for ii in 1..=5 {
        let mut path = PathBuf::new();
        path.push("tests");
        path.push(format!("invalid{}.txt", ii));

        let output = run_file(&path);
        assert!(output.is_err());
    }
}
