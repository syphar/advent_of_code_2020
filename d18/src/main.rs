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
use std::str::FromStr;

use simple_error::SimpleResult;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();

    let lines: Vec<String> = BufReader::new(file)
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("part 1: {:?}", part_1(&lines));
    println!("part 2: {:?}", part_2(&lines));
}

fn part_1(lines: &[String]) -> SimpleResult<u64> {
    let mut sum = 0;

    // for line in lines {
    //     let num = evaluate(&mut line.chars().peekable())?;
    //     sum += num;
    // }
    Ok(sum)
}

fn part_2(lines: &[String]) -> SimpleResult<i64> {
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

fn expr(i: &str) -> IResult<&str, i64> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // #[test_case(""; "no result")]
    // #[test_case("-1"; "operator at the beginning")]
    // #[test_case("123 123"; "without operator")]
    // #[test_case("+"; "operator alone")]
    // #[test_case("123 ++ 123"; "double operator")]
    // #[test_case("123+321 +"; "unexpected operator at the end")]
    // #[test_case("(1+2"; "simple braces, missing close")]
    // fn test_evaluate_errors(expression: &str) {
    //     assert!(evaluate(&mut expression.chars().peekable()).is_err());
    // }

    // #[test_case("123", 123)]
    // #[test_case("321", 321)]
    // #[test_case(" 123 ", 123; "123 whitespace")]
    // #[test_case(" 321 ", 321; "321 whitespace")]
    // #[test_case(" 123 + 123 ", 123+123)]
    // #[test_case(" 123 + 123 + 321", 123+123+321)]
    // #[test_case("1 + 2 * 3 + 4 * 5 + 6", 71; "d18 1")]
    // #[test_case("(1+2)", 1+2; "simple braces")]
    // #[test_case("1 + (1+2)", 1 + (1+2); "simple braces 2")]
    // #[test_case("1 + (2*3)", 7; "simple braces 3")]
    // #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51; "3")]
    // #[test_case("2 * 3 + (4 * 5)", 26; "4")]
    // #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437; "5")]
    // #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240; "6")]
    // #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632; "7")]
    // fn test_evaluate(expression: &str, result: u64) {
    //     assert_eq!(evaluate(&mut expression.chars().peekable()), Ok(result));
    // }
    //

    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 231)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[test_case("2 * 3 + (4 * 5)", 46)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)]
    fn test_part_2(input: &str, expected: i64) {
        assert_eq!(expr(input), Ok(("", expected)));
    }
}
