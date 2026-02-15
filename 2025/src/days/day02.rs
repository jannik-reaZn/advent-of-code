/// Solution for Day 2
///
/// This module contains the logic for solving both parts of day 2.
/// Tests are included in this file using #[cfg(test)].

pub fn part1(input: &str) -> i64 {
    let mut result = 0;
    let splitted_sequences = split_sequences(input);
    for seq in splitted_sequences {
        let start = seq.0;
        let end = seq.1;
        for number in start..=end {
            let number_str = number.to_string();
            if is_sequence_part_1(&number_str) {
                result += number;
            }
        }
    }
    result
}

pub fn part2(input: &str) -> i64 {
    let mut result = 0;
    let splitted_sequences = split_sequences(input);
    for seq in splitted_sequences {
        let start = seq.0;
        let end = seq.1;
        for number in start..=end {
            let number_str = number.to_string();
            if is_sequence_part_2(&number_str) {
                result += number;
            }
        }
    }
    result
}

/// Splits the input string into a vector of tuples representing the start and end of each sequence.
pub fn split_sequences(input: &str) -> Vec<(i64, i64)> {
    let mut splitted_sequences: Vec<(i64, i64)> = Vec::new();
    let sequence_ranges = input.split(",");
    for sequences in sequence_ranges {
        let sequence = sequences.split("-");
        let collection = sequence.collect::<Vec<&str>>();
        splitted_sequences.push((
            collection[0].parse::<i64>().unwrap(),
            collection[1].parse::<i64>().unwrap(),
        ));
    }
    splitted_sequences
}

/// Checks if a given string is made only of some sequence of digits repeated twice
pub fn is_sequence_part_1(seq: &str) -> bool {
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

pub fn is_sequence_part_2(seq: &str) -> bool {
    let seq_size = seq.len();

    if seq_size < 2 {
        return false;
    }

    // Try different pattern lengths (window_size)
    for window_size in 1..=seq_size {
        // Check if the entire string can be made by repeating this pattern
        if seq_size % window_size == 0 {
            let pattern = &seq[0..window_size];
            let num_repeats = seq_size / window_size;

            // Must be repeated at least twice
            if num_repeats >= 2 {
                let all_match = (0..num_repeats).all(|i| {
                    let start = i * window_size;
                    let end = start + window_size;
                    &seq[start..end] == pattern
                });

                if all_match {
                    return true;
                }
            }
        }
    }

    false
}

/// Checks wether all fragents all equal to each other
pub fn check_all_fragments_for_equality(fragments: Vec<&str>) -> bool {
    if fragments.len() == 0 {
        return false;
    }
    let first = fragments[0];
    let all_fragments_equal = fragments.iter().all(|&f| f == first);
    return all_fragments_equal;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part1_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part1(input), 1227775554);
    }

    #[rstest]
    #[case("55", true)]
    #[case("6464", true)]
    #[case("123123", true)]
    #[case("101", false)]
    #[case("5", false)]
    fn test_is_sequence_part_1(#[case] sequence: &str, #[case] expected: bool) {
        let result = is_sequence_part_1(sequence);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("12341234", true)]
    #[case("123123123", true)]
    #[case("1212121212", true)]
    #[case("1111111", true)]
    #[case("55", true)]
    #[case("6464", true)]
    #[case("123123", true)]
    #[case("101", false)]
    #[case("5", false)]
    fn test_is_sequence_part_2(#[case] sequence: &str, #[case] expected: bool) {
        let result = is_sequence_part_2(sequence);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part2(input), 4174379265);
    }

    #[test]
    fn test_split_sequences() {
        let input = "11-22,95-115";
        let splitted_sequence = split_sequences(input);
        assert!(splitted_sequence.len() == 2);
        assert!(splitted_sequence[0] == (11, 22));
        assert!(splitted_sequence[1] == (95, 115));
    }

    #[test]
    fn test_fragments_equal() {
        let fragments = vec!["123", "123"];
        assert!(check_all_fragments_for_equality(fragments) == true)
    }

    #[test]
    fn test_fragments_not_equal() {
        let fragments = vec!["123", "456"];
        assert!(check_all_fragments_for_equality(fragments) == false)
    }
}
