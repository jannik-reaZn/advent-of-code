/// Solution for Day 5
///
/// This module contains the logic for solving both parts of day 5.
/// Tests are included in this file using #[cfg(test)].

pub fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

pub fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

pub fn prepare_input(input: &str) -> (Vec<(i32, i32)>, Vec<i32>) {
    let mut ingredient_range: Vec<(i32, i32)> = Vec::new();
    let mut ingredient_ids: Vec<i32> = Vec::new();

    for line in input.lines() {
        if let Some((start, end)) = line.split_once('-') {
            let start = start.trim().parse::<i32>().unwrap();
            let end = end.trim().parse::<i32>().unwrap();
            ingredient_range.push((start, end));
        } else if !line.trim().is_empty() {
            let id = line.trim().parse::<i32>().unwrap();
            ingredient_ids.push(id);
        }
    }

    println!("Ingredient Ranges: {:?}", ingredient_range);
    println!("Ingredient IDs: {:?}", ingredient_ids);
    (ingredient_range, ingredient_ids)
}

pub fn is_ingredient_fresh(id: i32, ingredient_range: &Vec<(i32, i32)>) -> bool {
    for (start, end) in ingredient_range {
        if id >= *start && id <= *end {
            return true;
        }
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

    #[test]
    fn test_prepare_input() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

        let (ingredient_range, ingredient_ids) = prepare_input(input);
        assert_eq!(ingredient_range, vec![(3, 5), (10, 14), (16, 20), (12, 18)]);
        assert_eq!(ingredient_ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[rstest]
    #[case(1, false)]
    #[case(5, true)]
    #[case(8, false)]
    #[case(11, true)]
    #[case(17, true)]
    #[case(32, false)]
    fn test_is_ingredient_fresh(#[case] id: i32, #[case] expected: bool) {
        let ingredient_range = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
        let ingredient_ids = vec![1, 5, 8, 11, 17, 32];

        let is_ingredient_fresh = is_ingredient_fresh(id, &ingredient_range);
        assert_eq!(is_ingredient_fresh, expected);
    }

    #[test]
    fn test_part2_example() {
        let input = "your test input here";
        assert_eq!(part2(input), 0); // Replace with expected result
    }
}
