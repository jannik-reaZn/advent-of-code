/// Solution for Day 3
///
/// This module contains the logic for solving both parts of day 3.
/// Tests are included in this file using #[cfg(test)].

pub fn part1(banks: &str) -> i32 {
    let mut max_joltage = 0;

    for bank in banks.lines() {
        let highest_possible_joltage = find_largest_possible_joltage_part_1(bank.trim());
        max_joltage += highest_possible_joltage;
    }

    return max_joltage;
}

pub fn find_largest_possible_joltage_part_1(bank: &str) -> i32 {
    let bank_size = bank.len() as i32;
    let mut highest_possible_joltage = 0;

    if bank_size < 2 {
        return highest_possible_joltage;
    }

    // check if bank_size - 1 is needed
    for i in 0..bank_size {
        for j in i + 1..bank_size {
            let joltage_str = format!(
                "{}{}",
                &bank[i as usize..(i + 1) as usize],
                &bank[j as usize..(j + 1) as usize]
            );
            let joltage = joltage_str.parse::<i32>().unwrap_or(0);
            if joltage > highest_possible_joltage {
                highest_possible_joltage = joltage;
            }
        }
    }
    return highest_possible_joltage;
}

// PART 2
pub fn part2(banks: &str) -> i64 {
    let mut max_joltage = 0;

    for bank in banks.lines() {
        let highest_possible_joltage = find_largest_possible_joltage_part_2(bank.trim());
        max_joltage += highest_possible_joltage as i64;
    }

    return max_joltage;
}

pub fn find_largest_possible_joltage_part_2(bank: &str) -> i64 {
    let bank_digits: Vec<i64> = bank
        .chars()
        .map(|c| c.to_digit(10).unwrap_or(0) as i64)
        .collect();

    let bank_size = bank_digits.len();

    if bank_size < 12 {
        return 0;
    }

    // Greedy algorithm: for each position in the result (0..12),
    // find the largest digit in the remaining range that still leaves
    // enough digits for the remaining positions
    let mut result = Vec::new();
    let mut start_pos = 0;

    for result_pos in 0..12 {
        let remaining_needed = 12 - result_pos - 1; // Calculate how many digits (after the current picked digit) need to be picked
        let search_end = bank_size - remaining_needed; // Latest position we can pick from

        // Find the maximum digit in the valid range
        let mut max_digit = bank_digits[start_pos];
        let mut max_pos = start_pos;

        for pos in start_pos..search_end {
            if bank_digits[pos] > max_digit {
                max_digit = bank_digits[pos];
                max_pos = pos;
            }
        }

        result.push(max_digit);
        start_pos = max_pos + 1; // Next search starts after the digit we just picked
    }

    // Convert result vector to i64
    let mut highest_possible_joltage = 0;
    for digit in result {
        highest_possible_joltage = highest_possible_joltage * 10 + digit;
    }

    return highest_possible_joltage;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_part1_example() {
        let input = "
            987654321111111
            811111111111119
            234234234234278
            818181911112111
        ";
        assert_eq!(part1(input), 357);
    }

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("", 0)]
    #[case("1", 0)]
    fn test_find_largest_possible_joltage_part_1(
        #[case] bank: &str,
        #[case] highest_possible_voltage: i32,
    ) {
        assert_eq!(
            find_largest_possible_joltage_part_1(bank),
            highest_possible_voltage
        );
    }

    #[test]
    fn test_part2_example() {
        let input = "your test input here";
        assert_eq!(part2(input), 0); // Replace with expected result
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    #[case("", 0)]
    #[case("12345678901", 0)]
    fn test_find_largest_possible_joltage_part_2(
        #[case] bank: &str,
        #[case] highest_possible_voltage: i64,
    ) {
        assert_eq!(
            find_largest_possible_joltage_part_2(bank),
            highest_possible_voltage
        );
    }
}
