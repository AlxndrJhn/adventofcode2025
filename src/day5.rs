#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.trim();
    let ranges = input
        .lines()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let mut parts = line.split('-');
            let start = parts.next()?.parse::<usize>().ok()?;
            let end = parts.next()?.parse::<usize>().ok()?;
            Some((start, end))
        })
        .collect::<Vec<(usize, usize)>>();
    let numbers = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .filter_map(|line| line.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    let mut fresh_ingredients = 0;
    for &number in &numbers {
        for &(start, end) in &ranges {
            if number >= start && number <= end {
                fresh_ingredients += 1;
                break;
            }
        }
    }
    fresh_ingredients
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.trim();
    let ranges = input
        .lines()
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let mut parts = line.split('-');
            let start = parts.next()?.parse::<usize>().ok()?;
            let end = parts.next()?.parse::<usize>().ok()?;
            Some((start, end))
        })
        .collect::<Vec<(usize, usize)>>();

    let non_overlapping_ranges = {
        let mut sorted_ranges = ranges.clone();
        sorted_ranges.sort_by_key(|&(start, _)| start);
        let mut merged_ranges = Vec::new();
        for (start, end) in sorted_ranges {
            if let Some((_last_start, last_end)) = merged_ranges.last_mut() {
                if start <= *last_end {
                    *last_end = (*last_end).max(end);
                } else {
                    merged_ranges.push((start, end));
                }
            } else {
                merged_ranges.push((start, end));
            }
        }
        merged_ranges
    };

    non_overlapping_ranges
        .iter()
        .map(|(start, end)| end - start + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            3
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            ),
            14
        );
    }
}
