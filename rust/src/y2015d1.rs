pub fn part_one(input: &str) -> i32 {
    input.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    let mut negative_index: usize = 0;
    let mut sum: i32 = 0;
    for (i, c) in input.chars().enumerate() {
        sum += match c {
            '(' => 1,
            ')' => -1,
            _ => 0
        };
        if sum < 0 {
            negative_index = i + 1;
            break;
        }
    }
    negative_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("(())"), 0);
        assert_eq!(part_one("()()"), 0);
        assert_eq!(part_one("((("), 3);
        assert_eq!(part_one("(()(()("), 3);
        assert_eq!(part_one("))((((("), 3);
        assert_eq!(part_one("())"), -1);
        assert_eq!(part_one("))("), -1);
        assert_eq!(part_one(")))"), -3);
        assert_eq!(part_one(")())())"), -3);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(")"), 1);
        assert_eq!(part_two("()())"), 5);
    }
}