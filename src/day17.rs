pub fn dfs(
    containers: &[usize],
    target: usize,
    start: usize,
    current_sum: usize,
    count: &mut usize,
) {
    if current_sum == target {
        *count += 1;
        return;
    }
    if current_sum > target {
        return;
    }
    for i in start..containers.len() {
        dfs(
            containers,
            target,
            i + 1,
            current_sum + containers[i],
            count,
        );
    }
}

pub fn combinations(input: &str, target: usize) -> usize {
    let containers: Vec<usize> = input
        .lines()
        .map(|line| line.trim().trim_end_matches(",").parse().unwrap())
        .collect();
    let mut count = 0;
    dfs(&containers, target, 0, 0, &mut count);
    count
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> usize {
    combinations(input, 150)
}

// #[aoc(day17, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            combinations(
                "20,
                15,
                10,
                5,
                5",
                25
            ),
            4
        );
    }
}
