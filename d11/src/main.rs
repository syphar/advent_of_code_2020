#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate itertools;

use simple_error::SimpleError;
use std::convert::From;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
struct Seats {
    // TODO: use 2d vector
    data: Vec<Vec<Option<bool>>>,
    cols: usize,
    rows: usize,
}

impl fmt::Display for Seats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.data.iter() {
            writeln!(
                f,
                "{}",
                line.iter()
                    .map(|s| match s {
                        None => ".",
                        Some(false) => "L",
                        Some(true) => "#",
                    })
                    .collect::<String>()
            )?
        }
        Ok(())
    }
}

impl Seats {
    pub fn new(rows: usize, cols: usize) -> Self {
        Seats {
            data: vec![vec![None; cols]; rows],
            cols,
            rows,
        }
    }
    pub fn get(&self, row: i64, col: i64) -> Result<Option<bool>, SimpleError> {
        if row < 0 || col < 0 || row >= (self.rows as i64) || col >= (self.cols as i64) {
            Err(SimpleError::new("invalid row/col"))
        } else {
            Ok(self.data[row as usize][col as usize])
        }
    }

    pub fn set(&mut self, row: i64, col: i64, value: Option<bool>) -> () {
        if row >= 0 && col >= 0 || row < (self.rows as i64) || col < (self.cols as i64) {
            self.data[row as usize][col as usize] = value;
        }
    }

    pub fn count(&self, which: Option<bool>) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|v| **v == which).count())
            .sum()
    }

    pub fn from(lines: impl Iterator<Item = String>) -> Self {
        let data: Vec<Vec<Option<bool>>> = lines
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'L' => Some(false),
                        '#' => Some(true),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        if data.len() > 0 {
            let cols = data[0].len();
            let rows = data.len();
            Seats { data, cols, rows }
        } else {
            Seats::new(0, 0)
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let seats = Seats::from(BufReader::new(file).lines().map(|line| line.unwrap()));

    println!("part 1: {:?}", run(&seats, 4, &count_other_occupied_seats));
    // println!(
    //     "part 2: {:?}",
    //     run(&seats, 5, &count_other_occupied_seats_2)
    // );
}

// fn count_other_occupied_seats_2(seats: &Seats, row: usize, col: usize) -> u8 {
//     lazy_static! {
//         static ref DIRECTIONS: Vec<(i64, i64)> = iproduct!(-1..=1, -1..=1)
//             .filter(|(r, c)| !(*r == 0 && *c == 0))
//             .collect();
//     }

//     let width: i64 = seats[0].len() as i64;
//     let height: i64 = seats.len() as i64;

//     let mut count = 0;

//     for (rd, cd) in DIRECTIONS.iter() {
//         let mut r = row as i64;
//         let mut c = col as i64;
//         loop {
//             r += rd;
//             c += cd;

//             if r < 0 || c < 0 || r >= height || c >= width {
//                 break;
//             }

//             if let Some(value) = seats[r as usize][c as usize] {
//                 if value == true {
//                     count += 1;
//                 }
//                 break;
//             }
//         }
//     }

//     count
// }

fn count_other_occupied_seats(seats: &Seats, row: usize, col: usize) -> u8 {
    let row_ = row as i64;
    let col_ = col as i64;

    let surrounding_seats: Vec<(i64, i64)> = vec![
        (row_ - 1, col_),
        (row_ + 1, col_),
        (row_, col_ - 1),
        (row_, col_ + 1),
        (row_ - 1, col_ - 1),
        (row_ - 1, col_ + 1),
        (row_ + 1, col_ - 1),
        (row_ + 1, col_ + 1),
    ];

    surrounding_seats
        .iter()
        .filter(|(r, c)| (*r >= 0 && *c >= 0 && *r < seats.rows as i64 && *c < seats.cols as i64))
        .filter_map(|(r, c)| seats.get(*r as i64, *c as i64).ok())
        .filter(|v| *v == Some(true))
        .count() as u8
}

fn run(
    seats: &Seats,
    too_many_seats_visible: u8,
    seat_check_function: &dyn Fn(&Seats, usize, usize) -> u8,
) -> usize {
    let mut old_seats = seats.clone();

    let mut did_change = true;
    while did_change {
        did_change = false;

        let mut new_seats = Seats::new(seats.rows, seats.cols);

        for row in 0..old_seats.rows {
            for col in 0..old_seats.cols {
                new_seats.set(
                    row as i64,
                    col as i64,
                    match old_seats.get(row as i64, col as i64).unwrap() {
                        None => None, // floor
                        Some(false) => {
                            // If a seat is empty (L) and there are no occupied
                            // seats adjacent to it, the seat becomes occupied.
                            if seat_check_function(&seats, row, col) == 0 {
                                did_change = true;
                                Some(true)
                            } else {
                                Some(false)
                            }
                        }
                        Some(true) => {
                            // If a seat is occupied (#) and X our or more seats
                            // adjacent to it are also occupied, the seat becomes empty.
                            if seat_check_function(&seats, row, col) >= too_many_seats_visible {
                                did_change = true;
                                Some(false)
                            } else {
                                Some(true)
                            }
                        }
                    },
                );
            }
        }
        old_seats = new_seats;
    }

    seats.count(Some(true))
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
        static ref TEST_DATA_SEAT_CHECK_1: Vec<String> = vec![
            ".......#.",
            "...#.....",
            ".#.......",
            ".........",
            "..#L....#",
            "....#....",
            ".........",
            "#........",
            "...#.....",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        static ref TEST_DATA_SEAT_CHECK_2: Vec<String> =
            vec![".............", ".L.L.#.#.#.#.", ".............",]
                .iter()
                .map(|s| s.to_string())
                .collect();
        static ref TEST_DATA_SEAT_CHECK_3: Vec<String> =
            vec![".##.##.", "#.#.#.#", "##...##", "...L...", "##...##", "#.#.#.#", ".##.##.",]
                .iter()
                .map(|s| s.to_string())
                .collect();
    }

    #[test]
    fn part_1_works() {
        let seats = Seats::from(TEST_DATA.iter());
        assert_eq!(run(&seats, 4, &count_other_occupied_seats), 37);
    }

    // #[test]
    // fn new_seat_check() {
    //     let seats = read_seats(&TEST_DATA_SEAT_CHECK_1);
    //     assert_eq!(count_other_occupied_seats_2(&seats, 4, 3), 8);
    // }

    // #[test]
    // fn new_seat_check_2() {
    //     let seats = read_seats(&TEST_DATA_SEAT_CHECK_2);
    //     assert_eq!(count_other_occupied_seats_2(&seats, 1, 1), 0);
    // }
    // #[test]
    // fn new_seat_check_3() {
    //     let seats = read_seats(&TEST_DATA_SEAT_CHECK_3);
    //     assert_eq!(count_other_occupied_seats_2(&seats, 3, 3), 0);
    // }

    // #[test]
    // fn part_2_works() {
    //     assert_eq!(run(&TEST_DATA, 5, &count_other_occupied_seats_2), 26);
    // }
}
