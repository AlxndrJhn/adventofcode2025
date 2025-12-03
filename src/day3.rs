#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut maximum = 0;
            for i in 0..(bytes.len() - 1) {
                let byte_i = bytes[i] - b'0';
                for j in (i + 1)..bytes.len() {
                    let byte_j = bytes[j] - b'0';
                    maximum = maximum.max((byte_i * 10 + byte_j) as usize);
                }
            }
            maximum
        })
        .sum()
}

fn dfs(
    _bytes: &[u8],
    _index: usize,
    count_on: usize,
    _current_number: usize,
    _maximum: &mut usize,
) {
    let remaining = _bytes.len() - _index;
    if count_on + remaining < 12 {
        return;
    }
    if count_on == 12 {
        *_maximum = (*_maximum).max(_current_number);
        return;
    }
    let factor = 10_usize.pow((12 - count_on - 1) as u32);
    if count_on > 0 && *_maximum > 0 && _current_number < *_maximum - factor * 9 {
        return;
    }
    let byte = _bytes[_index] - b'0';
    if count_on < 12 {
        dfs(
            _bytes,
            _index + 1,
            count_on + 1,
            _current_number + (byte as usize) * factor,
            _maximum,
        );
    }
    dfs(_bytes, _index + 1, count_on, _current_number, _maximum);
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let bytes = line.as_bytes();
            let mut maximum = 0;
            dfs(&bytes, 0, 0, 0, &mut maximum);
            maximum
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
                "987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            357
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("234234234234278"), 78);
    }
    #[test]
    fn example_3() {
        assert_eq!(
            part2(
                "987654321111111
811111111111119
234234234234278
818181911112111"
            ),
            3121910778619
        );
    }
    #[test]
    fn example_4() {
        assert_eq!(part2("987654321111111"), 987654321111);
    }
    #[test]
    fn example_5() {
        assert_eq!(part2("811111111111119"), 811111111119);
    }
    #[test]
    fn example_6() {
        assert_eq!(part2("234234234234278"), 434234234278);
    }
}
