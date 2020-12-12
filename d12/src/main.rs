#[macro_use]
extern crate lazy_static;

use simple_error::SimpleError;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, PartialEq)]
enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Move(Direction, i64),
    Turn(TurnDirection, i64),
    Forward(i64),
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    if let Ok(actions) = read_actions(&input) {
        println!("part 1: {:?}", run(&actions));
        println!("part 2: {:?}", run_2(&actions));
    }
}

fn read_actions(lines: &Vec<String>) -> Result<Vec<Action>, SimpleError> {
    lines
        .iter()
        .map(|s| s.split_at(1))
        .map(|(a, b)| (a, b.parse::<i64>().unwrap()))
        .map(|(cmd, value)| match &cmd[..] {
            "N" => Ok(Action::Move(Direction::North, value)),
            "S" => Ok(Action::Move(Direction::South, value)),
            "E" => Ok(Action::Move(Direction::East, value)),
            "W" => Ok(Action::Move(Direction::West, value)),
            "L" => Ok(Action::Turn(TurnDirection::Left, value)),
            "R" => Ok(Action::Turn(TurnDirection::Right, value)),
            "F" => Ok(Action::Forward(value)),
            _ => Err(SimpleError::new("invalid command")),
        })
        .collect()
}

fn new_heading(current_heading: i64, diff: i64) -> i64 {
    if diff < 0 {
        (current_heading + (360 + diff)).abs() % 360
    } else {
        (current_heading + diff).abs() % 360
    }
}

fn run(actions: &Vec<Action>) -> Result<i64, SimpleError> {
    let mut position_east_west: i64 = 0;
    let mut position_north_south: i64 = 0;
    let mut current_heading = 90;

    for action in actions {
        match action {
            Action::Move(direction, value) => match direction {
                Direction::North => {
                    position_north_south += value;
                }
                Direction::South => {
                    position_north_south -= value;
                }
                Direction::East => {
                    position_east_west += value;
                }
                Direction::West => {
                    position_east_west -= value;
                }
            },
            Action::Turn(direction, value) => match direction {
                TurnDirection::Left => {
                    current_heading = new_heading(current_heading, value * -1);
                }
                TurnDirection::Right => {
                    current_heading = new_heading(current_heading, *value);
                }
            },
            Action::Forward(value) => match current_heading {
                0 => {
                    position_north_south += value;
                }
                90 => {
                    position_east_west += value;
                }
                180 => {
                    position_north_south -= value;
                }
                270 => {
                    position_east_west -= value;
                }
                _ => {
                    return Err(SimpleError::new("unknown heading"));
                }
            },
        }
    }

    Ok(position_east_west.abs() + position_north_south.abs())
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
        println!("\n\nBEFORE:");
        println!(
            "waypoint E{} / N{}",
            waypoint_position.0, waypoint_position.1
        );
        println!(
            "ship E{} / N{}",
            ship_position_east_west, ship_position_north_south
        );
        println!("action: {:?}", action);

        match action {
            Action::Move(direction, value) => match direction {
                Direction::North => {
                    waypoint_position.1 += value;
                }
                Direction::South => {
                    waypoint_position.1 -= value;
                }
                Direction::East => {
                    waypoint_position.0 += value;
                }
                Direction::West => {
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

        println!("AFTER:");
        println!(
            "waypoint E{} / N{}",
            waypoint_position.0, waypoint_position.1
        );
        println!(
            "ship E{} / N{}",
            ship_position_east_west, ship_position_north_south
        );
    }

    Ok(ship_position_east_west.abs() + ship_position_north_south.abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref TEST_DATA: Vec<String> = vec!["F10", "N3", "F7", "R90", "F11",]
            .iter()
            .map(|s| s.to_string())
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
    fn test_read_actions() {
        let actions = read_actions(&TEST_DATA).unwrap();
        assert_eq!(actions[0], Action::Forward(10));
        assert_eq!(actions[1], Action::Move(Direction::North, 3));
        assert_eq!(actions[2], Action::Forward(7));
        assert_eq!(actions[3], Action::Turn(TurnDirection::Right, 90));
        assert_eq!(actions[4], Action::Forward(11));
    }

    #[test]
    fn part_1_works() {
        let actions = read_actions(&TEST_DATA).unwrap();
        assert_eq!(run(&actions), Ok(25));
    }

    #[test]
    fn part_2_works() {
        let actions = read_actions(&TEST_DATA).unwrap();
        assert_eq!(run_2(&actions), Ok(286));
    }
}
