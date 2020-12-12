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

fn run(actions: &Vec<Action>) -> Result<i64, SimpleError> {
    let mut position: Position = Position::new(0, 0);
    let mut current_heading = Heading::East;

    for action in actions {
        match action {
            Action::Move(direction, value) => {
                position.change(*direction, *value);
            }
            Action::Turn(direction, value) => match direction {
                TurnDirection::Left => {
                    current_heading = current_heading.turn(value * -1);
                }
                TurnDirection::Right => {
                    current_heading = current_heading.turn(*value);
                }
            },
            Action::Forward(value) => {
                position.change(current_heading, *value);
            }
        }
    }

    Ok(position.manhattan_distance(&Position::new(0, 0)))
}

fn new_heading_for_waypoint(pos: &Position, turn: i64) -> Result<Position, SimpleError> {
    match turn {
        90 | -270 => Ok(Position::new(pos.north, pos.east * -1)),
        180 | -180 => Ok(Position::new(pos.east * -1, pos.north * -1)),
        270 | -90 => Ok(Position::new(pos.north * -1, pos.east)),
        0 => Ok(Position::new(pos.north, pos.east)),
        _ => Err(SimpleError::new("unknown turn")),
    }
}

fn run_2(actions: &Vec<Action>) -> Result<i64, SimpleError> {
    let mut waypoint = Position::new(10, 1);
    let mut ship = Position::new(0, 0);

    for action in actions {
        match action {
            Action::Move(direction, value) => {
                waypoint.change(*direction, *value);
            }
            Action::Forward(value) => {
                ship.east += value * waypoint.east;
                ship.north += value * waypoint.north;
            }
            Action::Turn(direction, value) => match direction {
                TurnDirection::Left => {
                    waypoint = new_heading_for_waypoint(&waypoint, value * -1)?;
                }
                TurnDirection::Right => {
                    waypoint = new_heading_for_waypoint(&waypoint, *value)?;
                }
            },
        }
    }

    Ok(ship.manhattan_distance(&Position::new(0, 0)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref TEST_DATA: Vec<Action> = vec!["F10", "N3", "F7", "R90", "F11",]
            .iter()
            .map(|s| s.parse().unwrap())
            .collect();
    }

    #[test]
    fn new_waypoint_heading() {
        assert_eq!(
            new_heading_for_waypoint(&Position::new(10, 4), 90),
            Ok(Position::new(4, -10))
        );
        assert_eq!(
            new_heading_for_waypoint(&Position::new(4, -10), 90),
            Ok(Position::new(-10, -4))
        );
        assert_eq!(
            new_heading_for_waypoint(&Position::new(-10, -4), 90),
            Ok(Position::new(-4, 10))
        );
        assert_eq!(
            new_heading_for_waypoint(&Position::new(-4, 10), 90),
            Ok(Position::new(10, 4))
        );
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
