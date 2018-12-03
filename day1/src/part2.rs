use std::collections::HashSet;

pub fn solution(input: &std::vec::Vec<i32>) -> i32 {
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
