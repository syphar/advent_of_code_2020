#[macro_use]
extern crate itertools;

use simple_error::SimpleError;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<u64> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    if let Ok(invalid_number) = run(&input, 25) {
        println!("part 1: {:?}", invalid_number);
        println!("part 2: {:?}", run2(&input, invalid_number));
    }
}

fn run(input: &Vec<u64>, check_window: usize) -> Result<u64, SimpleError> {
    for slice in input.windows(check_window + 1) {
        let current_value = slice.last().unwrap();
        let check_values = &slice[..check_window];

        let has_combinations = iproduct!(check_values.iter(), check_values.iter())
            .any(|(a, b)| *a + *b == *current_value);

        if !has_combinations {
            return Ok(*current_value);
        }
    }

    Err(SimpleError::new("no wrong number found"))
}

fn run2(input: &Vec<u64>, to_find: u64) -> Result<u64, SimpleError> {
    for window_size in 2..=input.len() {
        for slice in input.windows(window_size) {
            if slice.iter().sum::<u64>() == to_find {
                return Ok(slice.iter().min().unwrap() + slice.iter().max().unwrap());
            }
        }
    }

    Err(SimpleError::new("no combination found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<u64> {
        vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ]
    }

    #[test]
    fn it_works() {
        let td = test_data();
        assert_eq!(run(&td, 5), Ok(127));
    }

    #[test]
    fn it_works2() {
        let td = test_data();
        assert_eq!(run2(&td, 127), Ok(62));
    }
}
