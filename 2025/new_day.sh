#!/bin/bash

# Script to generate files for a new Advent of Code day
# Usage: ./new_day.sh <day_number>

if [ -z "$1" ]; then
    echo "Usage: ./new_day.sh <day_number>"
    echo "Example: ./new_day.sh 2"
    exit 1
fi

DAY=$(printf "%02d" $1)

echo "Creating files for Day $DAY..."

# Create the day module (src/days/dayXX.rs)
cat > "src/days/day${DAY}.rs" << 'EOF'
/// Solution for Day DAY_NUM
/// 
/// This module contains the logic for solving both parts of day DAY_NUM.
/// Tests are included in this file using #[cfg(test)].

pub fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

pub fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "your test input here";
        assert_eq!(part1(input), 0); // Replace with expected result
    }

    #[test]
    fn test_part2_example() {
        let input = "your test input here";
        assert_eq!(part2(input), 0); // Replace with expected result
    }
}
EOF

# Replace DAY_NUM placeholder
sed -i "s/DAY_NUM/$1/g" "src/days/day${DAY}.rs"

# Create the binary (src/bin/dayXX.rs)
cat > "src/bin/day${DAY}.rs" << EOF
/// Binary executable for Day $DAY
/// 
/// Run with: cargo run --bin day${DAY}
/// Run tests: cargo test day${DAY}

use aoc_2025::days::day${DAY};
use aoc_2025::read_input;

fn main() {
    let input = read_input($1);
    
    let result_part1 = day${DAY}::part1(&input);
    println!("Day $DAY - Part 1: {}", result_part1);
    
    let result_part2 = day${DAY}::part2(&input);
    println!("Day $DAY - Part 2: {}", result_part2);
}
EOF

# Create input files
touch "inputs/day${DAY}.txt"
echo "TODO: Add your test input here (from the problem description)" > "inputs/day${DAY}_test.txt"

# Add module declaration to mod.rs if not already present
if ! grep -q "pub mod day${DAY};" "src/days/mod.rs"; then
    echo "pub mod day${DAY};" >> "src/days/mod.rs"
fi

echo "✅ Created:"
echo "   - src/days/day${DAY}.rs (solution + tests)"
echo "   - src/bin/day${DAY}.rs (executable)"
echo "   - inputs/day${DAY}.txt"
echo "   - inputs/day${DAY}_test.txt"
echo ""
echo "Next steps:"
echo "   1. Add your puzzle input to inputs/day${DAY}.txt"
echo "   2. Add example input to inputs/day${DAY}_test.txt"
echo "   3. Implement solution in src/days/day${DAY}.rs"
echo "   4. Run with: cargo run --bin day${DAY}"
echo "   5. Test with: cargo test day${DAY}"
