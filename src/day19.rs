#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let n: usize = input.trim().parse().unwrap();
    let mut elves: Vec<usize> = vec![1; n];
    let mut index = 0;
    let mut remaining = n;
    loop {
        if elves[index] > 0 {
            let mut target_index = (index + 1) % n;
            while elves[target_index] == 0 {
                target_index = (target_index + 1) % n;
            }
            elves[index] += elves[target_index];
            elves[target_index] = 0;
            remaining -= 1;
            if remaining == 1 {
                return index + 1;
            }
        }
        index = (index + 1) % n;
    }
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    //  source: https://www.reddit.com/r/adventofcode/comments/5j4lp1/comment/dbdgnwd/
    let n: usize = input.trim().parse().unwrap();
    let mut w = 1;
    for i in 1..n {
        w = w % i + 1;
        if w > (i + 1) / 2 {
            w += 1;
        }
    }
    w
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("5"), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(part2("5"), 2);
    }
}
