#[macro_use]
extern crate lazy_static;

use counter::Counter;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

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

fn find_combinations(values: &HashSet<u16>, start_at: u16, end_at: u16) -> usize {
    let mut count = 0;
    for step in 1..=3 {
        let test = start_at + step;
        if test == end_at {
            count = 1;
        } else if values.contains(&test) {
            count += find_combinations(&values, test, end_at);
        }
    }
    count
}

fn run2(input: &Vec<u16>) -> usize {
    let values = HashSet::from_iter(input.iter().cloned());
    let end_at = values.iter().max().unwrap() + 3;

    find_combinations(&values, 0, end_at)
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
