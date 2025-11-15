pub fn get_value_at_position(row: usize, col: usize) -> usize {
    let mut value = 20151125;
    if row == 1 && col == 1 {
        return value;
    }
    for diag in 2..=(row + col) {
        let mut r = diag;
        let mut c = 1;

        while r > 0 {
            value = (value * 252533) % 33554393;
            if r == row && c == col {
                return value;
            }
            r -= 1;
            c += 1;
        }
    }
    value
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> usize {
    let mut row: usize = 0;
    let mut col: usize = 0;
    let regex = regex::Regex::new(r"row (\d+), column (\d+)").unwrap();
    for cap in regex.captures_iter(input) {
        row = cap[1].parse().unwrap();
        col = cap[2].parse().unwrap();
    }
    get_value_at_position(row, col)
}

// #[aoc(day25, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(get_value_at_position(1, 1), 20151125);
    }

    #[test]
    fn example_2() {
        assert_eq!(get_value_at_position(2, 1), 31916031);
    }

    #[test]
    fn example_3() {
        assert_eq!(get_value_at_position(2, 2), 21629792);
    }

    #[test]
    fn example_4() {
        assert_eq!(get_value_at_position(6, 6), 27995004);
    }
}
