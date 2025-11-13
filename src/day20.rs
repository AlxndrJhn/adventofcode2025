#[aoc(day20, part1)]
pub fn part1(input: &str) -> usize {
    let target: usize = input.trim().parse().unwrap();
    let mut house = 1;
    loop {
        let presents = (1..=house)
            .filter(|elf| house % elf == 0)
            .map(|elf| elf * 10)
            .sum::<usize>();
        if presents >= target {
            return house;
        }
        house += 1;
    }
}

// #[aoc(day20, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("10"), 1);
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("30"), 2);
    }
    #[test]
    fn example_3() {
        assert_eq!(part1("120"), 6);
    }
    #[test]
    fn example_4() {
        assert_eq!(part1("150"), 8);
    }
}
