use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    println!(
        "{:?}",
        doit2(
            BufReader::new(file).lines().map(|line| line.unwrap()),
            "shiny gold".to_string()
        )
    );
}

lazy_static! {
    static ref REGEX_CONTENT: Regex = Regex::new(r"(\d+) (\w+ \w+) (bag|bags)\.?").unwrap();
}

fn read_mapping(lines: impl Iterator<Item = String>) -> HashMap<String, HashMap<String, usize>> {
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
    search_for: String,
) {
    let contains_color: HashSet<String> = mapping
        .iter()
        .filter(|(_, v)| v.contains_key(&search_for))
        .map(|(k, _)| k.clone())
        .collect();

    result.extend(contains_color.clone());

    if !(contains_color.is_empty()) {
        for parent_color in contains_color.iter() {
            search_for_colors(mapping, result, parent_color.clone());
        }
    }
}

pub fn doit(lines: impl Iterator<Item = String>, search_for_color: String) -> usize {
    let mapping = read_mapping(lines);

    let mut result: HashSet<String> = HashSet::new();
    search_for_colors(&mapping, &mut result, search_for_color);
    result.len()
}

fn count_bags(mapping: &HashMap<String, HashMap<String, usize>>, search_for: String) -> usize {
    match mapping.get(&search_for) {
        None => 0,
        Some(children) => {
            1 + children
                .iter()
                .map(|(color, count)| count * count_bags(mapping, color.clone()))
                .sum::<usize>()
        }
    }
}

pub fn doit2(lines: impl Iterator<Item = String>, search_for_color: String) -> usize {
    let mapping = read_mapping(lines);

    count_bags(&mapping, search_for_color) - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        vec![
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
    }

    fn test_data2() -> impl Iterator<Item = String> {
        vec![
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
    }

    #[test]
    fn it_works() {
        assert_eq!(doit(test_data(), "shiny gold".to_string()), 4);
    }

    #[test]
    fn it_works2() {
        assert_eq!(doit2(test_data(), "shiny gold".to_string()), 32);
    }

    #[test]
    fn it_works3() {
        assert_eq!(doit2(test_data2(), "shiny gold".to_string()), 126);
    }
}
