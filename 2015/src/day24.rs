pub fn get_lowest_qe(input: &str, groups: usize) -> usize {
    let packages: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    let total_weight: usize = packages.iter().sum();
    let group_weight = total_weight / groups;

    let mut lowest_qe = usize::MAX;
    let n = packages.len();
    for size in 1..=n {
        let mut found = false;
        let mut indices = (0..size).collect::<Vec<usize>>();
        loop {
            let group: Vec<usize> = indices.iter().map(|&i| packages[i]).collect();
            let group_sum: usize = group.iter().sum();
            if group_sum == group_weight {
                let qe: usize = group.iter().product();
                if qe < lowest_qe {
                    lowest_qe = qe;
                }
                found = true;
            }
            let mut i = size - 1;
            while i > 0 && indices[i] == n - size + i {
                i -= 1;
            }
            if indices[i] == n - size + i {
                break;
            }
            indices[i] += 1;
            for j in i + 1..size {
                indices[j] = indices[j - 1] + 1;
            }
        }
        if found {
            break;
        }
    }
    lowest_qe
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    get_lowest_qe(input, 3)
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> usize {
    get_lowest_qe(input, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("1\n2\n3\n4\n5\n7\n8\n9\n10\n11"), 99);
    }

    #[test]
    fn example_2() {
        assert_eq!(part2("1\n2\n3\n4\n5\n7\n8\n9\n10\n11"), 44);
    }
}
