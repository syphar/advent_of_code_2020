// use lazy_static::lazy_static;
// use regex::Regex;
// use simple_error::SimpleError;
// use std::str::FromStr;

// pub type BitMask = Vec<Option<bool>>;

// #[derive(Debug, Clone, PartialEq)]
// pub enum Command {
//     SetValue(u64, u64),
//     SetMask(BitMask),
// }

// lazy_static! {
//     static ref REGEX_MASK: Regex = Regex::new(r"mask = ([01X]+)").unwrap();
//     static ref REGEX_MEM: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
// }

// impl FromStr for Command {
//     type Err = SimpleError;

//     fn from_str(input: &str) -> Result<Self, Self::Err> {
//         if let Some(cap) = REGEX_MASK.captures(input) {
//             Ok(Command::SetMask(
//                 cap.get(1)
//                     .unwrap()
//                     .as_str()
//                     .chars()
//                     .rev()
//                     .map(|v| match v {
//                         'X' => None,
//                         '1' => Some(true),
//                         _ => Some(false),
//                     })
//                     .collect(),
//             ))
//         } else if let Some(cap) = REGEX_MEM.captures(input) {
//             Ok(Command::SetValue(
//                 cap.get(1).unwrap().as_str().parse().unwrap(),
//                 cap.get(2).unwrap().as_str().parse().unwrap(),
//             ))
//         } else {
//             Err(SimpleError::new(format!(
//                 "could not parse command from {}",
//                 input
//             )))
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use test_case::test_case;

//     #[test_case("asdf")]
//     #[test_case("mem[-1] = 1234")]
//     #[test_case("mask = AB")]
//     fn test_parse_commands_error(input: &str) {
//         assert!(input.parse::<Command>().is_err());
//     }

//     #[test_case("mem[88] = 9999", Command::SetValue(88, 9999))]
//     #[test_case("mask = 0X01X", Command::SetMask(vec![None, Some(true), Some(false), None, Some(false)]))]
//     fn test_parse_commands(input: &str, expected: Command) {
//         assert_eq!(input.parse(), Ok(expected));
//     }
// }
