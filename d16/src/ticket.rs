use lazy_static::lazy_static;
use regex::Regex;
use simple_error::SimpleError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub range_1: RangeInclusive<usize>,
    pub range_2: RangeInclusive<usize>,
}

impl Field {
    pub fn check(&self, input: &usize) -> bool {
        self.range_1.contains(input) || self.range_2.contains(input)
    }
}

lazy_static! {
    static ref REGEX_FIELD: Regex = Regex::new(r"([^:]+): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
}

impl FromStr for Field {
    type Err = SimpleError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Some(cap) = REGEX_FIELD.captures(input) {
            Ok(Field {
                name: cap.get(1).unwrap().as_str().to_string(),
                range_1: cap.get(2).unwrap().as_str().parse().unwrap()
                    ..=cap.get(3).unwrap().as_str().parse().unwrap(),
                range_2: cap.get(4).unwrap().as_str().parse().unwrap()
                    ..=cap.get(5).unwrap().as_str().parse().unwrap(),
            })
        } else {
            Err(SimpleError::new(format!(
                "could not parse command from {}",
                input
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("asdf")]
    #[test_case("asdf: b-b or b-b")]
    fn test_parse_field_error(input: &str) {
        assert!(input.parse::<Field>().is_err());
    }

    #[test_case(
        "class: 1-3 or 5-7", 
        Field {
            name: "class".to_string(), 
            range_1: 1..=3, 
            range_2: 5..=7
        }
    )]
    #[test_case(
        "field with whitespace: 999-9999 or 1-90007", 
        Field {
            name: "field with whitespace".to_string(), 
            range_1: 999..=9999, 
            range_2: 1..=90007
        }
    )]
    fn test_parse_field(input: &str, expected: Field) {
        assert_eq!(input.parse(), Ok(expected));
    }

    #[test_case(1, true)]
    #[test_case(10, true)]
    #[test_case(11, false)]
    #[test_case(20, true)]
    #[test_case(25, true)]
    #[test_case(30, true)]
    #[test_case(31, false)]
    fn test_check(value: usize, expected: bool) {
        let f = Field {
            name: "fff".to_string(),
            range_1: 1..=10,
            range_2: 20..=30,
        };

        assert_eq!(f.check(&value), expected);
    }
}
