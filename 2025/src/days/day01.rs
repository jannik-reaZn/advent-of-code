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
    let mut nof_rotations: i32 = 0;
    let mut dial_current: i32 = 50;
    let mut dial_next: i32 = 0;

    for rotation in rotations {
        nof_rotations = rotation.1 / DIAL_MAX_NUMBER_OF_POSITIONS as i32;
        password += nof_rotations as u32;

        dial_next = apply_rotation_to_dial(dial_current, &rotation);

        if dial_next == 0 {
            // Landed on zero
            password += 1;
            continue;
        } else if dial_current > dial_next && rotation.0 == "R" {
            // Crossing zero clockwise
            password += 1;
        } else if dial_current < dial_next && rotation.0 == "L" {
            // Crossing zero counter-clockwise
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

    #[test]
    fn test_apply_rotation_to_dial() {
        let current_position = 42;
        let rotation = ("R", 10);
        assert_eq!(apply_rotation_to_dial(current_position, &rotation), 52);
    }

    #[test]
    fn test_apply_rotation_to_dial_zero() {
        let current_position = 42;
        let rotation = ("L", 42);
        assert_eq!(apply_rotation_to_dial(current_position, &rotation), 0);
    }

    #[test]
    fn test_apply_rotation_to_dial_under_limit() {
        let current_position = 42;
        let rotation = ("L", 43);
        assert_eq!(apply_rotation_to_dial(current_position, &rotation), 99);
    }

    #[test]
    fn test_apply_rotation_to_dial_over_limit() {
        let current_position = 98;
        let rotation = ("R", 5);
        assert_eq!(apply_rotation_to_dial(current_position, &rotation), 3);
    }

    #[test]
    fn test_apply_rotation_large_right() {
        // Test rotation larger than dial range (>99)
        let current_position = 50;
        let rotation = ("R", 250); // 250 rotations = 2*100 + 50
        assert_eq!(apply_rotation_to_dial(current_position, &rotation), 0);
    }

    #[test]
    fn test_apply_rotation_large_left() {
        // Test large negative rotation
        let current_position = 50;
        let rotation = ("L", 150); // -150 rotations
        assert_eq!(apply_rotation_to_dial(current_position, &rotation), 0);
    }

    #[test]
    fn test_apply_rotation_exact_multiple() {
        // Test exact multiple of dial positions
        let current_position = 25;
        let rotation = ("R", 100); // Exactly one full rotation
        assert_eq!(apply_rotation_to_dial(current_position, &rotation), 25);
    }

    #[test]
    fn test_apply_rotation_negative_large() {
        // Test very large negative rotation
        let current_position = 10;
        let rotation = ("L", 310); // -310 rotations
        assert_eq!(apply_rotation_to_dial(current_position, &rotation), 0);
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

    #[test]
    fn test_find_out_password_part_2_left_rotation() {
        let rotations = [("L", 68)];
        assert_eq!(find_out_password_part_2(&rotations), 1)
    }

    #[test]
    fn test_find_out_password_part_2_left_rotation_to_zero() {
        let rotations = [("L", 50)];
        assert_eq!(find_out_password_part_2(&rotations), 1)
    }

    #[test]
    fn test_find_out_password_part_2_multiple_rotations() {
        let rotations = [("L", 50), ("L", 5)];
        assert_eq!(find_out_password_part_2(&rotations), 1)
    }

    #[test]
    fn test_find_out_password_part_2_right_rotation() {
        let rotations = [("R", 220)];
        assert_eq!(find_out_password_part_2(&rotations), 2)
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
}
