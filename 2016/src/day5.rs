use std::io::{stdout, Write};

#[aoc(day5, part1)]
pub fn part1(input: &str) -> String {
    let input = input.trim();
    let mut password = String::new();
    for i in 0.. {
        let mut hash = md5::Context::new();
        hash.consume(input);
        hash.consume(&i.to_string());
        let digest = format!("{:x}", hash.finalize());
        if digest.starts_with("00000") {
            password.push(digest.chars().nth(5).unwrap());
            if password.len() == 8 {
                return password.to_string();
            }
        }
    }
    unreachable!()
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let mut password = vec!['_'; 8];
    let mut stdout = stdout();
    for i in 0.. {
        let mut hash = md5::Context::new();
        hash.consume(input);
        hash.consume(&i.to_string());
        let digest = format!("{:x}", hash.finalize());
        if digest.starts_with("00000") {
            let pos_char = digest.chars().nth(5).unwrap();
            let char = digest.chars().nth(6).unwrap();
            if pos_char.is_digit(10) {
                let pos = pos_char.to_digit(10).unwrap() as usize;
                if pos < 8 && password[pos] == '_' {
                    password[pos] = char;
                    print!("\r{:?}", password);
                    stdout.flush().unwrap();
                    if !password.contains(&'_') {
                        println!();
                        return password.iter().collect();
                    }
                }
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
        assert_eq!(part1("abc"), "18f47a30");
    }
    #[test]
    fn example_2() {
        assert_eq!(part2("abc"), "05ace8e3");
    }
}
