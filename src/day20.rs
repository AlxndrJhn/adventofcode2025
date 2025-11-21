#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let mut blocked_ranges: Vec<(u128, u128)> = Vec::new();
    for line in input.lines() {
        let parts = line
            .split('-')
            .map(|s| s.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        blocked_ranges.push((parts[0], parts[1]));
    }
    for i in 0.. {
        if !blocked_ranges
            .iter()
            .any(|(start, end)| i >= *start && i <= *end)
        {
            return i as usize;
        }
    }
    0
}

// #[aoc(day20, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "5-8
0-2
4-7"
            ),
            3
        );
    }
}
