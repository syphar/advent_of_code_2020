#![feature(str_split_once)]

// mod rules;
// use rules::Rule;

use regex::Regex;
use simple_error::{bail, SimpleResult};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn resolve_rule_by_number(rules: &HashMap<u16, String>, which: u16) -> SimpleResult<String> {
    if let Some(rule) = rules.get(&which) {
        resolve_rule(rules, &rule)
    } else {
        bail!("unknown rule {}", which)
    }
}

fn resolve_rule(rules: &HashMap<u16, String>, rule: &str) -> SimpleResult<String> {
    if rule.starts_with("\"") && rule.ends_with("\"") {
        // exact character match
        Ok(rule[1..(rule.len() - 1)].to_string())
    } else if let Some((lhs, rhs)) = rule.split_once("|") {
        // or of two sets of rules
        Ok(format!(
            "({}|{})",
            resolve_rule(rules, lhs.trim())?,
            resolve_rule(rules, rhs.trim())?,
        )
        .to_string())
    } else {
        // combine child sets of rules
        let mut new_rule = String::new();
        for rn in rule.split(" ") {
            let rnn = rn.parse::<u16>().unwrap();
            new_rule.extend(resolve_rule_by_number(rules, rnn));
        }

        Ok(new_rule)
    }
}

fn read_pattern(rules: impl Iterator<Item = String>) -> SimpleResult<Regex> {
    let mut r: HashMap<u16, String> = HashMap::new();

    for rule in rules {
        let (nt, rt) = rule.split_once(": ").unwrap();

        r.insert(nt.parse().unwrap(), rt.trim().to_string());
    }

    if let Ok(re) = Regex::new(&format!("^{}$", resolve_rule_by_number(&r, 0)?)) {
        Ok(re)
    } else {
        bail!("invalid regex")
    }
}

fn main() {
    let rule: Regex = read_pattern(
        BufReader::new(File::open("input_rules.txt").unwrap())
            .lines()
            .map(|line| line.unwrap().to_string()),
    )
    .unwrap();

    println!(
        "part 1: {:?}",
        part_1(
            BufReader::new(File::open("input_messages.txt").unwrap())
                .lines()
                .map(|line| line.unwrap().to_string()),
            &rule,
        )
    );
}

fn part_1(messages: impl Iterator<Item = String>, rule: &Regex) -> SimpleResult<usize> {
    Ok(messages.filter(|m| rule.is_match(m)).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rules() -> impl Iterator<Item = String> {
        vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
        ]
        .into_iter()
        .map(|line| line.to_string())
    }

    fn test_messages() -> impl Iterator<Item = String> {
        vec!["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"]
            .into_iter()
            .map(|line| line.to_string())
    }

    #[test]
    fn test_read_pattern() {
        assert_eq!(
            read_pattern(test_rules()).unwrap().as_str(),
            "^a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b$",
        )
    }

    #[test]
    fn part_1_works() {
        let re = read_pattern(test_rules()).unwrap();
        assert_eq!(part_1(test_messages(), &re), Ok(2));
    }
}
