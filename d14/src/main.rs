#[macro_use]
extern crate lazy_static;

use regex::Regex;
use simple_error::SimpleError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("part 1: {:?}", part_1(input.iter()));
}

lazy_static! {
    static ref REGEX_MASK: Regex = Regex::new(r"mask = (\w{36})").unwrap();
    static ref REGEX_MEM: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
}

fn part_1<'a>(lines: impl Iterator<Item = &'a String>) -> Result<u64, SimpleError> {
    let mut memory: HashMap<usize, u64> = HashMap::new();

    let mut current_mask: HashMap<u8, u8> = HashMap::new();

    for line in lines {
        println!("\nline {:?}", line);

        if let Some(cap) = REGEX_MASK.captures(line) {
            let mask = cap.get(1).unwrap().as_str();

            current_mask = mask
                .chars()
                .rev()
                .enumerate()
                .filter(|(_, v)| *v != 'X')
                .map(|(i, v)| (i as u8, if v == '1' { 1 } else { 0 }))
                .collect();

            println!("\tis mask: {}", mask);
            println!("\t{:?}", current_mask);
        } else if let Some(cap) = REGEX_MEM.captures(line) {
            let adr: usize = cap.get(1).unwrap().as_str().parse().unwrap();
            let mut value: u64 = cap.get(2).unwrap().as_str().parse().unwrap();

            println!("\tis mem: {} = {}", adr, value);

            println!("\tbits before: {:#036b} (decimal {})", value, value);

            for (bit, bitvalue) in current_mask.iter() {
                match *bitvalue {
                    1 => {
                        // set a bit
                        let mask = 1 << bit;
                        println!("\t\t set bit {}, mask {:#036b}", bit, mask);
                        value |= mask;
                    }
                    0 => {
                        // remove a bit
                        let mask = !(1 << bit);
                        println!("\t\t remove bit {}, mask {:#036b}", bit, mask);
                        value &= mask;
                    }
                    _ => {
                        return Err(SimpleError::new("unknown bit value"));
                    }
                }
            }

            println!("\tbits after: {:#036b} (decimal {})", value, value);

            memory.insert(adr, value);
        } else {
            return Err(SimpleError::new(format!("unknown line: {}", line)));
        }
    }

    Ok(memory.values().filter(|&v| *v > 0).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_DATA: Vec<String> = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]
        .into_iter()
        .map(|line| line.to_string())
        .collect();
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_DATA.iter()), Ok(165));
    }
}
