#[macro_use]
extern crate itertools;
#[macro_use]
extern crate lazy_static;

use simple_error::SimpleError;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_1(numbers: &Vec<i64>) -> Result<i64, SimpleError> {
    for (i, j) in iproduct!(numbers.iter(), numbers.iter()) {
        if i + j == 2020 {
            return Ok(i * j);
        }
    }
    Err(SimpleError::new("nothing found"))
}

fn part_2(numbers: &Vec<i64>) -> Result<i64, SimpleError> {
    for (i, j, k) in iproduct!(numbers.iter(), numbers.iter(), numbers.iter()) {
        if i + j + k == 2020 {
            return Ok(i * j * k);
        }
    }
    Err(SimpleError::new("nothing found"))
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let numbers: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!("part 1: {:?}", part_1(&numbers));
    println!("part 2: {:?}", part_2(&numbers));
}

#[cfg(test)]
mod tests {
    use super::*;
    lazy_static! {
        static ref TEST_DATA: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(&TEST_DATA), Ok(514579));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(&TEST_DATA), Ok(241861950));
    }
}
