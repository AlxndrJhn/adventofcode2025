pub fn one_step(n: &str) -> &str {
    let chars: Vec<char> = n.chars().collect();
    let mut result = String::new();

    let mut count = 1;
    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(chars[i - 1]);
            count = 1;
        }
    }
    result.push_str(&count.to_string());
    result.push(*chars.last().unwrap());

    Box::leak(result.into_boxed_str())
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let mut current = input.trim();
    for _ in 0..40 {
        current = one_step(current);
    }
    current.to_string().len()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let mut current = input.trim();
    for _ in 0..50 {
        current = one_step(current);
    }
    current.to_string().len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(one_step("1"), "11");
        assert_eq!(one_step("11"), "21");
        assert_eq!(one_step("21"), "1211");
        assert_eq!(one_step("1211"), "111221");
        assert_eq!(one_step("111221"), "312211");
    }
}
