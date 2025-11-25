fn swapping(input: &str, word: &str) -> String {
    let mut current_string = word.to_string();
    for line in input.trim().lines() {
        let line = line.trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "swap" => {
                if parts[1] == "position" {
                    let x: usize = parts[2].parse().unwrap();
                    let y: usize = parts[5].parse().unwrap();
                    let mut chars: Vec<char> = current_string.chars().collect();
                    chars.swap(x, y);
                    current_string = chars.into_iter().collect();
                } else if parts[1] == "letter" {
                    let x = parts[2].chars().next().unwrap();
                    let y = parts[5].chars().next().unwrap();
                    current_string = current_string
                        .chars()
                        .map(|c| {
                            if c == x {
                                y
                            } else if c == y {
                                x
                            } else {
                                c
                            }
                        })
                        .collect();
                } else {
                    panic!("Unknown swap operation: {}", line);
                }
            }
            "rotate" => {
                if parts[1] == "left" {
                    let steps: usize = parts[2].parse().unwrap();
                    let len = current_string.len();
                    let steps = steps % len;
                    current_string =
                        format!("{}{}", &current_string[steps..], &current_string[..steps]);
                } else if parts[1] == "right" {
                    let steps: usize = parts[2].parse().unwrap();
                    let len = current_string.len();
                    let steps = steps % len;
                    current_string = format!(
                        "{}{}",
                        &current_string[len - steps..],
                        &current_string[..len - steps]
                    );
                } else if parts[1] == "based" {
                    let x = parts[6].chars().next().unwrap();
                    let index = current_string.find(x).unwrap();
                    let mut steps = 1 + index;
                    if index >= 4 {
                        steps += 1;
                    }
                    let len = current_string.len();
                    let steps = steps % len;
                    current_string = format!(
                        "{}{}",
                        &current_string[len - steps..],
                        &current_string[..len - steps]
                    );
                } else {
                    panic!("Unknown rotate operation: {}", line);
                }
            }
            "reverse" => {
                let x: usize = parts[2].parse().unwrap();
                let y: usize = parts[4].parse().unwrap();
                let mut chars: Vec<char> = current_string.chars().collect();
                chars[x..=y].reverse();
                current_string = chars.into_iter().collect();
            }
            "move" => {
                let x: usize = parts[2].parse().unwrap();
                let y: usize = parts[5].parse().unwrap();
                let mut chars: Vec<char> = current_string.chars().collect();
                let c = chars.remove(x);
                chars.insert(y, c);
                current_string = chars.into_iter().collect();
            }
            _ => {
                panic!("Unknown operation: {}", line);
            }
        }
    }
    current_string
}

