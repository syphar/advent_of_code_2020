use num::ToPrimitive;
use num::{Integer, Signed};
use simple_error::SimpleError;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    pub east: i64,
    pub north: i64,
}

impl Position {
    pub fn new(east: i64, north: i64) -> Self {
        Self { east, north }
    }

    pub fn manhattan_distance(&self, other: &Position) -> i64 {
        (other.east - self.east).abs() + (other.north - self.north).abs()
    }

    pub fn change(&mut self, heading: Heading, value: i64) -> &mut Self {
        match heading {
            Heading::North => {
                self.north += value;
            }
            Heading::South => {
                self.north -= value;
            }
            Heading::East => {
                self.east += value;
            }
            Heading::West => {
                self.east -= value;
            }
        }
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Heading {
    North,
    South,
    East,
    West,
}

impl<T> From<T> for Heading
where
    T: Signed + Integer + ToPrimitive + Display,
{
    fn from(value: T) -> Self {
        let value_ = ToPrimitive::to_i64(&value).unwrap();

        let v = if value_ >= 0 {
            value_ % 360
        } else {
            360 - (value_.abs() % 360)
        };

        match v {
            0..=89 => Heading::North,
            90..=179 => Heading::East,
            180..=269 => Heading::South,
            270..=359 => Heading::West,
            360 => Heading::North,
            _ => {
                // This cannot happen because of the code before
                panic!(format!("unknown heading value: {}", value));
            }
        }
    }
}

impl From<Heading> for u16 {
    fn from(value: Heading) -> Self {
        match value {
            Heading::North => 0,
            Heading::East => 90,
            Heading::South => 180,
            Heading::West => 270,
        }
    }
}

impl Heading {
    pub fn turn(&self, by: i64) -> Self {
        let mut heading = u16::from(*self) as i64;
        if by < 0 {
            heading = (heading + (360 + by)).abs() % 360
        } else {
            heading = (heading + by).abs() % 360
        }

        Heading::from(heading)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TurnDirection {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    Move(Heading, i64),
    Turn(TurnDirection, i64),
    Forward(i64),
}

impl FromStr for Action {
    type Err = SimpleError;

    fn from_str(action_str: &str) -> Result<Self, Self::Err> {
        if action_str.len() < 2 {
            return Err(SimpleError::new("string too short"));
        }

        let (cmd, val) = action_str.split_at(1);

        if let Ok(value) = val.parse::<i64>() {
            match cmd {
                "N" => Ok(Action::Move(Heading::North, value)),
                "S" => Ok(Action::Move(Heading::South, value)),
                "E" => Ok(Action::Move(Heading::East, value)),
                "W" => Ok(Action::Move(Heading::West, value)),
                "L" => Ok(Action::Turn(TurnDirection::Left, value)),
                "R" => Ok(Action::Turn(TurnDirection::Right, value)),
                "F" => Ok(Action::Forward(value)),
                _ => Err(SimpleError::new("invalid command")),
            }
        } else {
            Err(SimpleError::new("could not parse number"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("F10", Action::Forward(10))]
    #[test_case("N3", Action::Move(Heading::North, 3))]
    #[test_case("R90", Action::Turn(TurnDirection::Right, 90))]
    fn test_actions_from_str(s: &str, expected: Action) {
        assert_eq!(s.parse::<Action>(), Ok(expected));
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(
            Position::new(0, 0).manhattan_distance(&Position::new(3, 2)),
            5
        );

        assert_eq!(
            Position::new(17, 8).manhattan_distance(&Position::new(0, 0)),
            25
        );
    }

    #[test_case(-450, Heading::West ;"-450" )]
    #[test_case(-360, Heading::North ; "-360" )]
    #[test_case(-270, Heading::East; "-270" )]
    #[test_case(-180, Heading::South; "-180" )]
    #[test_case(-90, Heading::West; "-90" )]
    #[test_case(0, Heading::North)]
    #[test_case(90, Heading::East)]
    #[test_case(180, Heading::South)]
    #[test_case(188, Heading::South)]
    #[test_case(270, Heading::West)]
    #[test_case(300, Heading::West)]
    #[test_case(360, Heading::North)]
    #[test_case(450, Heading::East)]
    fn test_heading_from(input: i16, expected: Heading) {
        assert_eq!(Heading::from(input), expected);
    }

    #[test_case(Heading::West, 0, Heading::West)]
    #[test_case(Heading::West, 90, Heading::North)]
    #[test_case(Heading::West, 180, Heading::East)]
    #[test_case(Heading::West, 270, Heading::South)]
    #[test_case(Heading::West, 360, Heading::West)]
    #[test_case(Heading::West, 450, Heading::North)]
    #[test_case(Heading::West, -90, Heading::South; "west/-90")]
    #[test_case(Heading::West, -180, Heading::East; "west/-180")]
    #[test_case(Heading::West, -270, Heading::North; "west/-270")]
    #[test_case(Heading::West, -360, Heading::West; "west/-360" )]
    #[test_case(Heading::West, -450, Heading::South; "west/-450")]
    fn test_heading_turn(initial: Heading, turn: i64, expected: Heading) {
        assert_eq!(initial.turn(turn), expected);
    }
}
