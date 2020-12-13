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
    println!("part2: {:?}", run_2(&lines2, 100000000000000));
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

fn run_2(input: &[&str], start_at: u64) -> u64 {
    let parsed_input: Vec<Option<u64>> = input.iter().map(|v| v.parse().ok()).collect();

    // find biggest number and its index
    let (biggest_bus_number_index, biggest_bus_number): (i64, i64) = parsed_input
        .iter()
        .enumerate()
        .filter(|(_, &b)| b.is_some())
        .map(|(i, &b)| (i as i64, b.unwrap() as i64))
        .max_by_key(|(_, b)| *b)
        .unwrap();

    // start at a multiple of biggest number
    let mut time: i64 =
        (start_at as i64 + biggest_bus_number) - (start_at as i64 % biggest_bus_number);

    // prepare list of things to check
    let checks: Vec<(i64, i64)> = parsed_input
        .iter()
        .enumerate()
        .filter(|(_, &b)| b.is_some())
        .map(|(i, b)| {
            (
                // make check index relative to the biggest number
                i as i64 - biggest_bus_number_index as i64,
                b.unwrap() as i64,
            )
        })
        .collect();

    loop {
        if checks
            .iter()
            .all(|(relative_index, bus)| (time + relative_index) % *bus == 0)
        {
            return (time - biggest_bus_number_index) as u64;
        }

        time += biggest_bus_number;
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
    fn it_works_2(input: &[&str], expected: u64) {
        assert_eq!(run_2(&input, 100), expected);
        assert_eq!(run_2(&input, 10), expected);
        assert_eq!(run_2(&input, 0), expected);
    }
}
