pub fn is_valid_password(password: &str) -> bool {
    let bytes = password.as_bytes();

    // Rule 1: Password must not contain 'i', 'o', or 'l'
    if bytes.iter().any(|&b| b == b'i' || b == b'o' || b == b'l') {
        return false;
    }

    // Rule 2: Password must contain at least one increasing straight of three letters
    let has_straight = bytes
        .windows(3)
        .any(|w| w[0] + 1 == w[1] && w[1] + 1 == w[2]);
    if !has_straight {
        return false;
    }

    // Rule 3: Password must contain at least two different, non-overlapping pairs of letters
    let mut pairs = std::collections::HashSet::new();
    let mut i = 0;
    while i < bytes.len() - 1 {
        if bytes[i] == bytes[i + 1] {
            pairs.insert(bytes[i]);
            i += 2; // Skip the next character to avoid overlapping pairs
        } else {
            i += 1;
        }
    }

    pairs.len() >= 2
}

#[aoc(day11, part1)]
pub fn part1(password: &str) -> String {
    let mut bytes = password.as_bytes().to_vec();

    loop {
        // Increment the password
        for i in (0..bytes.len()).rev() {
            if bytes[i] == b'z' {
                bytes[i] = b'a';
            } else {
                bytes[i] += 1;
                break;
            }
        }

        let candidate = String::from_utf8(bytes.clone()).unwrap();
        if is_valid_password(&candidate) {
            return candidate;
        }
    }
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> String {
    let first_password = part1(input);
    part1(&first_password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(!is_valid_password("hijklmmn"));
        assert!(!is_valid_password("abbceffg"));
        assert!(!is_valid_password("abbcegjk"));
        assert_eq!(part1("abcdefgh"), "abcdffaa");
        assert_eq!(part1("ghijklmn"), "ghjaabcc");
    }
}
