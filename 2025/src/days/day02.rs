/// Solution for Day 2
///
/// This module contains the logic for solving both parts of day 2.
/// Tests are included in this file using #[cfg(test)].

pub fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

pub fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

/// Cecks if a given string is made only of some sequence of digits repeated twice
pub fn is_sequence(seq: &str) -> bool {
    let seq_size = seq.len();

    if seq_size < 2 {
        return false;
    }

    let first_sub_sequence: &str = &seq[0..seq_size / 2];
    let second_sub_sequence: &str = &seq[(seq_size / 2)..seq_size];

    if first_sub_sequence == second_sub_sequence {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part1_example() {
        let input = "your test input here";
        assert_eq!(part1(input), 0); // Replace with expected result
    }

    #[rstest]
    #[case("55", true)]
    #[case("6464", true)]
    #[case("123123", true)]
    #[case("101", false)]
    #[case("5", false)]
    fn test_is_sequence(#[case] sequence: &str, #[case] expected: bool) {
        let result = is_sequence(sequence);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_example() {
        let input = "your test input here";
        assert_eq!(part2(input), 0); // Replace with expected result
    }
}
