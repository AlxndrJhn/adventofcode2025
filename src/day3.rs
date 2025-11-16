#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let possible_triangles = input.lines().filter(|line| {
        let mut sides: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        sides.sort_unstable();
        sides[0] + sides[1] > sides[2]
    });
    possible_triangles.count()
}

// #[aoc(day3, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("5 10 25"), 0);
    }
}
