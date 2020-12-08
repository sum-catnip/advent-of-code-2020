use std::convert::AsRef;
use std::collections::{ HashMap, HashSet };
use lazy_static::lazy_static;
use regex::Regex;

#[cfg(test)]
mod tests {
    use super::*;
    use input;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_d7_p1_example() {
        let data: Vec<String> = input::file_lines("../data/day7-1-test.txt").collect();
        let data: Vec<&str> = data.iter().map(AsRef::as_ref).collect();
        assert_eq!(4, count_outers(data, "shiny gold bag"));
    }

    #[test]
    fn test_d7_p1() {
        let data: Vec<String> = input::file_lines("../data/day7-1.txt").collect();
        let data: Vec<&str> = data.iter().map(AsRef::as_ref).collect();
        assert_eq!(139, count_outers(data, "shiny gold bag"));
    }

    #[test]
    fn test_d7_p2_example() {
        let data: Vec<String> = input::file_lines("../data/day7-2-test.txt").collect();
        let data: Vec<&str> = data.iter().map(AsRef::as_ref).collect();
        assert_eq!(126, count_inners(data, "shiny gold bag"));
    }

    #[test]
    fn test_d7_p2() {
        let data: Vec<String> = input::file_lines("../data/day7-1.txt").collect();
        let data: Vec<&str> = data.iter().map(AsRef::as_ref).collect();
        assert_eq!(58175, count_inners(data, "shiny gold bag"));
    }
}

lazy_static! {
    static ref RE_IO: Regex = Regex::new(r#"(?P<outer>.*?)s?\scontain\s(?P<inner>.*?)\."#)
        .expect("regex oofed");

    static ref RE_INNER: Regex = Regex::new(r#"(?P<amount>\d|no)\s(?P<name>.*?)s?(,|$)"#)
        .expect("regex oofed");
}

type Inner<'a> = Vec<(usize, &'a str)>;
type Rules<'a> = HashMap<&'a str, Inner<'a>>;
type Out<'a> = HashSet<&'a str>;

fn parse_line<'a>(line: &'a str) -> (&'a str, Inner<'a>) {
    let io = RE_IO.captures(line).unwrap();
    let outer = io.name("outer").unwrap().as_str();
    let inner = io.name("inner").unwrap().as_str();

    let inner = RE_INNER
        .captures_iter(inner)
        .map(|m| (m.name("amount").unwrap().as_str().parse().unwrap_or(0),
                  m.name("name").unwrap().as_str()))
        .collect();

    (outer, inner)
}

fn parse_rules<'a>(rules: Vec<&'a str>) -> Rules {
    rules.iter().map(|p| parse_line(p)).collect()
}

fn find_recurse_out<'a>(rules: &Rules<'a>, target: &str, out: &mut Out<'a>) {
    for b in rules.iter().filter(|(_, v)| v.iter().any(|(_, n)| n == &target)) {
        out.insert(b.0);
        find_recurse_out(rules, b.0, out);
    }
}

fn count_recurse_in<'a>(rules: &Rules<'a>, target: &str) -> usize {
    rules
        .get(target)
        .map(|b| b.iter()
             .fold(1, |a, e| (e.0 * count_recurse_in(rules, e.1)) + a))
        .unwrap_or(0)
}

pub fn count_outers(rules: Vec<&str>, target: &str) -> usize {
    let data = parse_rules(rules);
    let mut out = HashSet::new();
    find_recurse_out(&data, target, &mut out);
    out.iter().count()
}

pub fn count_inners(rules: Vec<&str>, target: &str) -> usize {
    count_recurse_in(&parse_rules(rules), target) -1
}
