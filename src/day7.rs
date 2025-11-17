#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| supports_tls(line))
        .count()
}
fn contains_abba(s: &str) -> bool {
    let bytes = s.as_bytes();
    for i in 0..bytes.len() - 3 {
        if bytes[i] != bytes[i + 1] && bytes[i] == bytes[i + 3] && bytes[i + 1] == bytes[i + 2] {
            return true;
        }
    }
    false
}

fn supports_tls(ip: &str) -> bool {
    let parts: Vec<&str> = ip.trim().split(&['[', ']'][..]).collect();
    let mut has_abba_outside = false;
    for (i, part) in parts.iter().enumerate() {
        if i % 2 == 0 {
            // Outside brackets
            if contains_abba(part) {
                has_abba_outside = true;
            }
        } else {
            // Inside brackets
            if contains_abba(part) {
                return false;
            }
        }
    }
    has_abba_outside
}
// #[aoc(day7, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(supports_tls("abba[mnop]qrst"));
    }
    #[test]
    fn example_2() {
        assert!(!supports_tls("abcd[bddb]xyyx"));
    }
    #[test]
    fn example_3() {
        assert!(!supports_tls("aaaa[qwer]tyui"));
    }
    #[test]
    fn example_4() {
        assert!(supports_tls("ioxxoj[asdfgh]zxcvbn"));
    }
}
