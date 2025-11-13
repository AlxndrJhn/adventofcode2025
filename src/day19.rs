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

// #[aoc(day19, part2)]
// pub fn part2(input: &str) -> usize {
// }

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
}
