use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

mod tile;

fn main() {
    // println!(
    //     "part 1: {:?}",
    //     part_1(
    //         BufReader::new(File::open("input_messages.txt").unwrap())
    //             .lines()
    //             .map(|line| line.unwrap().to_string()),
    //         &rule,
    //     )
    // );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> impl Iterator<Item = String> {
        BufReader::new(File::open("input_test.txt").unwrap())
            .lines()
            .map(|line| line.unwrap().to_string())
    }

    #[test]
    fn part_1_works() {
        assert!(true);
    }
}
