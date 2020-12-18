#[macro_use]
extern crate simple_error;
use core::iter::Peekable;
use simple_error::SimpleResult;

fn main() {
    println!("hello world");
}

#[derive(Debug, Clone)]
enum Operator {
    Sum,
    Product,
}

fn evaluate<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> SimpleResult<u64> {
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

                if let Some(r) = current_result {
                    match current_operator {
                        None => {
                            bail!("unexpected number");
                        }
                        Some(Operator::Sum) => {
                            current_result = Some(r + number);
                            current_operator = None;
                        }
                        Some(Operator::Product) => {
                            current_result = Some(r * number);
                            current_operator = None;
                        }
                    }
                } else {
                    current_result = Some(number);
                }
            }
            '+' | '*' => {
                if let Some(_) = current_operator {
                    bail!("unexpected operator");
                } else {
                    if let Some(_) = current_operator {
                        bail!("unexpected operator after operator");
                    } else {
                        it.next();

                        current_operator = Some(match c {
                            '+' => Operator::Sum,
                            '*' => Operator::Product,
                            _ => bail!("this should not happen"),
                        });
                    }
                }
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
    fn test_evaluate_errors(expression: &str) {
        assert!(evaluate(&mut expression.chars().peekable()).is_err());
    }

    #[test_case("123", 123)]
    #[test_case("321", 321)]
    #[test_case(" 123 ", 123; "123 whitespace")]
    #[test_case(" 321 ", 321; "321 whitespace")]
    #[test_case(" 123 + 123 ", 123+123)]
    #[test_case(" 123 + 123 + 321", 123+123+321)]
    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 71; "d18 1")]
    // #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51; "3")]
    // #[test_case("2 * 3 + (4 * 5)", 26; "4")]
    // #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437; "5")]
    // #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240; "6")]
    // #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632; "7")]
    fn test_evaluate(expression: &str, result: u64) {
        assert_eq!(evaluate(&mut expression.chars().peekable()), Ok(result));
    }
}
