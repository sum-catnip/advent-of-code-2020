use std::collections::{ HashSet, HashMap };

#[cfg(test)]
mod tests {
    use super::*;
    use input;

    #[test]
    fn test_d6_p1_example() {
        assert_eq!(3, count_answers("abc"));
        assert_eq!(3, count_answers("a\nb\nc"));
        assert_eq!(3, count_answers("ab\nac"));
        assert_eq!(1, count_answers("a\na\na\na"));
        assert_eq!(1, count_answers("b"));
    }

    #[test]
    fn test_d6_p1() {
        assert_eq!(6457usize, input::file_split("../data/day6-1.txt", "\n\n")
                   .iter()
                   .map(|g| count_answers(g))
                   .sum())
    }

    #[test]
    fn test_d6_p2_example() {
        assert_eq!(3, count_answers2("abc"));
        assert_eq!(0, count_answers2("a\nb\nc"));
        assert_eq!(1, count_answers2("ab\nac"));
        assert_eq!(1, count_answers2("a\na\na\na"));
        assert_eq!(1, count_answers2("b"));
    }

    #[test]
    fn test_d6_p2() {
        assert_eq!(3260usize, input::file_split("../data/day6-1.txt", "\n\n")
                   .iter()
                   .map(|g| count_answers2(g))
                   .sum())
    }
}

pub fn count_answers(group: &str) -> usize {
    group.lines()
         .flat_map(|x| x.chars())
         .collect::<HashSet<char>>()
         .len()
}

pub fn count_answers2(group: &str) -> usize {
    let mut counts = HashMap::new();
    let members = group.lines().count();

    group.lines()
         .flat_map(|x| x.chars())
         .for_each(|c| *counts.entry(c).or_insert(0) += 1);

    counts.values()
        .filter(|v| **v == members)
        .count()
}
