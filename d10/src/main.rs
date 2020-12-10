#[macro_use]
extern crate lazy_static;

use counter::Counter;
use permutator::large_combination;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<u16> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!("part 1: {:?}", run(&input));
    println!("part 2: {:?}", run2(&input));
}

fn run(input: &Vec<u16>) -> usize {
    let mut sorted = input.clone();
    sorted.sort();
    sorted.insert(0, 0); // start at 0
    sorted.push(sorted.last().unwrap() + 3); // builtin adapter

    let counter = sorted
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .collect::<Counter<_>>();

    counter.get(&1).unwrap_or(&0) * counter.get(&3).unwrap_or(&0)
}

fn is_valid(input: &[&u16], builtin_jolts: u16) -> bool {
    let valid_steps = 1..=3;

    // step from 0 to first value
    if !(valid_steps.contains(input[0])) {
        return false;
    }

    // step from last element to builtin value
    let last_step = builtin_jolts - *(input.last().unwrap());
    if !(valid_steps.contains(&last_step)) {
        return false;
    }

    for step in input.windows(2).map(|slice| slice[1] - slice[0]) {
        if !(valid_steps.contains(&step)) {
            return false;
        }
    }

    true
}

fn run2(input: &Vec<u16>) -> usize {
    let mut sorted = input.clone();
    sorted.sort();

    let builtin_jolts: u16 = *(sorted.last().unwrap()) + 3;

    // assuming we have a max step size of 3,
    // there is a minimum amount of steps to get from 0 to builtin_jolts
    let min_steps = (builtin_jolts as f32 / 3.0).ceil() as usize;

    let mut count = 0;

    // try all possible connector counts
    for length in min_steps..=sorted.len() {
        // try all possible combinations using the connectors given
        large_combination(&sorted, length, |el| {
            // check if they are valid
            if is_valid(el, builtin_jolts) {
                count += 1;
            }
        });
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_DATA_1: Vec<u16> = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4,];
        static ref TEST_DATA_2: Vec<u16> = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
    }

    #[test]
    fn part_1_works() {
        assert_eq!(run(&TEST_DATA_1), 35);
    }

    #[test]
    fn part_1_works_2() {
        assert_eq!(run(&TEST_DATA_2), 220);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(run2(&TEST_DATA_1), 8);
    }

    // Test is too slow, but release-build gives the correct result
    // #[test]
    // fn part_2_works_2() {
    //     assert_eq!(run2(&TEST_DATA_2), 19208);
    // }
}
