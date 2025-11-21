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

#[aoc(day20, part2)]
pub fn part2(input: &str) -> usize {
    let mut blocked_ranges: Vec<(u128, u128)> = Vec::new();
    for line in input.lines() {
        let parts = line
            .split('-')
            .map(|s| s.parse::<u128>().unwrap())
            .collect::<Vec<u128>>();
        blocked_ranges.push((parts[0], parts[1]));
    }
    blocked_ranges.sort_by_key(|k| k.0);
    let mut allowed_count = 0;
    let mut current = 0;
    for (start, end) in blocked_ranges {
        if start > current {
            allowed_count += start - current;
            current = end + 1;
        } else if end + 1 > current {
            current = end + 1;
        }
    }
    allowed_count as usize
}

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
