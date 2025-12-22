/// Binary executable for Day 02
/// 
/// Run with: cargo run --bin day02
/// Run tests: cargo test day02

use aoc_2025::days::day02;
use aoc_2025::read_input;

fn main() {
    let input = read_input(2);
    
    let result_part1 = day02::part1(&input);
    println!("Day 02 - Part 1: {}", result_part1);
    
    let result_part2 = day02::part2(&input);
    println!("Day 02 - Part 2: {}", result_part2);
}
