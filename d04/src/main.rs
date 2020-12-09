#![feature(str_split_once)]
#![feature(custom_test_frameworks)]

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
extern crate test_case;

use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use validator::{Validate, ValidationError};

lazy_static! {
    static ref RE_PASSPORT_ID: Regex = Regex::new(r"^\d{9}$").unwrap();
    static ref RE_COLOR_CODE: Regex = Regex::new(r"^#[a-fA-F0-9]{6}$").unwrap();
    static ref RE_EYE_COLOR: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref RE_HEIGHT: Regex = Regex::new(r"^\d{2,3}(in|cm)$").unwrap();
}

#[derive(Debug, Validate)]
struct Passport {
    // (Birth Year) - four digits; at least 1920 and at most 2002.
    #[validate(range(min = 1920, max = 2002))]
    byr: u16,

    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    #[validate(range(min = 2010, max = 2020))]
    iyr: u16,

    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    #[validate(range(min = 2020, max = 2030))]
    eyr: u16,

    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    #[validate(
        length(min = 4, max = 5),
        regex = "RE_HEIGHT",
        custom = "validate_height_value"
    )]
    hgt: String,

    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    #[validate(regex = "RE_COLOR_CODE")]
    hcl: String,

    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    #[validate(regex = "RE_EYE_COLOR")]
    ecl: String,

    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    #[validate(regex = "RE_PASSPORT_ID")]
    pid: String,
}

fn validate_height_value(hgt: &str) -> Result<(), ValidationError> {
    let split = hgt.len() - 2;
    let unit = &hgt[split..];
    let height: u16 = (&hgt[..split])
        .parse()
        .map_err(|_| ValidationError::new("no number"))?;

    match unit {
        "cm" => match height {
            150..=193 => Ok(()),
            _ => Err(ValidationError::new("wrong cm size")),
        },
        "in" => match height {
            59..=76 => Ok(()),
            _ => Err(ValidationError::new("wrong in size")),
        },
        _ => return Err(ValidationError::new("unknown unit")),
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let input: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    println!(
        "all with fields: {:?}\nvalidated: {:?}",
        doit(input.iter(), false),
        doit(input.iter(), true)
    );
}

fn get_passport(data: &str) -> Option<Passport> {
    let fields: HashMap<_, _> = data
        .split_whitespace()
        .filter_map(|e| e.split_once(":"))
        .collect();

    Some(Passport {
        byr: fields.get("byr")?.parse().unwrap(),
        iyr: fields.get("iyr")?.parse().unwrap(),
        eyr: fields.get("eyr")?.parse().unwrap(),
        hgt: fields.get("hgt")?.parse().unwrap(),
        hcl: fields.get("hcl")?.parse().unwrap(),
        ecl: fields.get("ecl")?.parse().unwrap(),
        pid: fields.get("pid")?.parse().unwrap(),
    })
}

fn split_passports<'a>(lines: impl Iterator<Item = &'a String>) -> Vec<String> {
    let mut result = vec![];

    let mut current_passport = String::new();

    for line in lines {
        if line.trim().len() == 0 {
            result.push(current_passport);
            current_passport = String::new();
        }

        current_passport.push_str(" ");
        current_passport.push_str(&line);
    }

    if current_passport.trim().len() > 0 {
        result.push(current_passport);
    }
    result
}

pub fn doit<'a>(lines: impl Iterator<Item = &'a String>, validate: bool) -> usize {
    let mut result = 0;

    for p in split_passports(lines) {
        result += match get_passport(&p) {
            Some(p) => match validate {
                true => match p.validate() {
                    Ok(_) => 1,
                    Err(_) => 0,
                },
                false => 1,
            },
            None => 0,
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    lazy_static! {
        static ref TEST_DATA_1: Vec<String> = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
            "byr:1937 iyr:2017 cid:147 hgt:183cm",
            "",
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
            "hcl:#cfa07d byr:1929",
            "",
            "hcl:#ae17e1 iyr:2013",
            "eyr:2024",
            "ecl:brn pid:760753108 byr:1931",
            "hgt:179cm",
            "",
            "hcl:#cfa07d eyr:2025 pid:166559648",
            "iyr:2011 ecl:brn hgt:59in",
        ]
        .into_iter()
        .map(|line| line.to_string())
        .collect();
        static ref TEST_DATA_2: Vec<String> = vec![
            "eyr:1972 cid:100",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
            "",
            "iyr:2019",
            "hcl:#602927 eyr:1967 hgt:170cm",
            "ecl:grn pid:012533040 byr:1946",
            "",
            "hcl:dab227 iyr:2012",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
            "",
            "hgt:59cm ecl:zzz",
            "eyr:2038 hcl:74454a iyr:2023",
            "pid:3556412378 byr:2007",
            "",
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
            "hcl:#623a2f",
            "",
            "eyr:2029 ecl:blu cid:129 byr:1989",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
            "",
            "hcl:#888785",
            "hgt:164cm byr:2001 iyr:2015 cid:88",
            "pid:545766238 ecl:hzl",
            "eyr:2022",
            "",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
        ]
        .into_iter()
        .map(|line| line.to_string())
        .collect();
    }

    #[test]
    fn it_works() {
        assert_eq!(doit(TEST_DATA_1.iter(), false), 2);
    }

    #[test]
    fn it_works2() {
        assert_eq!(doit(TEST_DATA_2.iter(), true), 4);
    }

    #[test_case("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\nhcl:#623a2f" ; "p1")]
    #[test_case("eyr:2029 ecl:blu cid:129 byr:1989\niyr:2014 pid:896056539 hcl:#a97842 hgt:165cm" ; "p2")]
    #[test_case("hcl:#888785\nhgt:164cm byr:2001 iyr:2015 cid:88\npid:545766238 ecl:hzl\neyr:2021" ; "p3")]
    #[test_case("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719" ; "p4")]
    fn valid_passport(passport_string: &str) {
        let p = get_passport(passport_string).unwrap();
        assert_eq!(p.validate(), Ok(()));
    }

    #[test_case("eyr:1972 cid:100\nhcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926" ; "p1")]
    #[test_case("iyr:2019\nhcl:#602927 eyr:1967 hgt:170cm\necl:grn pid:012533040 byr:1946" ; "p2")]
    #[test_case("hcl:dab227 iyr:2012\necl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"  ; "p3")]
    #[test_case("hgt:59cm ecl:zzz\neyr:2038 hcl:74454a iyr:2023\npid:3556412378 byr:2007" ; "p4")]
    fn invalid_passport(passport_string: &str) {
        let p = get_passport(passport_string).unwrap();
        assert!(p.validate().is_err());
    }
}
