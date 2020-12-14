mod commands;
use commands::{BitMask, Command};

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
    println!("part 2: {:?}", part_2(input.iter()));
}

fn part_1<'a>(commands: impl Iterator<Item = &'a Command>) -> Result<u64, SimpleError> {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut current_mask = BitMask::new();

    for command in commands {
        match command {
            Command::SetMask(mask) => current_mask = mask.iter().cloned().collect(),
            Command::SetValue(adr, v) => {
                let mut value = *v;

                for (bit, onoff) in current_mask.iter().enumerate() {
                    match onoff {
                        Some(true) => {
                            // set a bit
                            let mask = 1 << bit;
                            value |= mask;
                        }
                        Some(false) => {
                            // remove a bit
                            let mask = !(1 << bit);
                            value &= mask;
                        }
                        None => {}
                    }
                }
                memory.insert(*adr, value);
            }
        }
    }

    Ok(memory.values().filter(|&v| *v > 0).sum())
}

fn part_2<'a>(commands: impl Iterator<Item = &'a Command>) -> Result<u64, SimpleError> {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut current_mask = BitMask::new();

    for command in commands {
        match command {
            Command::SetMask(mask) => current_mask = mask.iter().cloned().collect(),
            Command::SetValue(a, value) => {
                let mut adresses: Vec<u64> = vec![*a];

                for (bit, onoff) in current_mask.iter().enumerate() {
                    match onoff {
                        Some(true) => {
                            // 1 means set a bit in all addresses
                            let mask = 1 << bit;
                            for i in 0..adresses.len() {
                                adresses[i] |= mask;
                            }
                        }
                        Some(false) => {} // do nothing for 0 bit in mask
                        None => {
                            // X means we need both possible values for
                            // the addresses. So:

                            // first set bit in all addresses,
                            let mask_set = 1 << bit;
                            for i in 0..adresses.len() {
                                adresses[i] |= mask_set;
                            }

                            // then duplicate with removed bit
                            let mask_del = !mask_set;
                            let new_adresses: Vec<u64> =
                                adresses.iter().map(|v| v & mask_del).collect();
                            adresses.extend(new_adresses);
                        }
                    }
                }

                memory.extend(adresses.iter().map(|a| (*a, *value)));
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
        static ref TEST_DATA_2: Vec<Command> = vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ]
        .into_iter()
        .map(|line| line.to_string().parse().unwrap())
        .collect();
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_DATA.iter()), Ok(165));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_DATA_2.iter()), Ok(208));
    }
}
