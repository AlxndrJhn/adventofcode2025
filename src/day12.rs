use regex::Regex;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> isize {
    let regex_pattern = Regex::new(r"-?\d+").unwrap();
    regex_pattern
        .find_iter(input)
        .map(|mat| mat.as_str().parse::<isize>().unwrap())
        .sum()
}

// #[aoc(day12, part2)]
// pub fn part2(input: &str) -> String {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("[1,2,3]"), 6);
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("{\"a\":2,\"b\":4}"), 6);
    }
    #[test]
    fn example_3() {
        assert_eq!(part1("[[[3]]]"), 3);
    }
    #[test]
    fn example_4() {
        assert_eq!(part1("{\"a\":{\"b\":4},\"c\":-1}"), 3);
    }
    #[test]
    fn example_5() {
        assert_eq!(part1("{\"a\":[-1,1]}"), 0);
    }
    #[test]
    fn example_6() {
        assert_eq!(part1("[-1,{\"a\":1}]"), 0);
    }
    #[test]
    fn example_7() {
        assert_eq!(part1("[]"), 0);
    }
    #[test]
    fn example_8() {
        assert_eq!(part1("{}"), 0);
    }
}
