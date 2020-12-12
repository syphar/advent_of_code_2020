use num::Signed;
use simple_error::SimpleError;
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

impl From<i64> for Heading {
    fn from(value: i64) -> Self {
        let v = if value >= 0 {
            value % 360
        } else {
            360 - (value.abs() % 360)
        };
        match v {
            0 => Heading::North,
            90 => Heading::East,
            180 => Heading::South,
            270 => Heading::West,
            360 => Heading::North,
            _ => {
                panic!(format!("unknown heading value: {} ({})", value, v));
            }
        }
    }
}

impl From<i32> for Heading {
    fn from(value: i32) -> Self {
        Heading::from(value as i64)
    }
}

impl From<&Heading> for u16 {
    fn from(value: &Heading) -> Self {
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
        let mut heading = u16::from(self) as i64;
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

    #[test]
    fn test_actions_from_str() {
        assert_eq!("F10".parse::<Action>(), Ok(Action::Forward(10)));
        assert_eq!("N3".parse::<Action>(), Ok(Action::Move(Heading::North, 3)));
        assert_eq!(
            "R90".parse::<Action>(),
            Ok(Action::Turn(TurnDirection::Right, 90))
        );
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

    #[test]
    fn test_heading_from() {
        assert_eq!(Heading::from(-450), Heading::West);
        assert_eq!(Heading::from(-360), Heading::North);
        assert_eq!(Heading::from(-270), Heading::East);
        assert_eq!(Heading::from(-180), Heading::South);
        assert_eq!(Heading::from(-90), Heading::West);
        assert_eq!(Heading::from(0), Heading::North);
        assert_eq!(Heading::from(90), Heading::East);
        assert_eq!(Heading::from(180), Heading::South);
        assert_eq!(Heading::from(270), Heading::West);
        assert_eq!(Heading::from(360), Heading::North);
        assert_eq!(Heading::from(450), Heading::East);
    }

    #[test]
    fn test_heading_turn() {
        assert_eq!(Heading::West.turn(0), Heading::West);
        assert_eq!(Heading::West.turn(90), Heading::North);
        assert_eq!(Heading::West.turn(180), Heading::East);
        assert_eq!(Heading::West.turn(270), Heading::South);
        assert_eq!(Heading::West.turn(360), Heading::West);
        assert_eq!(Heading::West.turn(450), Heading::North);

        assert_eq!(Heading::West.turn(-90), Heading::South);
        assert_eq!(Heading::West.turn(-180), Heading::East);
        assert_eq!(Heading::West.turn(-270), Heading::North);
        assert_eq!(Heading::West.turn(-360), Heading::West);
        assert_eq!(Heading::West.turn(-450), Heading::South);
    }
}
