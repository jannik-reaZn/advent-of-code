pub fn part1(input: &str) -> i32 {
    0
}

pub fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

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
