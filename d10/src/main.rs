#[macro_use]
extern crate lazy_static;

use counter::Counter;
use simple_error::SimpleError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<u16> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!("part 1: {:?}", run(&input));
    //     println!("part 2: {:?}", run2(&input, invalid_number));
    // }
}

fn run(input: &Vec<u16>) -> usize {
    let mut sorted = input.clone();
    sorted.sort();

    // start at 0
    sorted.insert(0, 0);

    // builtin adapter
    sorted.push(sorted.iter().max().unwrap() + 3);

    let mut counter: Counter<u16> = Counter::new();
    for slice in sorted.windows(2) {
        let diff = slice[1] - slice[0];
        counter[&diff] += 1;
    }

    counter.get(&1).unwrap_or(&0) * counter.get(&3).unwrap_or(&0)
}

// fn run2(input: &Vec<u64>, to_find: u64) -> Result<u64, SimpleError> {
//     for window_size in 2..=input.len() {
//         for slice in input.windows(window_size) {
//             if slice.iter().sum::<u64>() == to_find {
//                 return Ok(slice.iter().min().unwrap() + slice.iter().max().unwrap());
//             }
//         }
//     }

//     Err(SimpleError::new("no combination found"))
// }

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
    fn part_1_works_works2() {
        assert_eq!(run(&TEST_DATA_2), 220);
    }
}
