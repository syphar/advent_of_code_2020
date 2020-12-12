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
        //     println!("part 2: {:?}", try_fix(&commands));
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

fn run(actions: &Vec<Action>) -> Result<i64, SimpleError> {
    let mut position_east_west: i64 = 0;
    let mut position_north_south: i64 = 0;
    let mut current_heading = 90;

    for action in actions {
        println!(
            "\n\nbefore E{} / N{} => {}",
            position_east_west, position_north_south, current_heading
        );
        println!("action {:?}", action);
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
                    current_heading = (current_heading - value).abs() % 360;
                }
                TurnDirection::Right => {
                    current_heading = (current_heading + value).abs() % 360;
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
        println!(
            "after E{} / N{} => {}",
            position_east_west, position_north_south, current_heading
        );
    }

    Ok(position_east_west.abs() + position_north_south.abs())
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
}
