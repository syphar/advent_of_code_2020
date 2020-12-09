use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    println!(
        "{:?}",
        // doit(BufReader::new(file).lines().map(|line| line.unwrap()))
        doit2(BufReader::new(file).lines().map(|line| line.unwrap()))
    );
}

fn questions_answered_by_everyone(all_answers: &Vec<HashSet<char>>) -> usize {
    let mut all_answered: HashSet<char> = HashSet::with_capacity(30);

    let mut iter = all_answers.iter();

    if let Some(answers) = iter.next() {
        all_answered.extend(answers);
    }

    for questions in iter {
        all_answered = all_answered.intersection(&questions).map(|c| *c).collect();
    }

    all_answered.len()
}

pub fn doit2(lines: impl Iterator<Item = String>) -> usize {
    let mut count = 0;

    let mut current_group_answers: Vec<HashSet<char>> = Vec::with_capacity(10);

    for line in lines {
        if line.trim().len() == 0 {
            count += questions_answered_by_everyone(&current_group_answers);
            current_group_answers.clear();
        } else {
            current_group_answers.push(line.chars().collect());
        }
    }

    if !(current_group_answers.is_empty()) {
        count += questions_answered_by_everyone(&current_group_answers);
    }

    count
}

pub fn doit(lines: impl Iterator<Item = String>) -> usize {
    let mut count = 0;

    let mut current_group_answers: HashSet<char> = HashSet::with_capacity(30);

    for line in lines {
        if line.trim().len() == 0 {
            count += current_group_answers.len();
            current_group_answers.clear();
        } else {
            current_group_answers.extend(line.chars());
        }
    }

    if !(current_group_answers.is_empty()) {
        count += current_group_answers.len();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        vec![
            "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
        ]
        .into_iter()
        .map(|line| line.to_string())
    }

    #[test]
    fn it_works() {
        assert_eq!(doit(test_data()), 11);
    }

    #[test]
    fn it_works2() {
        assert_eq!(doit2(test_data()), 6);
    }
}
