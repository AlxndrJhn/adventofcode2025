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

pub fn dfs_counter(
    containers: &[usize],
    target: usize,
    start: usize,
    current_sum: usize,
    count_hashmap: &mut std::collections::HashMap<usize, usize>,
    current_count: usize,
) {
    if current_sum == target {
        *count_hashmap.entry(current_count).or_insert(0) += 1;
        return;
    }
    if current_sum > target {
        return;
    }
    for i in start..containers.len() {
        dfs_counter(
            containers,
            target,
            i + 1,
            current_sum + containers[i],
            count_hashmap,
            current_count + 1,
        );
    }
}

pub fn combinations_of_minimum(input: &str, target: usize) -> usize {
    let containers: Vec<usize> = input
        .lines()
        .map(|line| line.trim().trim_end_matches(",").parse().unwrap())
        .collect();
    let mut count_hashmap = std::collections::HashMap::new();
    dfs_counter(&containers, target, 0, 0, &mut count_hashmap, 0);
    let min_count = *count_hashmap.keys().min().unwrap();
    *count_hashmap.get(&min_count).unwrap()
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> usize {
    combinations_of_minimum(input, 150)
}

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
    #[test]
    fn example_2() {
        assert_eq!(
            combinations_of_minimum(
                "20,
                15,
                10,
                5,
                5",
                25
            ),
            3
        );
    }
}
