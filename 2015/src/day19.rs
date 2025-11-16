#[aoc(day19, part1)]
pub fn part1(input: &str) -> usize {
    let starting_molecule = input.lines().last().unwrap().trim();
    let mut replacements = vec![];
    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() || line == starting_molecule {
            continue;
        }
        let parts: Vec<&str> = line.split(" => ").collect();
        replacements.push((parts[0], parts[1]));
    }
    let mut distinct_molecules = std::collections::HashSet::new();
    for (from, to) in replacements {
        let mut start_index = 0;
        while let Some(index) = starting_molecule[start_index..].find(from) {
            let index = start_index + index;
            let new_molecule = format!(
                "{}{}{}",
                &starting_molecule[..index],
                to,
                &starting_molecule[index + from.len()..]
            );
            distinct_molecules.insert(new_molecule);
            start_index = index + 1;
        }
    }
    distinct_molecules.len()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let goal_molecule = input.lines().last().unwrap().trim();
    let mut replacements = vec![];
    for line in input.trim().lines() {
        let line = line.trim();
        if line.is_empty() || line == goal_molecule {
            continue;
        }
        let parts: Vec<&str> = line.split(" => ").collect();
        replacements.push((parts[0], parts[1]));
    }
    let mut heap = std::collections::BinaryHeap::new();
    heap.push((0usize, goal_molecule.to_string()));
    let mut visited = std::collections::HashSet::new();
    while let Some((steps, molecule)) = heap.pop() {
        if molecule == "e" {
            return steps;
        }
        if visited.contains(&molecule) {
            continue;
        }
        visited.insert(molecule.clone());
        for (from, to) in &replacements {
            let mut start_index = 0;
            while let Some(index) = molecule[start_index..].find(to) {
                let index = start_index + index;
                let new_molecule = format!(
                    "{}{}{}",
                    &molecule[..index],
                    from,
                    &molecule[index + to.len()..]
                );
                heap.push((steps + 1, new_molecule));
                start_index = index + 1;
            }
        }
    }
    panic!("No solution found");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "   H => HO
                    H => OH
                    O => HH
                    
                    HOH"
            ),
            4
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            part1(
                "   H => HO
                    H => OH
                    O => HH
                    
                    HOHOHO"
            ),
            7
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            part2(
                "   e => H
                    e => O
                    H => HO
                    H => OH
                    O => HH
                    
                    HOH"
            ),
            3
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(
            part2(
                "   e => H
                    e => O
                    H => HO
                    H => OH
                    O => HH
                    
                    HOHOHO"
            ),
            6
        );
    }
}
