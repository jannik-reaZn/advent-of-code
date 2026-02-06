/// Binary executable for Day 03
/// 
/// Run with: cargo run --bin day03
/// Run tests: cargo test day03

use aoc_2025::days::day03;
use aoc_2025::read_input;

fn main() {
    let input = read_input(3);
    
    let result_part1 = day03::part1(&input);
    println!("Day 03 - Part 1: {}", result_part1);
    
    let result_part2 = day03::part2(&input);
    println!("Day 03 - Part 2: {}", result_part2);
}
