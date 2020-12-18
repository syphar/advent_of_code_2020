#[macro_use]
extern crate simple_error;
extern crate nom;

use core::iter::Peekable;
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

mod part_1 {
    use super::*;

    pub fn run(lines: &[String]) -> SimpleResult<u64> {
        let mut sum = 0;

        for line in lines {
            let num = evaluate(&mut line.chars().peekable())?;
            sum += num;
        }
        Ok(sum)
    }

    #[derive(Debug, Clone)]
    enum Operator {
        Sum,
        Product,
    }

    fn calculate_result(
        current_result: &Option<u64>,
        current_operator: &Option<Operator>,
        number: u64,
    ) -> SimpleResult<u64> {
        if let Some(r) = current_result {
            match current_operator {
                None => {
                    bail!("unexpected number");
                }
                Some(Operator::Sum) => {
                    return Ok(r + number);
                }
                Some(Operator::Product) => {
                    return Ok(r * number);
                }
            }
        } else {
            return Ok(number);
        }
    }

    pub fn evaluate<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> SimpleResult<u64> {
        let mut current_result: Option<u64> = None;
        let mut current_operator: Option<Operator> = None;

        while let Some(&c) = it.peek() {
            match c {
                ' ' => {
                    it.next();
                }
                '0'..='9' => {
                    it.next();
                    let mut number = c
                        .to_string()
                        .parse::<u64>()
                        .expect("The caller should have passed a digit.");

                    while let Some(Ok(digit)) = it.peek().map(|c| c.to_string().parse::<u64>()) {
                        number = number * 10 + digit;
                        it.next();
                    }

                    current_result = Some(calculate_result(
                        &current_result,
                        &current_operator,
                        number,
                    )?);
                    current_operator = None;
                }
                '+' | '*' => {
                    if let Some(_) = current_operator {
                        bail!("unexpected operator");
                    } else {
                        it.next();

                        current_operator = Some(match c {
                            '+' => Operator::Sum,
                            '*' => Operator::Product,
                            _ => bail!("this should not happen"),
                        });
                    }
                }
                '(' => {
                    it.next();
                    let number = evaluate(it)?;

                    if let Some(&ch) = it.peek() {
                        if ch != ')' {
                            bail!("missing ')'");
                        } else {
                            it.next();
                            current_result = Some(calculate_result(
                                &current_result,
                                &current_operator,
                                number,
                            )?);
                            current_operator = None;
                        }
                    }
                }
                ')' => {
                    break;
                }
                _ => {
                    bail!("unexpected character: {}", c);
                }
            }
        }

        if let Some(value) = current_result {
            if let Some(_) = current_operator {
                bail!("unexpected operator")
            } else {
                Ok(value)
            }
        } else {
            bail!("no result!")
        }
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

    #[test_case(""; "no result")]
    #[test_case("-1"; "operator at the beginning")]
    #[test_case("123 123"; "without operator")]
    #[test_case("+"; "operator alone")]
    #[test_case("123 ++ 123"; "double operator")]
    #[test_case("123+321 +"; "unexpected operator at the end")]
    #[test_case("(1+2"; "simple braces, missing close")]
    fn test_evaluate_errors(expression: &str) {
        assert!(part_1::evaluate(&mut expression.chars().peekable()).is_err());
    }

    #[test_case("123", 123)]
    #[test_case("321", 321)]
    #[test_case(" 123 ", 123; "123 whitespace")]
    #[test_case(" 321 ", 321; "321 whitespace")]
    #[test_case(" 123 + 123 ", 123+123)]
    #[test_case(" 123 + 123 + 321", 123+123+321)]
    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 71; "d18 1")]
    #[test_case("(1+2)", 1+2; "simple braces")]
    #[test_case("1 + (1+2)", 1 + (1+2); "simple braces 2")]
    #[test_case("1 + (2*3)", 7; "simple braces 3")]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51; "3")]
    #[test_case("2 * 3 + (4 * 5)", 26; "4")]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437; "5")]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240; "6")]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632; "7")]
    fn test_evaluate(expression: &str, result: u64) {
        assert_eq!(
            part_1::evaluate(&mut expression.chars().peekable()),
            Ok(result)
        );
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
