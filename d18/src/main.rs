#[macro_use]
extern crate simple_error;
extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1 as digit, space0 as space},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};
use simple_error::SimpleResult;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let file = File::open("input.txt").unwrap();

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("part 1: {:?}", part_1::run(&lines));
    println!("part 2: {:?}", part_2::run(&lines));
}

// arithmetic NOM example:
// https://github.com/Geal/nom/blob/master/tests/arithmetic.rs

mod part_1 {
    use super::*;

    pub fn run(lines: &[String]) -> SimpleResult<i64> {
        let mut sum = 0;

        for line in lines {
            if let Ok((_, result)) = expr(line) {
                sum += result;
            } else {
                bail!("parsing error");
            }
        }
        Ok(sum)
    }

    fn parens(i: &str) -> IResult<&str, i64> {
        delimited(space, delimited(tag("("), expr, tag(")")), space)(i)
    }

    fn factor(i: &str) -> IResult<&str, i64> {
        alt((
            map_res(delimited(space, digit, space), FromStr::from_str),
            parens,
        ))(i)
    }

    pub fn expr(i: &str) -> IResult<&str, i64> {
        let (i, init) = factor(i)?;

        fold_many0(
            pair(alt((char('+'), char('*'))), factor),
            init,
            |acc, (op, val): (char, i64)| {
                if op == '+' {
                    acc + val
                } else {
                    acc * val
                }
            },
        )(i)
    }
}

mod part_2 {
    use super::*;

    pub fn run(lines: &[String]) -> SimpleResult<i64> {
        let mut sum = 0;

        for line in lines {
            if let Ok((_, result)) = expr(line) {
                sum += result;
            }
        }
        Ok(sum)
    }

    fn parens(i: &str) -> IResult<&str, i64> {
        delimited(space, delimited(tag("("), expr, tag(")")), space)(i)
    }

    fn factor(i: &str) -> IResult<&str, i64> {
        alt((
            map_res(delimited(space, digit, space), FromStr::from_str),
            parens,
        ))(i)
    }

    fn term(i: &str) -> IResult<&str, i64> {
        let (i, init) = factor(i)?;

        fold_many0(
            pair(alt((char('+'), char('-'))), factor),
            init,
            |acc, (op, val): (char, i64)| {
                if op == '+' {
                    acc + val
                } else {
                    acc - val
                }
            },
        )(i)
    }

    pub fn expr(i: &str) -> IResult<&str, i64> {
        let (i, init) = term(i)?;

        fold_many0(
            pair(alt((char('*'), char('/'))), term),
            init,
            |acc, (op, val): (char, i64)| {
                if op == '*' {
                    acc * val
                } else {
                    acc / val
                }
            },
        )(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 71; "d18 1")]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51; "3")]
    #[test_case("2 * 3 + (4 * 5)", 26; "4")]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437; "5")]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240; "6")]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632; "7")]
    fn test_part_1(expression: &str, result: i64) {
        assert_eq!(part_1::expr(expression), Ok(("", result)));
    }

    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 231)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[test_case("2 * 3 + (4 * 5)", 46)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)]
    fn test_part_2(input: &str, expected: i64) {
        assert_eq!(part_2::expr(input), Ok(("", expected)));
    }
}
