use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    println!(
        "{:?}",
        // doit(BufReader::new(file).lines().map(|line| line.unwrap()), 3, 1)
        doit2(BufReader::new(file).lines().map(|line| line.unwrap()))
    );
}

pub fn doit(lines: impl Iterator<Item = String>, right: usize, down: usize) -> usize {
    let mut trees = 0;

    let mut x = 0;
    let mut iter = lines.skip(down);

    while let Some(line) = iter.next() {
        for _n in 1..down {
            // skip additional lines if down > 1
            iter.next();
        }

        let line_size = line.len();

        x += right;
        if x >= line_size {
            x -= line_size;
        }

        if let Some(ch) = line.chars().nth(x) {
            if ch == '#' {
                trees += 1;
            }
        }
    }

    trees
}

pub fn doit2(lines: impl Iterator<Item = String>) -> usize {
    let data: Vec<String> = lines.collect();

    return vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(x, y)| doit(data.clone().into_iter(), x, y))
        .fold(1, |acc, x| acc * x);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        vec![
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .into_iter()
        .map(|line| line.to_string())
    }

    #[test]
    fn it_works() {
        assert_eq!(doit(test_data(), 3, 1), 7);
    }

    #[test]
    fn it_works2() {
        assert_eq!(doit2(test_data()), 336);
    }
}
