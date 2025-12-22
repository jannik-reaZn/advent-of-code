const DIAL_MAX_NUMBER_OF_POSITIONS: i32 = 99;

pub fn part1(input: &str) -> u32 {
    // Parse input into rotations
    let rotations: Vec<(&str, i32)> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let trimmed = line.trim();
            let direction = &trimmed[0..1];
            let amount = trimmed[1..].parse::<i32>().unwrap();
            (direction, amount)
        })
        .collect();

    find_out_password_part_1(&rotations)
}

pub fn part2(input: &str) -> u32 {
    // Parse input into rotations
    let rotations: Vec<(&str, i32)> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let trimmed = line.trim();
            let direction = &trimmed[0..1];
            let amount = trimmed[1..].parse::<i32>().unwrap();
            (direction, amount)
        })
        .collect();

    find_out_password_part_2(&rotations)
}

/// Calulates password for the dial.
/// This is done by counting how often the dial points to 0.
pub fn find_out_password_part_2(rotations: &[(&str, i32)]) -> u32 {
    let mut password: u32 = 0;
    let mut dial_current: i32 = 50;

    for rotation in rotations {
        let nof_rotations = rotation.1 / (DIAL_MAX_NUMBER_OF_POSITIONS + 1);
        password += nof_rotations as u32;

        let dial_next = apply_rotation_to_dial(dial_current, &rotation);

        if dial_next == 0 {
            // Landed on zero
            password += 1;
        } else if dial_current != 0 && dial_current > dial_next && rotation.0 == "R" {
            // Crossing zero clockwise (not starting from zero)
            password += 1;
        } else if dial_current != 0 && dial_current < dial_next && rotation.0 == "L" {
            // Crossing zero counter-clockwise (not starting from zero)
            password += 1;
        }
        dial_current = dial_next;
    }
    password
}

/// Calulates password for the dial.
/// This is done by counting how often the dial points to 0.
pub fn find_out_password_part_1(rotations: &[(&str, i32)]) -> u32 {
    let mut password: u32 = 0;
    // The current position always starts at 50
    let mut dial: i32 = 50;

    for rotation in rotations {
        dial = apply_rotation_to_dial(dial, &rotation);
        if dial == 0 {
            password += 1
        }
    }

    password
}

pub fn apply_rotation_to_dial(dial: i32, rotation: &(&str, i32)) -> i32 {
    let rotation_amount = if rotation.0 == "L" {
        -rotation.1
    } else {
        rotation.1
    };

    // Apply rotation and use rem_euclid to handle any rotation size and negative values
    // rem_euclid ensures the result is always in the range [0, modulus)
    let new_position = (dial + rotation_amount).rem_euclid(DIAL_MAX_NUMBER_OF_POSITIONS + 1);

    new_position
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(42, "R", 10, 52, "basic right rotation")]
    #[case(42, "L", 42, 0, "left rotation to zero")]
    #[case(42, "L", 43, 99, "left rotation under limit")]
    #[case(98, "R", 5, 3, "right rotation over limit")]
    #[case(50, "R", 250, 0, "large right rotation (>99)")]
    #[case(50, "L", 150, 0, "large left rotation")]
    #[case(25, "R", 100, 25, "exact multiple rotation")]
    #[case(10, "L", 310, 0, "very large left rotation")]
    fn test_apply_rotation_to_dial(
        #[case] current_position: i32,
        #[case] direction: &str,
        #[case] amount: i32,
        #[case] expected: i32,
        #[case] _description: &str,
    ) {
        let rotation = (direction, amount);
        assert_eq!(
            apply_rotation_to_dial(current_position, &rotation),
            expected
        );
    }

    #[test]
    fn test_find_out_password_part_1() {
        let rotations = [
            ("L", 68),
            ("L", 30),
            ("R", 48),
            ("L", 5),
            ("R", 60),
            ("L", 55),
            ("L", 1),
            ("L", 99),
            ("R", 14),
            ("L", 82),
        ];
        assert_eq!(find_out_password_part_1(&rotations), 3)
    }

    #[test]
    fn test_part_1() {
        let input = "
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
        ";
        assert_eq!(part1(input), 3);
    }

    /// Tests for part 2

    #[rstest]
    #[case(&[("L", 68)], 1, "single left rotation")]
    #[case(&[("L", 50)], 1, "left rotation to zero")]
    #[case(&[("R", 220)], 2, "right rotation with multiple crossings")]
    #[case(&[("L", 50), ("L", 5)], 1, "multiple rotations")]
    fn test_find_out_password_part_2_parameterized(
        #[case] rotations: &[(&str, i32)],
        #[case] expected: u32,
        #[case] _description: &str,
    ) {
        assert_eq!(find_out_password_part_2(rotations), expected)
    }

    #[test]
    fn test_find_out_password_part_2() {
        let rotations = [
            ("L", 68),
            ("L", 30),
            ("R", 48),
            ("L", 5),
            ("R", 60),
            ("L", 55),
            ("L", 1),
            ("L", 99),
            ("R", 14),
            ("L", 82),
        ];
        assert_eq!(find_out_password_part_2(&rotations), 6)
    }

    #[test]
    fn test_part_2() {
        let input = "
        L68
        L30
        R48
        L5
        R60
        L55
        L1
        L99
        R14
        L82
        ";
        assert_eq!(part2(input), 6);
    }
}
