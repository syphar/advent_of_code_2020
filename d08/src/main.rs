#![feature(str_split_once)]

use simple_error::SimpleError;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq)]
enum Command {
    Acc(i64),
    Jmp(i64),
    Nop(i64),
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    if let Ok(commands) = read_commands(&input) {
        println!("part 1:{:?}", run(&commands));
        println!("part 2:{:?}", try_fix(&commands));
    }
}

fn read_commands(lines: &Vec<String>) -> Result<Vec<Command>, SimpleError> {
    lines
        .iter()
        .filter_map(|line| line.split_once(" "))
        .map(|(op, value_str)| (op.to_owned(), value_str.parse::<i64>().unwrap()))
        .map(|(op, value)| match &op[..] {
            "acc" => Ok(Command::Acc(value)),
            "jmp" => Ok(Command::Jmp(value)),
            "nop" => Ok(Command::Nop(value)),
            _ => Err(SimpleError::new(format!("unknown command: {}", op))),
        })
        .collect()
}

fn run(commands: &Vec<Command>) -> Result<i64, SimpleError> {
    let mut accumulator: i64 = 0;
    let mut steps_done: HashSet<usize> = HashSet::new();

    let mut i: usize = 0;
    loop {
        if steps_done.contains(&i) {
            // jump to an already done location is an error
            return Err(SimpleError::new(accumulator.to_string()));
        }
        if i == commands.len() {
            // successful exit position is 1 after the last element
            return Ok(accumulator);
        }

        steps_done.insert(i);
        match commands[i] {
            Command::Acc(value) => {
                accumulator += value;
                i += 1;
            }
            Command::Jmp(value) => {
                i = (i as i64 + value) as usize;
            }
            Command::Nop(_) => {
                i += 1;
            }
        }
    }
}

fn try_fix(commands: &Vec<Command>) -> Result<i64, SimpleError> {
    for line_to_try in 0..commands.len() {
        let mut test_data: Vec<Command> = commands.clone();

        match test_data[line_to_try] {
            Command::Nop(value) => {
                test_data[line_to_try] = Command::Jmp(value);
                if let Ok(result) = run(&test_data) {
                    return Ok(result);
                }
            }
            Command::Jmp(value) => {
                test_data[line_to_try] = Command::Nop(value);
                if let Ok(result) = run(&test_data) {
                    return Ok(result);
                }
            }
            _ => {}
        }
    }
    Err(SimpleError::new("nothing found"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }

    #[test]
    fn test_unknown_commands() {
        let td: Vec<String> = vec!["asd +0".to_string()];

        assert_eq!(
            read_commands(&td),
            Err(SimpleError::new("unknown command: asd"))
        );
    }

    #[test]
    fn test_read_commands() {
        let td: Vec<String> = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
        ];

        let commands = read_commands(&td).unwrap();
        assert_eq!(commands[0], Command::Nop(0));
        assert_eq!(commands[1], Command::Acc(1));
        assert_eq!(commands[2], Command::Jmp(-4));
    }

    #[test]
    fn it_works() {
        let td = test_data();
        let commands = read_commands(&td).unwrap();
        assert_eq!(run(&commands), Err(SimpleError::new("5")));
    }

    #[test]
    fn test_try_fix() {
        let td = test_data();
        let commands = read_commands(&td).unwrap();
        assert_eq!(try_fix(&commands), Ok(8));
    }
}
