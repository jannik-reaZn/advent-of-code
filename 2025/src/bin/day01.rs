/// Binary executable for Day 01
/// 
/// Run with: cargo run --bin day01
/// Run tests: cargo test day01

use aoc_2025::days::day01;
use aoc_2025::read_input;

fn main() {
    let input = read_input(1);
    
    let result_part1 = day01::part1(&input);
    println!("Day 01 - Part 1: {}", result_part1);
    
    let result_part2 = day01::part2(&input);
    println!("Day 01 - Part 2: {}", result_part2);
}
