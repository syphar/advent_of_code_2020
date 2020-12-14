mod commands;
use commands::Command;

use simple_error::SimpleError;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<Command> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!("part 1: {:?}", part_1(input.iter()));
}

fn part_1<'a>(commands: impl Iterator<Item = &'a Command>) -> Result<u64, SimpleError> {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut current_mask: HashMap<u8, u8> = HashMap::new();

    for command in commands {
        match command {
            Command::Mask(mask) => {
                current_mask = mask
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.is_some())
                    .map(|(i, v)| (i as u8, if v.unwrap() == true { 1 } else { 0 }))
                    .collect();
            }
            Command::Set(adr, v) => {
                let mut value = *v;

                for (bit, bitvalue) in current_mask.iter() {
                    match *bitvalue {
                        1 => {
                            // set a bit
                            let mask = 1 << bit;
                            value |= mask;
                        }
                        0 => {
                            // remove a bit
                            let mask = !(1 << bit);
                            value &= mask;
                        }
                        _ => {
                            return Err(SimpleError::new("unknown bit value"));
                        }
                    }
                }
                memory.insert(*adr, value);
            }
        }
    }

    Ok(memory.values().filter(|&v| *v > 0).sum())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_DATA: Vec<Command> = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]
        .into_iter()
        .map(|line| line.to_string().parse().unwrap())
        .collect();
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_DATA.iter()), Ok(165));
    }
}
