pub fn part1(grid: &str) -> i32 {
    let grid_padded = append_non_paper_rolls_to_grid_border(grid);
    let (count, _) = count_and_remove_accessible_paper_rolls(grid_padded, false);
    count
}

pub fn part2(grid: &str) -> i32 {
    let mut grid_padded = append_non_paper_rolls_to_grid_border(grid);
    let mut nof_total_removable_paper_rolls = 0;

    loop {
        let (nof_accessable_paper_rolls, new_grid) =
            count_and_remove_accessible_paper_rolls(grid_padded, true);

        if nof_accessable_paper_rolls == 0 {
            break;
        }

        nof_total_removable_paper_rolls += nof_accessable_paper_rolls;
        grid_padded = new_grid;
    }

    nof_total_removable_paper_rolls
}

/// Counts accessible paper rolls and optionally removes them from the grid.
/// Returns the count and the (possibly modified) grid.
fn count_and_remove_accessible_paper_rolls(
    mut grid: Vec<Vec<char>>,
    remove: bool,
) -> (i32, Vec<Vec<char>>) {
    let mut nof_accessable_paper_rolls = 0;

    // row
    let row_start = 1;
    let row_end = grid.len() - 2;

    // col
    let col_start = 1;
    let col_end = grid[0].len() - 2;

    for row in row_start..=row_end {
        for col in col_start..=col_end {
            // Only count paper rolls (@), not empty spaces (.)
            if grid[row][col] != '@' {
                continue;
            }

            let nof_paper_rolls = count_adjacent_paper_rolls(&grid, row, col);

            if nof_paper_rolls < 4 {
                nof_accessable_paper_rolls += 1;
                if remove {
                    grid[row][col] = '.';
                }
            }
        }
    }

    (nof_accessable_paper_rolls, grid)
}

/// This function appends non paper rolls ('.') around the edges of the grid
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

/// Count adjacent paper rolls around a given position in the grid
pub fn count_adjacent_paper_rolls(grid: &[Vec<char>], row: usize, col: usize) -> i32 {
    let mut count = 0;

    // Check all 8 neighbors around the position
    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            // Skip the center cell
            if row_offset == 0 && col_offset == 0 {
                continue;
            }

            let neighbor_row = (row as i32 + row_offset) as usize;
            let neighbor_col = (col as i32 + col_offset) as usize;

            if grid[neighbor_row][neighbor_col] == '@' {
                count += 1;
            }
        }
    }

    count
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
    #[case(vec![vec!['.', '.', '@'], vec!['@', '@', '@'], vec!['@', '@', '@']], 1, 1, 6)]
    #[case(vec![vec!['.', '.', '.'], vec!['.', '@', '.'], vec!['.', '.', '.']], 1, 1, 0)]
    #[case(vec![vec!['@', '@', '@'], vec!['@', '@', '@'], vec!['@', '@', '@']], 1, 1, 8)]
    #[case(vec![vec!['@', '@', '@'], vec!['@', '.', '@'], vec!['@', '@', '@']], 1, 1, 8)]
    #[case(vec![vec!['@', '@', '@'], vec!['@', '@', '@'], vec!['@', '@', '.']], 1, 1, 7)]
    fn test_count_adjacent_paper_rolls(
        #[case] grid: Vec<Vec<char>>,
        #[case] row: usize,
        #[case] col: usize,
        #[case] expected_paper_rolls: i32,
    ) {
        assert_eq!(
            count_adjacent_paper_rolls(&grid, row, col),
            expected_paper_rolls
        );
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
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2_example() {
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
        assert_eq!(part2(input), 43);
    }
}
