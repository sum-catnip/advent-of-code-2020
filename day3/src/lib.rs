use std::iter::Iterator;

#[cfg(test)]
mod tests {
    use super::*;
    use input;

    #[test]
    fn test_part1_example() {
        assert_eq!(7, count_trees(input::file_lines("../data/day3-1-test.txt"), (3, 1)));
    }

    #[test]
    fn test_part1() {
        assert_eq!(250, count_trees(input::file_lines("../data/day3-1.txt"), (3, 1)));
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(2, count_trees(input::file_lines("../data/day3-1-test.txt"), (1, 1)));
        assert_eq!(7, count_trees(input::file_lines("../data/day3-1-test.txt"), (3, 1)));
        assert_eq!(3, count_trees(input::file_lines("../data/day3-1-test.txt"), (5, 1)));
        assert_eq!(4, count_trees(input::file_lines("../data/day3-1-test.txt"), (7, 1)));
        assert_eq!(2, count_trees(input::file_lines("../data/day3-1-test.txt"), (1, 2)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1592662500, [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
            .map(|x| count_trees(input::file_lines("../data/day3-1.txt"), *x))
            .fold(1, |x, y| x * y));
    }
}

pub fn count_trees(map: impl Iterator<Item = String>, slope: (usize, usize)) -> usize {
    let (stepx, stepy) = slope;
    let mut xpos = 0;
    map.skip(stepy).step_by(stepy).filter(|col| {
        xpos += stepx;
        col.chars().nth(xpos % col.chars().count()).unwrap() == '#'
    }).count()
}
