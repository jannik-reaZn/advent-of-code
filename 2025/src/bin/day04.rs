/// Binary executable for Day 04
/// 
/// Run with: cargo run --bin day04
/// Run tests: cargo test day04

use aoc_2025::days::day04;
use aoc_2025::read_input;

fn main() {
    let input = read_input(4);
    
    let result_part1 = day04::part1(&input);
    println!("Day 04 - Part 1: {}", result_part1);
    
    let result_part2 = day04::part2(&input);
    println!("Day 04 - Part 2: {}", result_part2);
}
