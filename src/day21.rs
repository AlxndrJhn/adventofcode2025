use core::panic;

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

#[aoc(day21, part1)]
pub fn part1(input: &str) -> String {
    swapping(input, "abcdefgh")
}

// #[aoc(day21, part2)]
// pub fn part2(input: &str) -> usize {
// }

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
}
