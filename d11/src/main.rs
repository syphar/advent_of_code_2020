#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate itertools;

use std::fs::File;
use std::io::{BufRead, BufReader};

use d11::Seats;

fn main() {
    let file = File::open("input.txt").unwrap();

    let seats = Seats::from(BufReader::new(file).lines().map(|line| line.unwrap()));

    println!("part 1: {:?}", run(&seats, 4, &count_other_occupied_seats));
    println!(
        "part 2: {:?}",
        run(&seats, 5, &count_other_occupied_seats_2)
    );
}

fn count_other_occupied_seats_2(seats: &Seats, row: usize, col: usize) -> u8 {
    lazy_static! {
        static ref DIRECTIONS: Vec<(i64, i64)> = iproduct!(-1..=1, -1..=1)
            .filter(|(r, c)| !(*r == 0 && *c == 0))
            .collect();
    }

    let mut count = 0;

    for (rd, cd) in DIRECTIONS.iter() {
        let mut r = row as i64;
        let mut c = col as i64;
        loop {
            r += rd;
            c += cd;

            if r < 0 || c < 0 || r >= (seats.rows() as i64) || c >= (seats.cols() as i64) {
                break;
            }

            if let Some(value) = seats.get(r, c).unwrap() {
                if value == true {
                    count += 1;
                }
                break;
            }
        }
    }

    count
}

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
        .filter(|(r, c)| *r >= 0 && *c >= 0 && *r < seats.rows() as i64 && *c < seats.cols() as i64)
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
    // for _ in 0..3 {
    while did_change {
        did_change = false;

        let mut new_seats = Seats::new(seats.rows(), seats.cols());
        for row in 0..old_seats.rows() {
            for col in 0..old_seats.cols() {
                new_seats
                    .set(
                        row as i64,
                        col as i64,
                        match old_seats.get(row as i64, col as i64).unwrap() {
                            None => None, // floor
                            Some(false) => {
                                // If a seat is empty (L) and there are no occupied
                                // seats adjacent to it, the seat becomes occupied.
                                if seat_check_function(&old_seats, row, col) == 0 {
                                    did_change = true;
                                    Some(true)
                                } else {
                                    Some(false)
                                }
                            }
                            Some(true) => {
                                // If a seat is occupied (#) and X our or more seats
                                // adjacent to it are also occupied, the seat becomes empty.
                                if seat_check_function(&old_seats, row, col)
                                    >= too_many_seats_visible
                                {
                                    did_change = true;
                                    Some(false)
                                } else {
                                    Some(true)
                                }
                            }
                        },
                    )
                    .unwrap();
            }
        }
        old_seats = new_seats;
    }

    old_seats.count(Some(true))
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
        let seats = Seats::from(TEST_DATA.iter().cloned());
        assert_eq!(run(&seats, 4, &count_other_occupied_seats), 37);
    }

    #[test]
    fn new_seat_check() {
        let seats = Seats::from(TEST_DATA_SEAT_CHECK_1.iter().cloned());
        assert_eq!(count_other_occupied_seats_2(&seats, 4, 3), 8);
    }

    #[test]
    fn new_seat_check_2() {
        let seats = Seats::from(TEST_DATA_SEAT_CHECK_2.iter().cloned());
        assert_eq!(count_other_occupied_seats_2(&seats, 1, 1), 0);
    }

    #[test]
    fn new_seat_check_3() {
        let seats = Seats::from(TEST_DATA_SEAT_CHECK_3.iter().cloned());
        assert_eq!(count_other_occupied_seats_2(&seats, 3, 3), 0);
    }

    #[test]
    fn part_2_works() {
        let seats = Seats::from(TEST_DATA.iter().cloned());
        assert_eq!(run(&seats, 5, &count_other_occupied_seats_2), 26);
    }
}
