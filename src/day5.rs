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

// #[aoc(day5, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("abc"), "18f47a30");
    }
}
