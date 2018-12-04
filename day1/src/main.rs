use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_BASE_PATH: &str = "./input/";

fn main() {
    let path = format!("{}{}", INPUT_BASE_PATH, "puzzle.txt");
    let contents = read_file(&path);

    let pt1 = part1::solution(&contents);
    let pt2 = part2::solution(&contents);

    println!("Part 1: {}\n", pt1);
    println!("Part 2: {}\n", pt2);
}

fn read_file(name: &str) -> std::vec::Vec<i32> {
    let file = File::open(name).unwrap();
    let buffer = BufReader::new(file);

    buffer.lines()
        .filter_map(|line| line.ok().and_then(|l| l.parse().ok()))
        .collect()
}

mod part1 {
    type VecType = Vec<i32>;

    pub fn solution(input: &VecType) -> i32 {
        input.iter().sum()
    }
}

mod part2 {
    use std::collections::HashSet;

    type VecType = Vec<i32>;

    pub fn solution(input: &VecType) -> i32 {
        let mut freqs: HashSet<_> = [0].iter()
            .cloned()
            .collect();

        let mut accumulator = 0;

        for freq in input.iter().cycle() {
            accumulator += freq;

            if !freqs.insert(accumulator) {
                return accumulator;
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use read_file;
    use part1;
    use part2;
    use INPUT_BASE_PATH;

    fn solve_part1(name: &str) -> i32 {
        let path = format!("{}example/{}", INPUT_BASE_PATH, name);
        let contents = read_file(&path);

        part1::solution(&contents)
    }

    fn solve_part2(name: &str) -> i32 {
        let path = format!("{}example/{}", INPUT_BASE_PATH, name);
        let contents = read_file(&path);

        part2::solution(&contents)
    }

    #[test]
    fn part1() {
        assert_eq!(3, solve_part1("1.txt"));
        assert_eq!(0, solve_part1("2.txt"));
        assert_eq!(-6, solve_part1("3.txt"));
    }

    #[test]
    fn part2() {
        assert_eq!(0, solve_part2("4.txt"));
        assert_eq!(10, solve_part2("5.txt"));
        assert_eq!(5, solve_part2("6.txt"));
        assert_eq!(14, solve_part2("7.txt"));
    }
}
