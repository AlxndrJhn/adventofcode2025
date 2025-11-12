use regex::Regex;

#[aoc(day12, part1)]
pub fn part1(input: &str) -> isize {
    let regex_pattern = Regex::new(r"-?\d+").unwrap();
    regex_pattern
        .find_iter(input)
        .map(|mat| mat.as_str().parse::<isize>().unwrap())
        .sum()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> i128 {
    let json_value: serde_json::Value = serde_json::from_str(input).unwrap();
    fn sum_json(value: &serde_json::Value) -> i128 {
        match value {
            serde_json::Value::Number(n) => n.as_i128().unwrap_or(0),
            serde_json::Value::Array(arr) => arr.iter().map(sum_json).sum(),
            serde_json::Value::Object(map) => {
                if map.values().any(|v| v == "red") {
                    0
                } else {
                    map.values().map(sum_json).sum()
                }
            }
            _ => 0,
        }
    }
    sum_json(&json_value)
}

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
    #[test]
    fn example_9() {
        assert_eq!(part2("[1,2,3]"), 6);
    }
    #[test]
    fn example_10() {
        assert_eq!(part2(r#"[1,{"c":"red","b":2},3]"#), 4);
    }
    #[test]
    fn example_11() {
        assert_eq!(part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
    }
    #[test]
    fn example_12() {
        assert_eq!(part2(r#"[1,"red",5]"#), 6);
    }
}
