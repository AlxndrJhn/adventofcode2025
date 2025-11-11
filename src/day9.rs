use permute::permute;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let mut locations = vec![];
    let mut distances = std::collections::HashMap::new();

    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let loc1 = parts[0];
        let loc2 = parts[2];
        let dist: usize = parts[4].parse().unwrap();

        if !locations.contains(&loc1) {
            locations.push(loc1);
        }
        if !locations.contains(&loc2) {
            locations.push(loc2);
        }

        distances.insert((loc1, loc2), dist);
        distances.insert((loc2, loc1), dist);
    }

    let mut min_distance = usize::MAX;

    for perm in permute(locations) {
        let mut total_dist = 0;
        for i in 0..perm.len() - 1 {
            total_dist += distances.get(&(perm[i], perm[i + 1])).unwrap();
        }
        if total_dist < min_distance {
            min_distance = total_dist;
        }
    }

    min_distance
}

// #[aoc(day9, part2)]
// pub fn part2(input: &str) -> usize {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "
London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        assert_eq!(part1(input), 605);
    }
}
