/// Solution for Day 4
///
/// This module contains the logic for solving both parts of day 4.
/// Tests are included in this file using #[cfg(test)].

pub fn part1(input: &str) -> i32 {
    // TODO: Implement part 1
    0
}

pub fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

pub fn get_nof_adjecent_neighbours(matrix: [[char; 3]; 3]) -> i32 {
    let mut nof_adjecent_neighbours = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            // exclude middle
            if i == 1 && j == 1 {
                continue;
            }

            if matrix[i][j] == '@' {
                nof_adjecent_neighbours += 1;
            }
        }
    }
    nof_adjecent_neighbours
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case([['.', '.', '@'], ['@', '@', '@'], ['@', '@', '@']], 6)]
    #[case([['.', '.', '.'], ['.', '@', '.'], ['.', '.', '.']], 0)]
    #[case([['@', '@', '@'], ['@', '@', '@'], ['@', '@', '@']], 8)]
    #[case([['@', '@', '@'], ['@', '.', '@'], ['@', '@', '@']], 8)]
    #[case([['@', '@', '@'], ['@', '@', '@'], ['@', '@', '.']], 7)]
    fn test_get_nof_adjecent_neighbours(
        #[case] matrix: [[char; 3]; 3],
        #[case] expected_neighbours: i32,
    ) {
        assert_eq!(get_nof_adjecent_neighbours(matrix), expected_neighbours);
    }

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
