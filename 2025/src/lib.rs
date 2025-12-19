// Shared utilities and modules for all days
pub mod days;

use std::fs;

/// Read input file for a specific day
pub fn read_input(day: u8) -> String {
    let filename = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&filename).unwrap_or_else(|_| panic!("Unable to read file: {}", filename))
}

/// Read test input file for a specific day
pub fn read_test_input(day: u8) -> String {
    let filename = format!("inputs/day{:02}_test.txt", day);
    fs::read_to_string(&filename).unwrap_or_else(|_| panic!("Unable to read file: {}", filename))
}
