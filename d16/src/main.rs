mod ticket;
use ticket::Field;

use simple_error::SimpleError;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("part 1: {:?}", part_1(&input));
    println!("part 2: {:?}", part_2(&input, "departure"));
}

fn read_tickets(input: &[String]) -> Vec<Vec<usize>> {
    input
        .iter()
        .map(|l| l.split(",").map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn read_fields<'a>(lines: impl Iterator<Item = &'a String>) -> Vec<Field> {
    lines.filter_map(|l| l.parse().ok()).collect()
}

fn wrong_values(fields: &Vec<Field>, numbers: &Vec<usize>) -> Vec<usize> {
    numbers
        .iter()
        .filter(|number| !(fields.iter().any(|f| f.check(&number))))
        .cloned()
        .collect()
}

fn wrong_values_single_field(field: &Field, numbers: &Vec<usize>) -> Vec<usize> {
    numbers
        .iter()
        .cloned()
        .filter(|number| !(field.check(number)))
        .collect()
}

fn part_1(lines: &Vec<String>) -> Result<usize, SimpleError> {
    //read field definitions
    let fields = read_fields(lines.iter().take_while(|l| !(l.is_empty())));

    let mut invalid_values: Vec<usize> = Vec::new();

    // read lines for nearby tickets
    // skip: fields, 2 empty lines, 2 headers, your own ticket
    for numbers in read_tickets(&lines[(fields.len() + 2 + 2 + 1)..]) {
        invalid_values.extend(wrong_values(&fields, &numbers));
    }

    Ok(invalid_values.iter().cloned().sum())
}

fn part_2(lines: &Vec<String>, field_starts_with: &str) -> Result<usize, SimpleError> {
    //read field definitions
    let fields = read_fields(lines.iter().take_while(|l| !(l.is_empty())));

    let my_ticket_location = fields.len() + 2;
    let mut tickets = read_tickets(&lines[my_ticket_location..=my_ticket_location]);
    let other_tickets_location = my_ticket_location + 3;

    tickets.extend(
        read_tickets(&lines[other_tickets_location..])
            .iter()
            .cloned()
            .filter(|numbers| wrong_values(&fields, &numbers).is_empty()),
    );

    let mut field_matches: Vec<(usize, Vec<usize>)> = Vec::new();

    for field_idx in 0..fields.len() {
        let field = &fields[field_idx];
        let mut matching_data_idx: Vec<usize> = Vec::new();

        for data_idx in 0..fields.len() {
            if tickets.iter().all(|ticket| field.check(&ticket[data_idx])) {
                matching_data_idx.push(data_idx);
            }
        }

        if matching_data_idx.is_empty() {
            return Err(SimpleError::new("could not find a mapping for all fields"));
        }

        field_matches.push((field_idx, matching_data_idx));
    }

    field_matches.sort_by_key(|t| t.1.len());

    let mut data_idx_to_field_idx: HashMap<usize, usize> = HashMap::new();

    for (field_idx, matching_data_idx) in field_matches {
        let remaining_data_idx: Vec<usize> = matching_data_idx
            .iter()
            .cloned()
            .filter(|i| !(data_idx_to_field_idx.contains_key(&i)))
            .collect();

        if remaining_data_idx.is_empty() {
            return Err(SimpleError::new("no remaining data index to assign"));
        }

        data_idx_to_field_idx.insert(remaining_data_idx[0], field_idx);
    }

    if data_idx_to_field_idx.len() != fields.len() {
        return Err(SimpleError::new("could not map all the fields"));
    }

    let my_ticket = &tickets[0];

    Ok(data_idx_to_field_idx
        .iter()
        .filter(|(_, &f)| fields[f].name.starts_with(field_starts_with))
        .map(|(&d, _)| my_ticket[d])
        .product::<usize>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_DATA: Vec<String> = vec![
            "class: 1-3 or 5-7",
            "row: 6-11 or 33-44",
            "seat: 13-40 or 45-50",
            "",
            "your ticket:",
            "7,1,14",
            "",
            "nearby tickets:",
            "7,3,47",
            "40,4,50",
            "55,2,20",
            "38,6,12",
        ]
        .into_iter()
        .map(|line| line.to_string())
        .collect();
        static ref TEST_DATA_2: Vec<String> = vec![
            "class: 0-1 or 4-19",
            "row: 0-5 or 8-19",
            "seat: 0-13 or 16-19",
            "",
            "your ticket:",
            "11,12,13",
            "",
            "nearby tickets:",
            "3,9,18",
            "15,1,5",
            "5,14,9",
        ]
        .into_iter()
        .map(|line| line.to_string())
        .collect();
    }

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(&TEST_DATA), Ok(71));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(&TEST_DATA_2, ""), Ok(11 * 12 * 13));
    }
}
