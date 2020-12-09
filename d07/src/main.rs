#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("part 1: {:?}", part_1(input.iter(), "shiny gold"));
    println!("part 2: {:?}", part_2(input.iter(), "shiny gold"));
}

lazy_static! {
    static ref REGEX_CONTENT: Regex = Regex::new(r"(\d+) (\w+ \w+) (bag|bags)\.?").unwrap();
}

fn read_mapping<'a>(
    lines: impl Iterator<Item = &'a String>,
) -> HashMap<String, HashMap<String, usize>> {
    let mut mapping: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for line in lines {
        let content: Vec<&str> = line.split(" bags contain ").collect();
        if content.len() < 2 {
            continue;
        }

        mapping.insert(
            content[0].to_owned(),
            content[1]
                .split(", ")
                .filter_map(|s| REGEX_CONTENT.captures(s))
                .map(|cap| (cap[2].parse().unwrap(), cap[1].parse().unwrap()))
                .collect(),
        );
    }

    mapping
}

fn search_for_colors(
    mapping: &HashMap<String, HashMap<String, usize>>,
    result: &mut HashSet<String>,
    search_for: &str,
) {
    let contains_color: HashSet<String> = mapping
        .iter()
        .filter(|(_, v)| v.contains_key(search_for))
        .map(|(k, _)| k.clone())
        .collect();

    result.extend(contains_color.clone());

    if !(contains_color.is_empty()) {
        for parent_color in contains_color.iter() {
            search_for_colors(mapping, result, parent_color);
        }
    }
}

fn part_1<'a>(lines: impl Iterator<Item = &'a String>, search_for_color: &str) -> usize {
    let mapping = read_mapping(lines);

    let mut result: HashSet<String> = HashSet::new();
    search_for_colors(&mapping, &mut result, search_for_color);
    result.len()
}

fn count_bags(mapping: &HashMap<String, HashMap<String, usize>>, search_for: &str) -> usize {
    match mapping.get(search_for) {
        None => 0,
        Some(children) => {
            1 + children
                .iter()
                .map(|(color, count)| count * count_bags(mapping, color))
                .sum::<usize>()
        }
    }
}

fn part_2<'a>(lines: impl Iterator<Item = &'a String>, search_for_color: &str) -> usize {
    let mapping = read_mapping(lines);

    count_bags(&mapping, search_for_color) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_DATA: Vec<String> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ]
        .into_iter()
        .map(|line| line.to_string())
        .collect();
        static ref TEST_DATA_2: Vec<String> = vec![
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ]
        .into_iter()
        .map(|line| line.to_string())
        .collect();
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_DATA.iter(), "shiny gold"), 4);
    }

    #[test]
    fn part_2_works_1() {
        assert_eq!(part_2(TEST_DATA.iter(), "shiny gold"), 32);
    }

    #[test]
    fn part_2_works_2() {
        assert_eq!(part_2(TEST_DATA_2.iter(), "shiny gold"), 126);
    }
}
