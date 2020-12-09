use simple_error::SimpleError;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn doit(lines: impl Iterator<Item = String>) -> Result<usize, SimpleError> {
    Ok(lines
        .map(|l| seat_id_for_input(&l))
        .map(|sid| sid.unwrap())
        .max()
        .ok_or(SimpleError::new("no max found"))?)
}

pub fn doit2(lines: impl Iterator<Item = String>) -> Result<usize, SimpleError> {
    let all_seats: HashSet<(usize, usize)> = lines
        .map(|l| seat_for_input(&l))
        .map(|sid| sid.unwrap())
        .collect();

    let min_row = all_seats.iter().map(|s| s.0).min().unwrap();
    let max_row = all_seats.iter().map(|s| s.0).max().unwrap();

    let min_col = all_seats.iter().map(|s| s.1).min().unwrap();
    let max_col = all_seats.iter().map(|s| s.1).max().unwrap();

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            if !(all_seats.contains(&(row, col))) {
                return Ok(row * 8 + col);
            }
        }
    }

    Err(SimpleError::new("nothing missing"))
}

fn seat_for_input(line: &str) -> Result<(usize, usize), SimpleError> {
    let mut input = line.to_string();
    if input.len() != 10 {
        return Err(SimpleError::new("wrong input length"));
    }

    input = input.replace("F", "0");
    input = input.replace("B", "1");
    input = input.replace("L", "0");
    input = input.replace("R", "1");

    Ok((
        usize::from_str_radix(&input[..7], 2)
            .map_err(|_| SimpleError::new("binary conversion error"))?,
        usize::from_str_radix(&input[7..], 2)
            .map_err(|_| SimpleError::new("binary conversion error"))?,
    ))
}

fn seat_id_for_input(line: &str) -> Result<usize, SimpleError> {
    let (row, column) = seat_for_input(line)?;

    Ok((row * 8) + column)
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let reader = BufReader::new(file).lines().map(|line| line.unwrap());

    println!("{:?}", doit2(reader));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pass_1() {
        assert_eq!(seat_for_input("BFFFBBFRRR"), Ok((70, 7)));
        assert_eq!(seat_id_for_input("BFFFBBFRRR"), Ok(567));
    }

    #[test]
    fn pass_2() {
        assert_eq!(seat_for_input("FFFBBBFRRR"), Ok((14, 7)));
        assert_eq!(seat_id_for_input("FFFBBBFRRR"), Ok(119));
    }

    #[test]
    fn pass_3() {
        assert_eq!(seat_for_input("BBFFBBFRLL"), Ok((102, 4)));
        assert_eq!(seat_id_for_input("BBFFBBFRLL"), Ok(820));
    }

    #[test]
    fn get_max() {
        let test_data = vec!["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]
            .into_iter()
            .map(|line| line.to_string());

        assert_eq!(doit(test_data), Ok(820));
    }
}
