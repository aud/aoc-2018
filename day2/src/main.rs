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

fn read_file(name: &str) -> Vec<String> {
    let file = File::open(name).unwrap();
    let buffer = BufReader::new(file);

    buffer.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

mod part1 {
    use std::collections::HashMap;

    type VecType = Vec<String>;

    pub fn solution(input: &VecType) -> i32 {
        let mut two = 0;
        let mut three = 0;

        for word in input.iter() {
            let mut freq = HashMap::new();

            for c in word.chars() {
                *freq.entry(c).or_insert(0) += 1;
            }

            if freq.values().any(|f| *f == 3) {
                three += 1
            }

            if freq.values().any(|f| *f == 2) {
                two += 1
            }
        }

        two * three

    }
}

mod part2 {
    type VecType = Vec<String>;

    pub fn solution(input: &VecType) -> String {
        for (idx, id) in input.iter().enumerate() {
            for id2 in input.iter().skip(idx + 1) {
                let mut found = String::new();
                let mut diff = 0;

                for (a, b) in id.chars().zip(id2.chars()) {
                    if a != b {
                        diff +=1;
                        if diff > 1 { break; }
                    } else {
                        found.push(a);
                        continue;
                    }
                }

                if diff == 1 {
                    return found;
                }
            }
        }

        String::new()
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

    fn solve_part2(name: &str) -> String {
        let path = format!("{}example/{}", INPUT_BASE_PATH, name);
        let contents = read_file(&path);

        part2::solution(&contents).to_string()
    }

    #[test]
    fn part1() {
        assert_eq!(12, solve_part1("1.txt"));
    }

    #[test]
    fn part2() {
        assert_eq!("fgij", solve_part2("2.txt"));
    }
}
