/// Binary executable for Day 05
/// 
/// Run with: cargo run --bin day05
/// Run tests: cargo test day05

use aoc_2025::days::day05;
use aoc_2025::read_input;

fn main() {
    let input = read_input(5);
    
    let result_part1 = day05::part1(&input);
    println!("Day 05 - Part 1: {}", result_part1);
    
    let result_part2 = day05::part2(&input);
    println!("Day 05 - Part 2: {}", result_part2);
}
