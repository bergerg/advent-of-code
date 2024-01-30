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

pub mod day5 {

    pub trait NiceString {
        fn is_nice(&self) -> bool;
        fn is_nicer(&self) -> bool;
    }

    impl NiceString for String {
        fn is_nice(&self) -> bool {

            fn check_vows(input: &str) -> bool {
                input.chars()
                    .into_iter()
                    .filter(|c| ['a', 'e', 'i', 'o', 'u'].contains(c))
                    .count() >= 3
            }

            fn check_row(input: &str) -> bool {
                let char_vec: Vec<char> = input.chars().into_iter().collect();
                for i in 0..(input.len() - 1) {
                    if char_vec.get(i) == char_vec.get(i + 1) {
                        return true
                    }
                }
                false
            }

            fn check_no_dislikes(input: &str) -> bool {
                for disliked_sequence in ["ab", "cd", "pq", "xy"] {
                    if input.contains(disliked_sequence) {
                        return false
                    }
                }
                true
            }


            check_vows(self) && check_row(self) && check_no_dislikes(self)
        }

        fn is_nicer(&self) -> bool {

            fn check_twice_but_not_overlapping(input: &str) -> bool {
                for i in 0..(input.len() - 1) {
                    if String::from(&input[i+2..]).contains(&input[i..i+2]) {
                        return true
                    }
                }
                false
            }

            fn check_sandwich_sequence(input: &str) -> bool {
                let char_vec: Vec<char> = input.chars().into_iter().collect();
                for i in 0..(input.len() - 2) {
                    if char_vec.get(i) == char_vec.get(i + 2) {
                        return true
                    }
                }
                false
            }

            check_twice_but_not_overlapping(self) && check_sandwich_sequence(self)
        }
    }

    pub fn part_one(input: &str) -> usize {
        input.split("|")
            .filter(|&s| s.to_string().is_nice())
            .count()
    }

    pub fn part_two(input: &str) -> usize {
        input.split("|")
            .filter(|&s| s.to_string().is_nicer())
            .count()
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

    mod day5 {
        use crate::y2015::day5::*;

        #[test]
        fn test_part_one() {
            assert_eq!(1, part_one("ugknbfddgicrmopn"));
            assert_eq!(1, part_one("aaa"));
            assert_eq!(0, part_one("aaab"));
            assert_eq!(0, part_one("jchzalrnumimnmhp"));
            assert_eq!(0, part_one("haegwjzuvuyypxyu"));
            assert_eq!(0, part_one("dvszwmarrgswjxmb"));
        }

        #[test]
        fn test_part_two() {
            assert_eq!(1, part_two("xyxy"));
            assert_eq!(1, part_two("aabedefgaa"));
            assert_eq!(1, part_two("qjhvhtzxzqqjkmpb"));
            assert_eq!(1, part_two("xxyxx"));
            assert_eq!(0, part_two("aaa"));
            assert_eq!(0, part_two("uurcxstgmygtbstg"));
            assert_eq!(0, part_two("ieodomkazucvgmuy"));
        }
    }
}