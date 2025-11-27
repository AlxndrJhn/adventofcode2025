#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut sum: usize = 0;
    let digits: Vec<u32> = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    for i in 0..digits.len() - 1 {
        if digits[i] == digits[i + 1] {
            sum += digits[i] as usize;
        }
    }
    if digits[0] == digits[digits.len() - 1] {
        sum += digits[0] as usize;
    }
    sum
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut sum: usize = 0;
    let digits: Vec<u32> = input
        .trim()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    let half_len = digits.len() / 2;
    for i in 0..digits.len() {
        if digits[i] == digits[(i + half_len) % digits.len()] {
            sum += digits[i] as usize;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(part1("1122"), 3);
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("1111"), 4);
    }
    #[test]
    fn example_3() {
        assert_eq!(part2("1212"), 6);
    }
    #[test]
    fn example_4() {
        assert_eq!(part2("1221"), 0);
    }
}
