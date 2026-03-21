/// Solution for Day 5
///
/// This module contains the logic for solving both parts of day 5.
/// Tests are included in this file using #[cfg(test)].
use std::collections::HashSet;

pub fn part1(input: &str) -> i64 {
    let mut amount_fresh_ingredients = 0;
    let (ingredient_range, ingredient_ids) = prepare_input(input);

    for ingredient_id in ingredient_ids {
        if is_ingredient_fresh(ingredient_id, &ingredient_range) {
            amount_fresh_ingredients += 1;
        }
    }

    amount_fresh_ingredients
}

// This approach is naive since the ingredient range can be very large.
// The algorithm uses a HashSet to store all the ingredient ids that are in the range, which can be memory intensive if the range is large.
// A more efficient approach would be to implement a merge of the ranges and then check if the ingredient ids fall within any of the merged ranges,
// which would reduce the memory usage and improve the performance of the algorithm.
pub fn part2_naive(input: &str) -> i64 {
    let (ingredient_range, _) = prepare_input(input);
    let ingredients = get_ingredients_from_range(&ingredient_range);
    ingredients.len() as i64
}

pub fn part2(input: &str) -> i64 {
    0
}

pub fn prepare_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut ingredient_range: Vec<(i64, i64)> = Vec::new();
    let mut ingredient_ids: Vec<i64> = Vec::new();

    for line in input.lines() {
        if let Some((start, end)) = line.split_once('-') {
            let start = start.trim().parse::<i64>().unwrap();
            let end = end.trim().parse::<i64>().unwrap();
            ingredient_range.push((start, end));
        } else if !line.trim().is_empty() {
            let id = line.trim().parse::<i64>().unwrap();
            ingredient_ids.push(id);
        }
    }

    (ingredient_range, ingredient_ids)
}

pub fn is_ingredient_fresh(id: i64, ingredient_range: &Vec<(i64, i64)>) -> bool {
    for (start, end) in ingredient_range {
        if id >= *start && id <= *end {
            return true;
        }
    }
    false
}

pub fn get_ingredients_from_range(ingredient_range: &Vec<(i64, i64)>) -> HashSet<i64> {
    let mut ingredient_in_ranges: HashSet<i64> = HashSet::new();

    for (start, end) in ingredient_range {
        for id in *start..=*end {
            ingredient_in_ranges.insert(id);
        }
    }

    ingredient_in_ranges
}

pub fn merge_ingredient_ranges(ingredient_range: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut ingredient_ids: Vec<i64> = Vec::new();
    for (start, end) in ingredient_range {
        ingredient_ids.push(*start);
        ingredient_ids.push(*end);
    }

    ingredient_ids.sort();
    ingredient_ids.dedup();

    return ingredient_ids;
}
