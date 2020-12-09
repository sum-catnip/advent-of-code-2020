use day1;
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;
    use input;

    #[test]
    fn test_d9_p1_example() {
        let data = input::file_as_nums("../data/day9-1-test.txt");
        assert_eq!(Some(127), find_invalid(&data, 5));
    }

    #[test]
    fn test_d9_p1() {
        let data = input::file_as_nums("../data/day9-1.txt");
        assert_eq!(Some(756008079), find_invalid(&data, 25));
    }

    #[test]
    fn test_d9_p2_example() {
        let data = input::file_as_nums("../data/day9-1-test.txt");
        let invalid = find_invalid(&data, 5);
        assert_eq!(Some(127), invalid);
        let sumn = find_sumn(&data, invalid.unwrap()).unwrap();
        assert_eq!(62, sumn.iter().min().unwrap() + sumn.iter().max().unwrap())
    }

    #[test]
    fn test_d9_p2() {
        let data = input::file_as_nums("../data/day9-1.txt");
        let invalid = find_invalid(&data, 25);
        assert_eq!(Some(756008079), invalid);
        let sumn = find_sumn(&data, invalid.unwrap()).unwrap();
        assert_eq!(93727241, sumn.iter().min().unwrap() + sumn.iter().max().unwrap())
    }
}

fn validate_num(n: u64, pre: &[u64]) -> bool {
    day1::find_sum2(pre, n).is_some()
}

pub fn find_sumn(items: &[u64], n: u64) -> Option<&[u64]> {
    let (mut l, mut sum) = (0, 0);
    for (i, x) in items.iter().enumerate() {
        sum += x;
        while sum > n { sum -= items[l]; l += 1; }
        if sum == n { return Some(&items[l .. i +1]) }
    }
    None
}

pub fn find_invalid(cy: &[u64], preamble: usize) -> Option<u64> {
    cy.iter()
      .enumerate()
      .skip(preamble)
      .find(|(i, n)| !validate_num(**n, &cy[i-preamble..*i]))
      .map(|(_, n)| *n)
}
