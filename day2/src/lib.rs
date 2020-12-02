use regex::Regex;
use lazy_static::lazy_static;

#[cfg(test)]
mod tests {
    use super::*;
    use input;

    #[test]
    fn test_part1() {
        let data = input::file_lines("../data/day2-1.txt");
        assert_eq!(548, data.filter(|l| validate_pw(l.as_str())).count());
    }

    #[test]
    fn test_part2() {
        let data = input::file_lines("../data/day2-1.txt");
        assert!(validate_pw2("1-3 a: abcde"));
        assert!(!validate_pw2("1-3 b: cdefg"));
        assert!(!validate_pw2("2-9 c: ccccccccc"));
        assert_eq!(502, data.filter(|l| validate_pw2(l.as_str())).count());
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r#"(?P<from>\d+)-(?P<to>\d+)\s(?P<ch>\w):\s(?P<pw>.*)"#)
        .expect("regex oofed");
}

struct Policy { from: usize, to: usize, ch: char }

fn parse_line(line: &str) -> (Policy, &str) {
    let cap = RE.captures(line).expect("malformed input");
    (Policy {
        from: cap.name("from").expect("malformed input: Policy::from")
            .as_str().parse().expect("malformed input: Policy::from"),

        to: cap.name("to").expect("malformed input: Policy::to")
            .as_str().parse().expect("malformed input: Policy::to"),

        ch: cap.name("ch").expect("malformed input: Policy::ch")
            .as_str().chars().next().expect("malformed input: Policy::ch")

    }, cap.name("pw").expect("malformed input: pw").as_str())
}

pub fn validate_pw(line: &str) -> bool {
    let (pol, pw) = parse_line(line);
    let count = pw.matches(pol.ch).count();
    pol.from <= count && count <= pol.to
}

pub fn validate_pw2(line: &str) -> bool {
    let (pol, pw) = parse_line(line);
    (pw.chars().nth(pol.from -1).expect("invalid policy index") == pol.ch) ^
    (pw.chars().nth(pol.to -1).expect("invalid policy index") == pol.ch)
}
