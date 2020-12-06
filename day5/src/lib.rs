#[cfg(test)]
mod tests {
    use super::*;
    use input;

    #[test]
    fn test_day5_example() {
        assert_eq!(357, find_seat("FBFBBFFRLR"));
        assert_eq!(567, find_seat("BFFFBBFRRR"));
        assert_eq!(119, find_seat("FFFBBBFRRR"));
        assert_eq!(820, find_seat("BBFFBBFRLL"));
    }

    #[test]
    fn test_day5_p1() {
        assert_eq!(822, input::file_lines("../data/day5-1.txt")
            .map(|l| find_seat(&l)).max().unwrap())
    }

    #[test]
    fn test_day5_p2() {
        let data = input::file_lines("../data/day5-1.txt")
            .map(|l| find_seat(&l))
            .collect::<Vec<u64>>();

        let min = *data.iter().min().unwrap();
        let max = *data.iter().max().unwrap() +1;
        assert_eq!(705, (min..max).find(|x| ! data.contains(x)).unwrap());
    }
}

fn up  (h: u64, l: u64) -> u64 { ((h - l) as f64 /2.0).ceil() as u64 + l }
fn down(h: u64, l: u64) -> u64 { ((h - l) /2) + l }

pub fn find_seat(path: &str) -> u64 {
    let (mut ly, mut lx, mut hy, mut hx) = (0, 0, 127, 7);
    for c in path.chars() {
        match c {
            'F' => hy = down(hy, ly),
            'B' => ly = up(hy, ly),
            'L' => hx = down(hx, lx),
            'R' => lx = up(hx, lx),
            _ => unreachable!()
        }
    }

    assert_eq!(lx, lx);
    assert_eq!(hy, ly);
    (ly * 8) + lx
}
