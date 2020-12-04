#![feature(str_split_once)]

use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    use input;

    fn fields() -> Vec<&'static str> {
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    }

    #[test]
    fn test_part1_example() {
        assert_eq!(2, input::file_split("../data/day4-1-test.txt", "\n\n")
            .iter()
            .filter(|p| validate_passport(p, &fields()))
            .count());
    }

    #[test]
    fn test_part1() {
        assert_eq!(206, input::file_split("../data/day4-1.txt", "\n\n")
            .iter()
            .filter(|p| validate_passport(p, &fields()))
            .count());
    }

    #[test]
    fn test_part2_test_valid() {
        assert!(input::file_split("../data/day4-2-valid.txt", "\n\n")
            .iter()
            .all(|p| validate_passport2(p, &fields())));
    }

    #[test]
    fn test_part2_test_invalid() {
        assert!(input::file_split("../data/day4-2-invalid.txt", "\n\n")
            .iter()
            .all(|p| ! validate_passport2(p, &fields())));
    }

    #[test]
    fn test_part2() {
        assert_eq!(123, input::file_split("../data/day4-1.txt", "\n\n")
            .iter()
            .filter(|p| validate_passport2(p, &fields()))
            .count());
    }
}

lazy_static! {
    static ref RE_HEX: Regex = Regex::new("^#[0-9a-f]{6}$")
        .expect("regex oofed");

    static ref RE_COL: Regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$")
        .expect("regex oofed");

    static ref RE_PID: Regex = Regex::new("^\\d{9}$")
        .expect("regex oofed");
}

fn parse_passport<'a>(pp: &'a str) -> HashMap<&'a str, &'a str> {
    pp.split(&[' ', '\n'][..])
        .take_while(|p| p.chars().any(|_| true))
        .map(|kv| kv.split_once(':').unwrap())
        .collect()
}

pub fn validate_passport(pp: &str, fields: &[&str]) -> bool {
    let passports = parse_passport(pp);
    fields.iter().all(|f| passports.contains_key(f))
}

fn validate_yr(val: &str, min: u32, max: u32) -> bool {
    val.parse::<u32>().map_or(false, |p| p >= min && p <= max)
}

fn validate_hgt(val: &str) -> bool {
    match val {
        val if val.ends_with("cm") => 
            val.replace("cm", "")
               .parse::<u8>()
               .map_or(false, |h| h >= 150 && h <= 193),

        val if val.ends_with("in") =>
            val.replace("in", "")
               .parse::<u8>()
               .map_or(false, |h| h >= 59 && h <= 76),

        _ => false
    }
}

pub fn validate_passport2(pp: &str, fields: &[&str]) -> bool {
    let pp = parse_passport(pp);
    fields.iter().all(|f| pp.contains_key(f))
        && validate_yr(pp["byr"], 1920, 2002)
        && validate_yr(pp["iyr"], 2010, 2020)
        && validate_yr(pp["eyr"], 2020, 2030)
        && validate_hgt(pp["hgt"])
        && RE_HEX.is_match(pp["hcl"])
        && RE_COL.is_match(pp["ecl"])
        && RE_PID.is_match(pp["pid"])
}
