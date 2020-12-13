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

    let time: u64 = input[0].parse().unwrap();

    let lines: Vec<u16> = input[1].split(",").filter_map(|s| s.parse().ok()).collect();

    println!("part1: {:?}", run(time, &lines));

    let lines2: Vec<&str> = input[1].split(",").collect();
    println!("part2: {:?}", run_2(&lines2));
}

fn run(time: u64, buses: &[u16]) -> u64 {
    let mut t = time;

    loop {
        for b in buses.iter().filter(|&b| t % ((*b) as u64) == 0) {
            return (t - time) * (*b as u64);
        }
        t += 1;
    }
}

fn chinese_remainder(input: &HashMap<i64, i64>) -> Option<i64> {
    fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
        if a == 0 {
            (b, 0, 1)
        } else {
            let (g, x, y) = egcd(b % a, a);
            (g, y - (b / a) * x, x)
        }
    }

    fn mod_inv(x: i64, n: i64) -> Option<i64> {
        let (g, x, _) = egcd(x, n);
        if g == 1 {
            Some((x % n + n) % n)
        } else {
            None
        }
    }

    // https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
    let prod = input.values().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in input.iter() {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn run_2(input: &[&str]) -> Result<i64, SimpleError> {
    let parsed_input: HashMap<i64, i64> = input
        .iter()
        .enumerate()
        .map(|(i, v)| (i, v.parse().ok()))
        .filter(|(_, v)| v.is_some())
        .map(|(i, v)| (i as i64, v.unwrap()))
        .collect();

    if let Some(result) = chinese_remainder(&parsed_input) {
        Ok(parsed_input.values().product::<i64>() - result)
    } else {
        Err(SimpleError::new("no value found"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn it_works() {
        assert_eq!(run(939, &[7, 13, 59, 31, 19]), 295);
    }

    #[test_case(&["17", "x", "13", "19"], 3_417)]
    #[test_case(&["67", "7", "59", "61"], 754_018)]
    #[test_case(&["67", "x", "7", "59", "61"], 779_210)]
    #[test_case(&["7", "13", "x", "x", "59", "x", "31", "19"], 1_068_781)]
    #[test_case(&["67", "7", "x", "59", "61"], 1_261_476)]
    #[test_case(&["1789", "37", "47", "1889"], 1_202_161_486)]
    fn it_works_2(input: &[&str], expected: i64) {
        assert_eq!(run_2(&input), Ok(expected));
    }
}
