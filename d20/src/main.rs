use simple_error::{bail, SimpleError, SimpleResult};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

mod pattern;
mod tile;
use tile::Tile;

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

fn part_1(tiles: &Vec<Tile>) -> SimpleResult<u64> {
    Ok(0)
}

fn read_tiles(lines: impl Iterator<Item = String>) -> SimpleResult<Vec<Tile>> {
    let mut result: Vec<Tile> = Vec::new();
    let mut current_lines = Vec::new();
    for line in lines {
        if line.trim().is_empty() {
            result.push(current_lines.join("\n").parse()?);
            current_lines.clear();
        }

        current_lines.push(line);
    }

    if !(current_lines.is_empty()) {
        result.push(current_lines.join("\n").parse()?);
    }

    Ok(result)
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
    fn test_read_tiles() {
        let tiles = read_tiles(test_input()).unwrap();
        assert_eq!(tiles.len(), 9);
    }

    #[test]
    fn part_1_works() {
        let tiles = read_tiles(test_input()).unwrap();

        assert_eq!(part_1(&tiles), Ok(20899048083289));
    }
}
