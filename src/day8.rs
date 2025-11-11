#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let mut total_code_chars = 0;
    let mut total_memory_chars = 0;
    for line in input.lines() {
        let code_chars = line.len();
        total_code_chars += code_chars;
        let mut memory_chars = 0;
        let chars: Vec<char> = line.chars().collect();
        let mut i = 1;
        while i < chars.len() - 1 {
            if chars[i] == '\\' {
                if i + 1 < chars.len() {
                    match chars[i + 1] {
                        '\\' | '"' => {
                            memory_chars += 1;
                            i += 2;
                        }
                        'x' => {
                            memory_chars += 1;
                            i += 4;
                        }
                        _ => {
                            memory_chars += 1;
                            i += 1;
                        }
                    }
                }
            } else {
                memory_chars += 1;
                i += 1;
            }
        }
        total_memory_chars += memory_chars;
    }
    total_code_chars - total_memory_chars
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    let mut total_code_chars = 0;
    let mut total_encoded_chars = 0;
    for line in input.lines() {
        let code_chars = line.len();
        total_code_chars += code_chars;
        let mut encoded_chars = 2; // for the new surrounding quotes
        for c in line.chars() {
            match c {
                '"' | '\\' => {
                    encoded_chars += 2;
                }
                _ => {
                    encoded_chars += 1;
                }
            }
        }
        total_encoded_chars += encoded_chars;
    }
    total_encoded_chars - total_code_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "
\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"
"
        .trim();
        assert_eq!(part1(input), 12);
    }

    #[test]
    fn example_2() {
        let input = "
\"\"
\"abc\"
\"aaa\\\"aaa\"
\"\\x27\"
"
        .trim();
        assert_eq!(part2(input), 19);
    }
}
