#[macro_use]
extern crate lazy_static;

use counter::Counter;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input2.txt").unwrap();

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

fn is_valid(input: &Vec<&u16>, builtin_jolts: u16) -> bool {
    let mut counter = input
        .windows(2)
        .map(|slice| slice[1] - slice[0])
        .collect::<Counter<_>>();

    counter[&input[0]] += 1; // step from 0 to first value

    // step from last element to builting jolts
    let last_step = builtin_jolts - *(input.last().unwrap());
    counter[&last_step] += 1;

    let valid_steps = 1..=3;
    !(counter.keys().any(|k| !valid_steps.contains(k)))
}

fn run2(input: &Vec<u16>) -> usize {
    let mut sorted = input.clone();
    sorted.sort();

    let builtin_jolts: u16 = *(sorted.last().unwrap()) + 3;

    let mut count = 0;

    for length in 1..=sorted.len() {
        // println!("try len {:?}", length);

        for el in sorted.iter().combinations(length) {
            // println!("\n\n{:?}", el);
            // println!("value: {:?}", is_valid(&el, builtin_jolts));
            if is_valid(&el, builtin_jolts) {
                count += 1;
            }
        }
    }
    count

    // sorted.insert(0, 0); // start at 0
    // sorted.push(sorted.last().unwrap() + 3); // builtin adapter

    // println!("{:?}", permutations);

    // assert_eq!(permutations.len(), 720);

    // 0
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

    #[test]
    fn part_2_works_2() {
        assert_eq!(run2(&TEST_DATA_2), 19208);
    }
}
