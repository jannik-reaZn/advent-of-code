/// Solution for Day 01
/// 
/// This module contains the logic for solving both parts of day 1.
/// Tests are included in this file using #[cfg(test)].

const DIAL_MAX_NUMBER_OF_POSITIONS: i32 = 99;

pub fn part1(_input: &str) -> i32 {
    // TODO: Implement part 1
    // This is where your solution logic goes
    0
}

pub fn part2(_input: &str) -> i32 {
    // TODO: Implement part 2
    0
}

/// Calulates password for the dial.
/// This is done by counting how often the dial points to 0.
pub fn find_out_password(rotations: &[(&str, i32)]) -> u32 {
    let mut password: u32 = 0;
    // The current position always starts at 50
    let mut current_position: i32 = 50;

    for rotation in rotations {
        current_position = apply_rotation_to_dial(current_position, &rotation);
        if current_position == 0 {
            password += 1
        }
    }

    password
}

pub fn apply_rotation_to_dial(current_position: i32, rotation: &(&str, i32)) -> i32 {
    let mut rotation_number: i32 = rotation.1;
    if rotation.0 == "L" {
        rotation_number = rotation.1 * -1;
    }

    let applied_rotation: i32 = current_position + rotation_number;
    let mut _final_position: i32 = applied_rotation;

    // NOTE: The assumption is that the number of rotations is between 0-99
    if applied_rotation < 0 {
        _final_position = DIAL_MAX_NUMBER_OF_POSITIONS + 1 + applied_rotation;
    } else if applied_rotation >= DIAL_MAX_NUMBER_OF_POSITIONS {
        _final_position = applied_rotation - DIAL_MAX_NUMBER_OF_POSITIONS - 1;
    } 

    return _final_position;
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
        let current_position= 42;
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
    fn test_find_out_password() {
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
        assert_eq!(find_out_password(&rotations), 3)
    }

    #[test]
    fn test_part1() {
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
        assert_eq!(part1(input), 0);
    }

    #[test]
    fn test_part2_example() {
        let input = "your test input here";
        assert_eq!(part2(input), 0);
    }
}
