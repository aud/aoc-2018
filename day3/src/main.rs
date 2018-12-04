use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate regex;

#[macro_use]
extern crate lazy_static;

const INPUT_BASE_PATH: &str = "./input/";

fn main() {
    let path = format!("{}{}", INPUT_BASE_PATH, "puzzle.txt");
    let contents = read_file(&path);

    let pt1 = part1::solution(&contents);
    let pt2 = part2::solution(&contents);

    println!("Part 1: {}\n", pt1);
    println!("Part 2: {}\n", pt2);
}

fn read_file(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap();
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

mod claim {
    impl Claim {
        pub fn rects(&self) -> impl Iterator<Item = (i16, i16)> + '_ {
            (self.left..self.left + self.width).flat_map(move |width| {
                (self.top..self.top + self.height).map(move |height| (width, height))
            })
        }
    }

    #[derive(Debug)]
    pub struct Claim {
        pub id: i16,
        pub height: i16,
        pub width: i16,
        pub top: i16,
        pub left: i16,
    }

    pub fn parse_to_int(string: &str) -> i16 {
        string.parse::<i16>().unwrap()
    }
}

mod part1 {
    use std::collections::HashMap;
    use regex::Regex;
    use claim;

    type VecType = Vec<String>;

    pub fn solution(input: &VecType) -> usize {
        lazy_static! {
            static ref re: Regex = Regex::new(r"\#([0-9]+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }

        let mut fabric = HashMap::new();

        for i in input {
            for cap in re.captures_iter(i) {
                let claim: claim::Claim = claim::Claim {
                    id: claim::parse_to_int(&cap[1]),
                    left: claim::parse_to_int(&cap[2]),
                    top: claim::parse_to_int(&cap[3]),
                    width: claim::parse_to_int(&cap[4]),
                    height: claim::parse_to_int(&cap[5]),
                };

                for rect in claim.rects() {
                    *fabric.entry(rect).or_insert(0) += 1;
                }
            }
        }

        fabric.values().filter(|n| **n >= 2).count()
    }
}

mod part2 {
    use std::collections::HashMap;
    use regex::Regex;
    use claim;

    type VecType = Vec<String>;

    pub fn solution(input: &VecType) -> i16 {
        lazy_static! {
            static ref re: Regex = Regex::new(r"\#([0-9]+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
        }

        let mut stack: Vec<claim::Claim> = Vec::new();
        let mut fabric = HashMap::new();

        for i in input {
            for cap in re.captures_iter(i) {
                let claim: claim::Claim = claim::Claim {
                    id: claim::parse_to_int(&cap[1]),
                    left: claim::parse_to_int(&cap[2]),
                    top: claim::parse_to_int(&cap[3]),
                    width: claim::parse_to_int(&cap[4]),
                    height: claim::parse_to_int(&cap[5]),
                };

                for rect in claim.rects() {
                    *fabric.entry(rect).or_insert(0) += 1;
                }

                stack.push(claim);
            }
        }

        let opt = stack.iter().find(
            |claim| claim.rects().all(|i| fabric[&i] == 1),
        );

        return match opt {
            Some(name) => name.id,
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use read_file;
    use INPUT_BASE_PATH;
    use part1;
    use part2;

    fn solve_part1(name: &str) -> usize {
        let path = format!("{}example/{}", INPUT_BASE_PATH, name);
        let contents = read_file(&path);

        part1::solution(&contents)
    }

    fn solve_part2(name: &str) -> i16 {
        let path = format!("{}example/{}", INPUT_BASE_PATH, name);
        let contents = read_file(&path);

        part2::solution(&contents)
    }

    #[test]
    fn part1() {
        assert_eq!(4, solve_part1("1.txt"));
    }

    #[test]
    fn part2() {
        assert_eq!(3, solve_part2("1.txt"));
    }
}
