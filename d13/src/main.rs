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
}

fn run(time: u64, buses: &[u16]) -> u64 {
    let mut t = time;

    loop {
        for b in buses.iter().filter(|b| t % ((**b) as u64) == 0) {
            return (t - time) * (*b as u64);
        }
        t += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(run(939, &[7, 13, 59, 31, 19]), 295);
    }
}
