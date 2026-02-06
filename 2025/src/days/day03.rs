/// Solution for Day 3
///
/// This module contains the logic for solving both parts of day 3.
/// Tests are included in this file using #[cfg(test)].

pub fn part1(banks: &str) -> i32 {
    let mut max_joltage = 0;

    for bank in banks.lines() {
        let highest_possible_joltage = find_largest_possible_jolt(bank.trim());
        max_joltage += highest_possible_joltage;
    }

    return max_joltage;
}

pub fn find_largest_possible_jolt(bank: &str) -> i32 {
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

pub fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
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
    fn test_find_largest_possible_jolt(#[case] bank: &str, #[case] highest_possible_voltage: i32) {
        assert_eq!(find_largest_possible_jolt(bank), highest_possible_voltage);
    }

    #[test]
    fn test_part2_example() {
        let input = "your test input here";
        assert_eq!(part2(input), 0); // Replace with expected result
    }
}
