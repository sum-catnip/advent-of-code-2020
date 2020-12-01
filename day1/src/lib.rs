#[cfg(test)]
mod tests {
    use super::*;
    use input;

    #[test]
    fn test_find_sum2() {
        let data = input::file_as_nums("../data/day1-1.txt");
        assert_eq!(Some(494475), find_sum2(&data, 2020));
    }

    #[test]
    fn test_find_sum3() {
        let data = input::file_as_nums("../data/day1-1.txt");
        assert_eq!(Some(267520550), find_sum3(&data, 2020));
    }
}

fn find_sum(data: &[u64], sum: u64) -> Option<(u64, u64)> {
    data.iter()
        .find_map(|x| data.iter().find(|y| x + *y == sum).map(|y| (*x, *y)))
}

pub fn find_sum2(data: &[u64], sum: u64) -> Option<u64> {
    find_sum(data, sum).map(|(x, y)| x * y)
}

pub fn find_sum3(data: &[u64], sum: u64) -> Option<u64> {
    data.iter()
        .find_map(|x| find_sum(data, sum - x).map(|(y, z)| (x, y, z)))
        .map(|(x, y, z)| x * y * z)
}
