#[macro_use]
extern crate lazy_static;

use d12::{Action, Heading, Position, TurnDirection};
use simple_error::SimpleError;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let actions: Vec<Action> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!("part 1: {:?}", run(&actions));
    println!("part 2: {:?}", run_2(&actions));
}

fn new_heading(current_heading: i64, diff: i64) -> i64 {
    if diff < 0 {
        (current_heading + (360 + diff)).abs() % 360
    } else {
        (current_heading + diff).abs() % 360
    }
}

fn run(actions: &Vec<Action>) -> Result<i64, SimpleError> {
    let mut position: Position = Position::new(0, 0);
    let mut current_heading = Heading::East;

    for action in actions {
        // match action {
        //     Action::Move(direction, value) => {
        //         position.change(*direction, *value);
        //     }
        //     Action::Turn(direction, value) => match direction {
        //         // TurnDirection::Left => {
        //         //     current_heading = new_heading(current_heading, value * -1);
        //         // }
        //         // TurnDirection::Right => {
        //         //     current_heading = new_heading(current_heading, *value);
        //         // }
        //     },
        //     Action::Forward(value) => {
        //         position.change(current_heading, *value);
        //     }
        // }
    }

    Ok(position.manhattan_distance(&Position::new(0, 0)))
}

fn new_heading_for_waypoint(pos: (i64, i64), turn: i64) -> Result<(i64, i64), SimpleError> {
    match turn {
        90 | -270 => Ok((pos.1, pos.0 * -1)),
        180 | -180 => Ok((pos.0 * -1, pos.1 * -1)),
        270 | -90 => Ok((pos.1 * -1, pos.0)),
        0 => Ok((pos.0, pos.1)),
        _ => Err(SimpleError::new("unknown turn")),
    }
}

fn run_2(actions: &Vec<Action>) -> Result<i64, SimpleError> {
    let mut waypoint_position: (i64, i64) = (10, 1);
    let mut ship_position_east_west: i64 = 0;
    let mut ship_position_north_south: i64 = 0;

    for action in actions {
        match action {
            Action::Move(direction, value) => match direction {
                Heading::North => {
                    waypoint_position.1 += value;
                }
                Heading::South => {
                    waypoint_position.1 -= value;
                }
                Heading::East => {
                    waypoint_position.0 += value;
                }
                Heading::West => {
                    waypoint_position.0 -= value;
                }
            },
            Action::Forward(value) => {
                ship_position_east_west += value * waypoint_position.0;
                ship_position_north_south += value * waypoint_position.1;
            }
            Action::Turn(direction, value) => match direction {
                TurnDirection::Left => {
                    waypoint_position = new_heading_for_waypoint(waypoint_position, value * -1)?;
                }
                TurnDirection::Right => {
                    waypoint_position = new_heading_for_waypoint(waypoint_position, *value)?;
                }
            },
        }
    }

    Ok(ship_position_east_west.abs() + ship_position_north_south.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_DATA: Vec<Action> = vec!["F10", "N3", "F7", "R90", "F11",]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
    }

    #[test]
    fn test_new_heading() {
        assert_eq!(new_heading(0, 0), 0);
        assert_eq!(new_heading(0, 90), 90);
        assert_eq!(new_heading(0, 180), 180);
        assert_eq!(new_heading(0, 270), 270);
        assert_eq!(new_heading(0, 360), 0);

        assert_eq!(new_heading(90, 0), 90);
        assert_eq!(new_heading(90, 90), 180);
        assert_eq!(new_heading(90, 180), 270);
        assert_eq!(new_heading(90, 270), 0);
        assert_eq!(new_heading(90, 360), 90);

        assert_eq!(new_heading(180, 0), 180);
        assert_eq!(new_heading(180, 90), 270);
        assert_eq!(new_heading(180, 180), 0);
        assert_eq!(new_heading(180, 270), 90);
        assert_eq!(new_heading(180, 360), 180);

        assert_eq!(new_heading(0, -90), 270);
        assert_eq!(new_heading(0, -180), 180);
        assert_eq!(new_heading(0, -270), 90);
        assert_eq!(new_heading(0, -360), 0);

        assert_eq!(new_heading(90, -90), 0);
        assert_eq!(new_heading(90, -180), 270);
        assert_eq!(new_heading(90, -270), 180);
        assert_eq!(new_heading(90, -360), 90);

        assert_eq!(new_heading(180, -90), 90);
        assert_eq!(new_heading(180, -180), 0);
        assert_eq!(new_heading(180, -270), 270);
        assert_eq!(new_heading(180, -360), 180);
    }

    #[test]
    fn new_waypoint_heading() {
        assert_eq!(new_heading_for_waypoint((10, 4), 90), Ok((4, -10)));
        assert_eq!(new_heading_for_waypoint((4, -10), 90), Ok((-10, -4)));
        assert_eq!(new_heading_for_waypoint((-10, -4), 90), Ok((-4, 10)));
        assert_eq!(new_heading_for_waypoint((-4, 10), 90), Ok((10, 4)));
    }

    #[test]
    fn part_1_works() {
        assert_eq!(run(&TEST_DATA), Ok(25));
    }

    #[test]
    fn part_2_works() {
        assert_eq!(run_2(&TEST_DATA), Ok(286));
    }
}
