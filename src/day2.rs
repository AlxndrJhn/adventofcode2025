fn is_valid_id_twice(candidate: &str) -> bool {
    let check_length = candidate.len() / 2;
    if &candidate[0..check_length] == &candidate[check_length..] {
        return false;
    }

    true
}

fn is_valid_id_at_least_twice(candidate: &str) -> bool {
    for check_length in 1..=candidate.len() / 2 {
        if candidate.len() % check_length != 0 {
            continue;
        }
        let expected_pattern = &candidate[0..check_length];
        let mut matched = true;
        for expected_times in 1..(candidate.len() / check_length) {
            let start = expected_times * check_length;
            let end = start + check_length;
            if &candidate[start..end] != expected_pattern {
                matched = false;
            }
        }
        if matched {
            return false;
        }
    }
    true
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|range| {
            let range = range.trim();
            let mut bounds = range.split('-').map(|n| n.parse::<usize>().unwrap());
            let start = bounds.next().unwrap();
            let end = bounds.next().unwrap();
            let mut invalid_ids = 0;
            for candidate in start..=end {
                if !is_valid_id_twice(&candidate.to_string()) {
                    invalid_ids += candidate;
                }
            }
            invalid_ids
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|range| {
            let range = range.trim();
            let mut bounds = range.split('-').map(|n| n.parse::<usize>().unwrap());
            let start = bounds.next().unwrap();
            let end = bounds.next().unwrap();
            let mut invalid_ids = 0;
            for candidate in start..=end {
                if !is_valid_id_at_least_twice(&candidate.to_string()) {
                    invalid_ids += candidate;
                }
            }
            invalid_ids
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"
            ),
            1227775554
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"
            ),
            4174379265
        );
    }
}
