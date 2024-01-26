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

pub mod day3 {
    use std::collections::HashSet;

    #[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
    struct Coordinate (i32,i32);

    impl Coordinate {
        fn right(&self) -> Coordinate {
            Coordinate(self.0 + 1, self.1)
        }

        fn left(&self) -> Coordinate {
            Coordinate(self.0 - 1, self.1)
        }

        fn up(&self) -> Coordinate {
            Coordinate(self.0, self.1 + 1)
        }

        fn down(&self) -> Coordinate {
            Coordinate(self.0, self.1 - 1)
        }
    }

    trait IndexParity {
        fn evens(&self) -> String;
        fn odds(&self) -> String;
    }

    impl IndexParity for String {
        fn evens(&self) -> String {
            self.chars().enumerate().filter(|&ele| ele.0 % 2 == 0).map(|ele| ele.1).collect()
        }

        fn odds(&self) -> String {
            self.chars().enumerate().filter(|&ele| ele.0 % 2 == 1).map(|ele| ele.1).collect()
        }
    }

    pub fn part_one(input: &str) -> usize {
        visited_coordinates(input).len()
    }

    pub fn part_two(input :&str) -> usize {
        let visited_by_santa = visited_coordinates(&input.to_string().clone().evens());
        let visited_by_robot = visited_coordinates(&input.to_string().clone().odds());

        visited_by_santa
            .union(&visited_by_robot)
            .collect::<HashSet<&Coordinate>>()
            .len()
    }

    fn visited_coordinates(input: &str) -> HashSet<Coordinate> {
        let mut current: Coordinate = Coordinate(0,0);
        let mut visited_cor: HashSet<Coordinate> = HashSet::from([current]);
        input.chars().for_each(|c| {
            current = next_coordinate(current, c);
            visited_cor.insert(current);
        });
        visited_cor
    }

    fn next_coordinate(current: Coordinate, step: char) -> Coordinate {
        match step {
            '>' => current.right(),
            '<' => current.left(),
            '^' => current.up(),
            'v' | 'V' => current.down(),
            _ => panic!("invalid input")
        }
    }
}

pub mod day4 {
    use md5::compute;

    pub fn part_one(input: &str) -> u32 {
        unhash_for_prefix(input, "00000")
    }

    pub fn part_two(input: &str) -> u32 {
        unhash_for_prefix(input, "000000")
    }

    fn unhash_for_prefix(input: &str, prefix: &str) -> u32 {
        let mut num = 1;
        loop {
            if format!("{:x}", compute(format!("{}{}", input, num))).starts_with(prefix) {
                break num
            } else {
                num += 1;
            }
        }
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

    mod day3 {
        use crate::y2015::day3::*;

        #[test]
        fn test_part_one() {
            assert_eq!(part_one(">"), 2);
            assert_eq!(part_one("^>v<"), 4);
            assert_eq!(part_one("^v^v^v^v^v"), 2);
        }

        #[test]
        fn test_part_two() {
            assert_eq!(part_two("^v"), 3);
            assert_eq!(part_two("^>v<"), 3);
            assert_eq!(part_two("^v^v^v^v^v"), 11);
        }
    }

    mod day4 {
        use crate::y2015::day4::*;

        #[test]
        fn test_part_one() {
            assert_eq!(part_one("abcdef"), 609043);
            assert_eq!(part_one("pqrstuv"), 1048970);
        }

        #[test]
        fn test_part_two() {
        }
    }
}