fn unscramble(input: &str, word: &str) -> String {
    let mut current_string = word.to_string();
    let lines: Vec<&str> = input.trim().lines().map(|l| l.trim()).collect();
    for line in lines.iter().rev() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "swap" => {
                if parts[1] == "position" {
                    let x: usize = parts[2].parse().unwrap();
                    let y: usize = parts[5].parse().unwrap();
                    let mut chars: Vec<char> = current_string.chars().collect();
                    chars.swap(x, y);
                    current_string = chars.into_iter().collect();
                } else if parts[1] == "letter" {
                    let x = parts[2].chars().next().unwrap();
                    let y = parts[5].chars().next().unwrap();
                    current_string = current_string
                        .chars()
                        .map(|c| {
                            if c == x {
                                y
                            } else if c == y {
                                x
                            } else {
                                c
                            }
                        })
                        .collect();
                } else {
                    panic!("Unknown swap operation: {}", line);
                }
            }
            "rotate" => {
                if parts[1] == "left" {
                    let steps: usize = parts[2].parse().unwrap();
                    let len = current_string.len();
                    let steps = steps % len;
                    current_string = format!(
                        "{}{}",
                        &current_string[len - steps..],
                        &current_string[..len - steps]
                    );
                } else if parts[1] == "right" {
                    let steps: usize = parts[2].parse().unwrap();
                    let len = current_string.len();
                    let steps = steps % len;
                    current_string =
                        format!("{}{}", &current_string[steps..], &current_string[..steps]);
                } else if parts[1] == "based" {
                    let x = parts[6].chars().next().unwrap();
                    let len = current_string.len();
                    let mut index = 0;
                    for i in 0..len {
                        let mut test_string = current_string.clone();
                        // Rotate left i steps
                        test_string = format!("{}{}", &test_string[i..], &test_string[..i]);
                        let idx = test_string.find(x).unwrap();
                        let mut steps = 1 + idx;
                        if idx >= 4 {
                            steps += 1;
                        }
                        steps = steps % len;
                        if steps == i {
                            index = i;
                            break;
                        }
                    }
                    current_string =
                        format!("{}{}", &current_string[index..], &current_string[..index]);
                } else {
                    panic!("Unknown rotate operation: {}", line);
                }
            }
            "reverse" => {
                let x: usize = parts[2].parse().unwrap();
                let y: usize = parts[4].parse().unwrap();
                let mut chars: Vec<char> = current_string.chars().collect();
                chars[x..=y].reverse();
                current_string = chars.into_iter().collect();
            }
            "move" => {
                let y: usize = parts[2].parse().unwrap();
                let x: usize = parts[5].parse().unwrap();
                let mut chars: Vec<char> = current_string.chars().collect();
                let c = chars.remove(x);
                chars.insert(y, c);
                current_string = chars.into_iter().collect();
            }
            _ => {
                panic!("Unknown operation: {}", line);
            }
        }
    }
    current_string
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> String {
    swapping(input, "abcdefgh")
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> String {
    unscramble(input, "fbgdceah")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            swapping(
                "   swap position 4 with position 0
                    swap letter d with letter b
                    reverse positions 0 through 4
                    rotate left 1 step
                    move position 1 to position 4
                    move position 3 to position 0
                    rotate based on position of letter b
                    rotate based on position of letter d",
                "abcde"
            ),
            "decab"
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            swapping("swap position 4 with position 0", "abcde"),
            "ebcda"
        );
    }
    #[test]
    fn example_3() {
        assert_eq!(swapping("swap letter d with letter b", "ebcda"), "edcba");
    }
    #[test]
    fn example_4() {
        assert_eq!(swapping("reverse positions 0 through 4", "edcba"), "abcde");
    }
    #[test]
    fn example_5() {
        assert_eq!(swapping("rotate left 1 step", "abcde"), "bcdea");
    }
    #[test]
    fn example_6() {
        assert_eq!(swapping("move position 1 to position 4", "bcdea"), "bdeac");
    }
    #[test]
    fn example_7() {
        assert_eq!(swapping("move position 3 to position 0", "bdeac"), "abdec");
    }
    #[test]
    fn example_8() {
        assert_eq!(
            swapping("rotate based on position of letter b", "abdec"),
            "ecabd"
        );
    }
    #[test]
    fn example_9() {
        assert_eq!(
            swapping("rotate based on position of letter d", "ecabd"),
            "decab"
        );
    }
    #[test]
    fn example_10() {
        assert_eq!(
            unscramble(
                "   swap position 4 with position 0
                    swap letter d with letter b
                    reverse positions 0 through 4
                    rotate left 1 step
                    move position 1 to position 4
                    move position 3 to position 0
                    rotate based on position of letter b
                    rotate based on position of letter d",
                "decab"
            ),
            "abcde"
        );
    }

    #[test]
    fn example_12() {
        assert_eq!(
            unscramble("swap position 4 with position 0", "ebcda"),
            "abcde"
        );
    }
    #[test]
    fn example_13() {
        assert_eq!(unscramble("swap letter d with letter b", "edcba"), "ebcda");
    }
    #[test]
    fn example_14() {
        assert_eq!(
            unscramble("reverse positions 0 through 4", "abcde"),
            "edcba"
        );
    }
    #[test]
    fn example_15() {
        assert_eq!(unscramble("rotate left 1 step", "bcdea"), "abcde");
    }
    #[test]
    fn example_16() {
        assert_eq!(
            unscramble("move position 1 to position 4", "bdeac"),
            "bcdea"
        );
    }
    #[test]
    fn example_17() {
        assert_eq!(
            unscramble("move position 3 to position 0", "abdec"),
            "bdeac"
        );
    }
    #[test]
    fn example_18() {
        assert_eq!(
            unscramble("rotate based on position of letter b", "ecabd"),
            "abdec"
        );
    }
    #[test]
    fn example_19() {
        assert_eq!(
            unscramble("rotate based on position of letter d", "decab"),
            "ecabd"
        );
    }
}
