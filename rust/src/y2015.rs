pub mod day1 {
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
}

pub mod day2 {
    use std::cmp::min;

    pub fn part_one(input: &str) -> u32 {
        special_sum(input, wrapping_paper)
    }

    pub fn part_two(input: &str) -> u32 {
        special_sum(input, ribbon)
    }

    fn special_sum(input: &str, f: fn(u32, u32, u32) -> u32) -> u32  {
        input.split("|")
            .map(|box_dim| {
                let mut iter = box_dim.split("x")
                    .map(|str_len| str_len.parse::<u32>().unwrap());
                f(iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
            })
            .sum()
    }

    fn wrapping_paper(length: u32, width: u32, height: u32) -> u32 {
        let (x_plane, y_plane, z_plane) = (length * width, length * height, width * height);
        let min_plane = min(min(x_plane, y_plane), z_plane);
        2 * x_plane + 2 * y_plane + 2 * z_plane + min_plane
    }

    fn ribbon(length: u32, width: u32, height: u32) -> u32 {
        let (cir_xy, cir_yz, cir_xz) = (2 * length + 2 * width, 2 * width + 2 * height, 2 * length + 2 * height);
        min(min(cir_xy, cir_yz), cir_xz) + length * width * height
    }
}

#[cfg(test)]
mod tests {
    mod day1 {
        use crate::y2015::day1::*;

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

    mod day2 {
        use crate::y2015::day2::*;

        #[test]
        fn test_part_one() {
            assert_eq!(part_one("2x3x4"), 58);
            assert_eq!(part_one("1x1x10"), 43);
        }

        #[test]
        fn test_part_two() {
            assert_eq!(part_two("2x3x4"), 34);
            assert_eq!(part_two("1x1x10"), 14);
        }
    }
}