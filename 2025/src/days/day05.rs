/// Solution for Day 5
///
/// This module contains the logic for solving both parts of day 5.
/// Tests are included in this file using #[cfg(test)].
use std::{
    cmp::{max, min},
    collections::HashSet,
};

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
    let (ingredient_range, _) = prepare_input(input);
    let ingredients = merge_ingredient_ranges(&ingredient_range);

    let mut number_of_fresh_ingredients = 0;
    for ingredient in ingredients {
        number_of_fresh_ingredients += ingredient.1 - ingredient.0;
    }

    number_of_fresh_ingredients
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

// trait Stack {
//     fn top(&self) -> Option<&f64>;
// }

// impl Stack for Vec<f64> {
//     fn top(&self) -> Option<&f64> {
//         match &self[..] {
//             [] => None,
//             [.., n] => Some(n),
//         }
//     }
// }

/*
Algorithm:
1. If no ingredients, return empty vec
2. Sort vec of tuples. First check first element, if equal use the second element.
3. Remove duplicates
4. Push first tuple onto stack
5. Iterate over 1..n tuple
    5.1 Check for overlapping ranges.
    This is only the case of the first element in tuple is larger than the the first element in the tuple at the top of the stack
    AND smaller than the second elements of the same tuple
    5.2 If it is overlapping, the ranges can be merged.
    The question which second element should be taken (the tuple form the stack or the tuple in the current iteration),
    is answered by its maximum value.
    5.3 If no overlap is present, the tuple is pushed to the stack.
*/
pub fn merge_ingredient_ranges(ingredient_range: &Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut merged_ingredient_ids: Vec<(i64, i64)> = Vec::new();

    merged_ingredient_ids.sort();
    merged_ingredient_ids.dedup();

    merged_ingredient_ids.push(ingredient_range[0]);

    for i in 1..ingredient_range.len() {
        let size = merged_ingredient_ids.len();
        // Check for overlap
        if merged_ingredient_ids[size - 1].0 <= ingredient_range[i].0
            && ingredient_range[i].0 <= merged_ingredient_ids[size - 1].1
        {
            // Merge
            merged_ingredient_ids[size - 1].1 =
                max(merged_ingredient_ids[size - 1].1, ingredient_range[i].1)
        } else if merged_ingredient_ids[size - 1].0 <= ingredient_range[i].1
            && ingredient_range[i].1 <= merged_ingredient_ids[size - 1].1
        {
            merged_ingredient_ids[size - 1].0 =
                min(merged_ingredient_ids[size - 1].0, ingredient_range[i].0)
        } else {
            // No overlap
            merged_ingredient_ids.push(ingredient_range[i])
        }
    }

    return merged_ingredient_ids;
}
