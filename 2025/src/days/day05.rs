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
        let splitted_line = line.split('-').collect::<Vec<&str>>();
        if splitted_line.len() == 2 {
            let start = splitted_line[0].trim().parse::<i32>().unwrap();
            let end = splitted_line[1].trim().parse::<i32>().unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_part2_example() {
        let input = "your test input here";
        assert_eq!(part2(input), 0); // Replace with expected result
    }
}
