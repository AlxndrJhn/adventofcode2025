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
fn aba_options(strings: Vec<&str>) -> Vec<(char, char)> {
    let mut options = Vec::new();
    for s in strings {
        let bytes = s.as_bytes();
        for i in 0..bytes.len() - 2 {
            if bytes[i] == bytes[i + 2] && bytes[i] != bytes[i + 1] {
                options.push((bytes[i] as char, bytes[i + 1] as char));
            }
        }
    }
    options
}
fn bab_is_ok(s: &str, options: Vec<(char, char)>) -> bool {
    let bytes = s.as_bytes();
    for i in 0..bytes.len() - 2 {
        if bytes[i] == bytes[i + 2] && bytes[i] != bytes[i + 1] {
            let bab = (bytes[i] as char, bytes[i + 1] as char);
            if options.contains(&(bab.1, bab.0)) {
                return true;
            }
        }
    }
    false
}
fn supports_ssl(ip: &str) -> bool {
    let parts: Vec<&str> = ip.trim().split(&['[', ']'][..]).collect();
    let supernet_sequences = parts
        .iter()
        .enumerate()
        .filter_map(|(i, part)| if i % 2 == 0 { Some(*part) } else { None })
        .collect::<Vec<&str>>();
    let aba_opts = aba_options(supernet_sequences);
    let hypernet_sequences = parts
        .iter()
        .enumerate()
        .filter_map(|(i, part)| if i % 2 == 1 { Some(*part) } else { None })
        .collect::<Vec<&str>>()
        .join(" ");

    bab_is_ok(&hypernet_sequences, aba_opts)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    input
        .trim()
        .lines()
        .filter(|line| supports_ssl(line))
        .count()
}

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
    #[test]
    fn example_5() {
        assert!(supports_ssl("aba[bab]xyz"));
    }
    #[test]
    fn example_6() {
        assert!(!supports_ssl("xyx[xyx]xyx"));
    }
    #[test]
    fn example_7() {
        assert!(supports_ssl("aaa[kek]eke"));
    }
    #[test]
    fn example8() {
        assert!(supports_ssl("zazbz[bzb]cdb"));
    }
}
