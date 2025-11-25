use std::vec;

fn get_checksum(truncated: Vec<u8>) -> String {
    let mut checksum = truncated;
    while checksum.len() % 2 == 0 {
        let mut new_checksum = vec![];
        for i in (0..checksum.len()).step_by(2) {
            if checksum[i] == checksum[i + 1] {
                new_checksum.push(1);
            } else {
                new_checksum.push(0);
            }
        }
        checksum = new_checksum;
    }
    checksum
        .into_iter()
        .map(|b| if b == 0 { '0' } else { '1' })
        .collect()
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> String {
    let converted: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| if c == '0' { 0 } else { 1 })
        .collect();
    const DISK_SIZE: usize = 272;
    let mut current = converted.clone();
    while current.len() < DISK_SIZE {
        current = one_step(&current);
    }
    let truncated: Vec<u8> = current.into_iter().take(DISK_SIZE).collect();
    // checksum
    get_checksum(truncated)
}

fn one_step(a: &Vec<u8>) -> Vec<u8> {
    let mut b = a.clone();
    b.reverse();
    for i in 0..b.len() {
        b[i] = 1 - b[i];
    }
    let mut result = a.clone();
    result.push(0);
    result.extend(b);
    result
}
#[aoc(day16, part2)]
pub fn part2(input: &str) -> String {
    let converted: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| if c == '0' { 0 } else { 1 })
        .collect();
    const DISK_SIZE: usize = 35651584;
    let mut current = converted.clone();
    while current.len() < DISK_SIZE {
        current = one_step(&current);
    }
    let truncated: Vec<u8> = current.into_iter().take(DISK_SIZE).collect();
    get_checksum(truncated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(one_step(&vec![1]), vec![1, 0, 0]);
    }
    #[test]
    fn example_2() {
        assert_eq!(one_step(&vec![0]), vec![0, 0, 1]);
    }
    #[test]
    fn example_3() {
        assert_eq!(
            one_step(&vec![1, 1, 1, 1, 1]),
            vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]
        );
    }
}
