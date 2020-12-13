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

fn run_2(input: &[&str]) -> u64 {
    let mut t: u64 = 0;

    loop {
        if input
            .iter()
            .enumerate()
            .filter(|(_, &b)| b != "x")
            .map(|(i, b)| (i as u64, b.parse::<u64>().unwrap()))
            .all(|(i, b)| (t + i) % b == 0)
        {
            return t;
        }

        t += 1;
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

    #[test_case(&["17", "x", "13", "19"], 3417)]
    #[test_case(&["67", "7", "59", "61"], 754018)]
    #[test_case(&["67", "x", "7", "59", "61"], 779210)]
    #[test_case(&["7", "13", "x", "x", "59", "x", "31", "19"], 1068781)]
    #[test_case(&["67", "7", "x", "59", "61"], 1261476)]
    #[test_case(&["1789", "37", "47", "1889"], 1202161486)]
    fn it_works_2(input: &[&str], expected: u64) {
        assert_eq!(run_2(&input), expected);
    }
}
