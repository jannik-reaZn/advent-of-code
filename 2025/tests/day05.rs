use aoc_2025::days::day05::*;
use rstest::rstest;
use std::collections::HashSet;

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
fn test_is_ingredient_fresh(#[case] id: i64, #[case] expected: bool) {
    let ingredient_range = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
    let result = is_ingredient_fresh(id, &ingredient_range);
    assert_eq!(result, expected);
}

#[test]
fn test_get_ingredients_from_range() {
    let ingredient_range = vec![(3, 5), (10, 14), (16, 20), (12, 18)];
    let ingredients = get_ingredients_from_range(&ingredient_range);
    let expected_ingredients: HashSet<i64> =
        HashSet::from([3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    assert!(ingredients == expected_ingredients);
}

#[test]
fn test_merge_ingredient_ranges() {
    let ingredient_range: Vec<(i64, i64)> = vec![(3, 5), (4, 5), (16, 20), (12, 18)];
    let merged_ranges: Vec<(i64, i64)> = merge_ingredient_ranges(&ingredient_range);
    let expected_merged_ranges: Vec<(i64, i64)> = vec![(3, 5), (12, 20)];
    assert_eq!(merged_ranges, expected_merged_ranges);
}

#[test]
fn test_part1() {
    let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    let result = part1(input);
    assert_eq!(result, 3);
}

#[test]
fn test_part2() {
    let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
    let result = part2(input);
    assert_eq!(result, 14);
}
