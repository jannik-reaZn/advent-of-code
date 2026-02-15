/// Solution for Day 4
///
/// This module contains the logic for solving both parts of day 4.
/// Tests are included in this file using #[cfg(test)].

pub fn part1(grid: &str) -> i32 {
    let nof_accessable_paper_rolls = 0;
    return nof_accessable_paper_rolls;
}

pub fn part2(input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

// This function appends non paper rolls around the edges of the grid
pub fn append_non_paper_rolls_to_grid_border(grid: &str) -> Vec<Vec<char>> {
    // Convert grid input into a 2D vector of chars
    let mut grid_padded: Vec<Vec<char>> = grid
        .lines()
        .map(|line| line.trim().chars().collect())
        .filter(|row: &Vec<char>| !row.is_empty())
        .collect();

    // Add non paper rolls, which are marked as '.', to the beginning and end
    for row in &mut grid_padded {
        row.insert(0, '.');
        row.push('.');
    }

    // Add first row and last row with non paper rolls
    match grid_padded.first() {
        Some(first_row) => {
            let length = first_row.len();
            let empty_row = vec!['.'; length];

            grid_padded.insert(0, empty_row.clone());
            grid_padded.push(empty_row);
        }
        None => {}
    }

    grid_padded
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

    #[test]
    fn test_append_non_paper_rolls_to_grid_border() {
        let input = "
        ..@@.@@@@.
        @@@.@.@.@@
        @@@@@.@.@@
        @.@@@@..@.
        @@.@@@@.@@
        .@@@@@@@.@
        .@.@.@.@@@
        @.@@@.@@@@
        .@@@@@@@@.
        @.@.@@@.@.
        ";
        let expected_output = vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '@', '@', '.', '@', '@', '@', '@', '.', '.'], // 1st row
            vec!['.', '@', '@', '@', '.', '@', '.', '@', '.', '@', '@', '.'], // 2nd row
            vec!['.', '@', '@', '@', '@', '@', '.', '@', '.', '@', '@', '.'], // 3rd row
            vec!['.', '@', '.', '@', '@', '@', '@', '.', '.', '@', '.', '.'], // 4th row
            vec!['.', '@', '@', '.', '@', '@', '@', '@', '.', '@', '@', '.'], // 5th row
            vec!['.', '.', '@', '@', '@', '@', '@', '@', '@', '.', '@', '.'], // 6th row
            vec!['.', '.', '@', '.', '@', '.', '@', '.', '@', '@', '@', '.'], // 7th row
            vec!['.', '@', '.', '@', '@', '@', '.', '@', '@', '@', '@', '.'], // 8th row
            vec!['.', '.', '@', '@', '@', '@', '@', '@', '@', '@', '.', '.'], // 9th row
            vec!['.', '@', '.', '@', '.', '@', '@', '@', '.', '@', '.', '.'], // 10th row
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        assert_eq!(
            append_non_paper_rolls_to_grid_border(input),
            expected_output
        );
    }

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
        let input = "
            ..@@.@@@@.
            @@@.@.@.@@
            @@@@@.@.@@
            @.@@@@..@.
            @@.@@@@.@@
            .@@@@@@@.@
            .@.@.@.@@@
            @.@@@.@@@@
            .@@@@@@@@.
            @.@.@@@.@.
        ";
        assert_eq!(part1(input), 0); // Replace with expected result
    }

    #[test]
    fn test_part2_example() {
        let input = "your test input here";
        assert_eq!(part2(input), 0); // Replace with expected result
    }
}
