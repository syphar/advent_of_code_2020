use recap::Recap;
use serde::Deserialize;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Deserialize, Recap)]
#[recap(regex = r"(?P<min>\d+)\-(?P<max>\d+) (?P<letter>.): (?P<password>.*)")]
pub struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

pub fn doit(passwords: impl Iterator<Item = Password>) -> usize {
    passwords
        .map(|p| (p.min, p.max, p.password.matches(p.letter).count()))
        .filter(|(min, max, count)| min <= count && count <= max)
        .count()
}

fn main() {
    let file = File::open("input.txt").unwrap();

    println!(
        "{:?}",
        doit(
            BufReader::new(file)
                .lines()
                .map(|line| line.unwrap().parse().unwrap())
        )
    );
}

pub fn doit2(passwords: impl Iterator<Item = Password>) -> usize {
    passwords
        .filter(|p| {
            (p.password.chars().nth(p.min - 1).unwrap() == p.letter)
                ^ (p.password.chars().nth(p.max - 1).unwrap() == p.letter)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = Password> {
        vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"]
            .into_iter()
            .map(|line| line.parse().unwrap())
    }

    #[test]
    fn it_works() {
        assert_eq!(doit(test_data()), 2);
    }

    #[test]
    fn it_works2() {
        assert_eq!(doit2(test_data()), 1);
    }
}
