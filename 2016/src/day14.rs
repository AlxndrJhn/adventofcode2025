use std::collections::HashMap;

fn get_triplet(s: &str) -> Option<char> {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 2 {
        if chars[i] == chars[i + 1] && chars[i] == chars[i + 2] {
            return Some(chars[i]);
        }
    }
    None
}

fn contains_quintuplet(s: &str, c: char) -> bool {
    let target = vec![c; 5];
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 4 {
        if chars[i..i + 5] == target[..] {
            return true;
        }
    }
    false
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    let mut keys_found = 0;
    for added_number in 0.. {
        let mut hash = md5::Context::new();
        hash.consume(input);
        hash.consume(&added_number.to_string());
        let hex_str = format!("{:x}", hash.finalize());
        let triplet = get_triplet(&hex_str);
        if triplet.is_none() {
            continue;
        }
        let triplet = triplet.unwrap();
        for future_number in added_number + 1..=added_number + 1001 {
            let mut future_hash = md5::Context::new();
            future_hash.consume(input);
            future_hash.consume(&future_number.to_string());
            let future_hex_str = format!("{:x}", future_hash.finalize());
            if contains_quintuplet(&future_hex_str, triplet) {
                keys_found += 1;
                if keys_found == 64 {
                    return added_number;
                }
            }
        }
    }
    unreachable!()
}

fn stretched_hash(input: &str, index: usize, memo: &mut HashMap<usize, String>) -> String {
    if let Some(cached) = memo.get(&index) {
        return cached.clone();
    }
    let mut hash = md5::Context::new();
    hash.consume(input);
    hash.consume(&index.to_string());
    let mut hex_str = format!("{:x}", hash.finalize());
    for _ in 0..2016 {
        let mut new_hash = md5::Context::new();
        new_hash.consume(&hex_str);
        hex_str = format!("{:x}", new_hash.finalize());
    }
    memo.insert(index, hex_str.clone());
    hex_str
}
#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    let input = input.trim();
    let mut keys_found = 0;
    let mut memo = HashMap::new();
    for added_number in 0.. {
        let hex_str = stretched_hash(input, added_number, &mut memo);
        let triplet = get_triplet(&hex_str);
        if triplet.is_none() {
            continue;
        }
        let triplet = triplet.unwrap();
        for future_number in added_number + 1..=added_number + 1001 {
            let future_hex_str = stretched_hash(input, future_number, &mut memo);
            if contains_quintuplet(&future_hex_str, triplet) {
                keys_found += 1;
                if keys_found == 64 {
                    return added_number;
                }
                break;
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("abc"), 22728);
    }
    #[test]
    fn example_2() {
        assert_eq!(part2("abc"), 22551);
    }
}
