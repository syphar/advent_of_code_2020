#[macro_use]
extern crate itertools;

use simple_error::SimpleError;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit(numbers: Vec<i64>) -> Result<i64, SimpleError> {
    for (i, j) in iproduct!(numbers.iter(), numbers.iter()) {
        if i + j == 2020 {
            return Ok(i * j);
        }
    }
    Err(SimpleError::new("nothing found"))
}

pub fn doit3(numbers: Vec<i64>) -> Result<i64, SimpleError> {
    for (i, j, k) in iproduct!(numbers.iter(), numbers.iter(), numbers.iter()) {
        if i + j + k == 2020 {
            return Ok(i * j * k);
        }
    }
    Err(SimpleError::new("nothing found"))
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let vec: Vec<i64> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!("{:?}", doit3(vec).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            doit(vec![1721i64, 979, 366, 299, 675, 1456]).unwrap(),
            514579
        );
    }

    #[test]
    fn it_works3() {
        assert_eq!(
            doit3(vec![1721i64, 979, 366, 299, 675, 1456]).unwrap(),
            241861950
        );
    }
}
