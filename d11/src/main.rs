#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("part 1:{:?}", run(&input, 4));
}

fn print_seats(seats: &Vec<Vec<Option<bool>>>) {
    for line in seats {
        println!(
            "{}",
            line.iter()
                .map(|s| match s {
                    None => ".",
                    Some(false) => "L",
                    Some(true) => "#",
                })
                .collect::<String>()
        );
    }
}

fn read_seats(lines: &Vec<String>) -> Vec<Vec<Option<bool>>> {
    let mut seats: Vec<Vec<Option<bool>>> = Vec::new();
    for line in lines {
        seats.push(
            line.chars()
                .map(|c| match c {
                    'L' => Some(false),
                    _ => None,
                })
                .collect(),
        );
    }

    seats
}

fn count_other_occupied_seats(seats: &Vec<Vec<Option<bool>>>, row: usize, col: usize) -> u8 {
    let row_ = row as i64;
    let col_ = col as i64;

    let surrounding_seats = vec![
        (row_ - 1, col_),
        (row_ + 1, col_),
        (row_, col_ - 1),
        (row_, col_ + 1),
        (row_ - 1, col_ - 1),
        (row_ - 1, col_ + 1),
        (row_ + 1, col_ - 1),
        (row_ + 1, col_ + 1),
    ];

    let width: i64 = seats[0].len() as i64;
    let height: i64 = seats.len() as i64;

    surrounding_seats
        .iter()
        .filter(|(r, c)| (*r >= 0 && *c >= 0 && *r < height && *c < width))
        .map(|(r, c)| seats[*r as usize][*c as usize])
        .filter(|v| *v == Some(true))
        .count() as u8
}

fn run(lines: &Vec<String>, too_many_seats_visible: u8) -> usize {
    let mut seats = read_seats(&lines);

    let mut did_change = true;
    while did_change {
        did_change = false;

        let mut new_seats: Vec<Vec<Option<bool>>> = Vec::new();

        for (row, line) in seats.iter().enumerate() {
            let mut new_line: Vec<Option<bool>> = Vec::new();

            for (col, value) in line.iter().enumerate() {
                new_line.push(match value {
                    None => None, // floor
                    Some(false) => {
                        // If a seat is empty (L) and there are no occupied
                        // seats adjacent to it, the seat becomes occupied.
                        if count_other_occupied_seats(&seats, row, col) == 0 {
                            did_change = true;
                            Some(true)
                        } else {
                            Some(false)
                        }
                    }
                    Some(true) => {
                        // If a seat is occupied (#) and four or more seats
                        // adjacent to it are also occupied, the seat becomes empty.
                        if count_other_occupied_seats(&seats, row, col) >= too_many_seats_visible {
                            did_change = true;
                            Some(false)
                        } else {
                            Some(true)
                        }
                    }
                });
            }
            new_seats.push(new_line);
        }
        seats = new_seats;
    }

    seats
        .iter()
        .map(|row| row.iter().filter(|v| **v == Some(true)).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_DATA: Vec<String> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
    }

    #[test]
    fn part_1_works() {
        assert_eq!(run(&TEST_DATA, 4), 37);
    }

    // #[test]
    // fn part_2_works() {
    //     assert_eq!(run2(&TEST_DATA), 26);
    // }
}
