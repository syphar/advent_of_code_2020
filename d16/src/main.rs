mod ticket;
use ticket::Field;

use simple_error::SimpleError;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("part 1: {:?}", part_1(&input));
}

fn part_1(lines: &Vec<String>) -> Result<usize, SimpleError> {
    //read field definitions
    let fields: Vec<Field> = lines
        .iter()
        .take_while(|l| !(l.is_empty()))
        .filter_map(|l| l.parse().ok())
        .collect();

    println!("{:?}", fields);

    let mut invalid_values: Vec<usize> = Vec::new();

    // read lines for nearby tickets
    // skip: fields, 2 empty lines, 2 headers, your own ticket
    for line in lines.iter().skip(fields.len() + 2 + 2 + 1) {
        let numbers: Vec<usize> = line.split(",").map(|s| s.parse().unwrap()).collect();
        println!("{:?} => {:?}", line, numbers);

        for number in numbers {
            if !(fields.iter().any(|f| f.check(&number))) {
                invalid_values.push(number);
            }
        }
    }

    Ok(invalid_values.iter().cloned().sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_DATA: Vec<String> = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ]
        .into_iter()
        .map(|line| line.to_string())
        .collect();
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(&TEST_DATA), Ok(71));
    }
}
