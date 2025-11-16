#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let mut location = (1, 1); // starting at '5' on the keypad
    let keypad = [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']];
    let mut code = String::new();
    for line in input.trim().lines() {
        for command in line.chars() {
            match command {
                'U' if location.0 > 0 => location.0 -= 1,
                'D' if location.0 < 2 => location.0 += 1,
                'L' if location.1 > 0 => location.1 -= 1,
                'R' if location.1 < 2 => location.1 += 1,
                _ => {}
            }
        }
        code.push(keypad[location.0][location.1]);
    }
    code.parse().unwrap()
}

// #[aoc(day2, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "   ULL
                    RRDDD
                    LURDL
                    UUUUD
"
            ),
            1985
        );
    }
}
