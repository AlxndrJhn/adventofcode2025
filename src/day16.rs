#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let reading = "
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"
        .trim();

    let mut target = std::collections::HashMap::new();
    for line in reading.lines() {
        let mut parts = line.split(": ");
        let key = parts.next().unwrap();
        let value: usize = parts.next().unwrap().parse().unwrap();
        target.insert(key, value);
    }

    // Sue 1: goldfish: 6, trees: 9, akitas: 0
    for line in input.lines() {
        let mut parts = line.splitn(2, ": ");
        let _sue = parts.next().unwrap();
        let rest = parts.next().unwrap();
        // goldfish: 6, trees: 9, akitas: 0
        let mut sue_map = std::collections::HashMap::new();
        for item in rest.split(", ") {
            let mut kv = item.splitn(2, ": ");
            let key = kv.next().unwrap();
            let value: usize = kv.next().unwrap().parse().unwrap();
            sue_map.insert(key, value);
        }

        let mut is_match = true;
        for (k, v) in sue_map.iter() {
            if let Some(tv) = target.get(k) {
                if tv != v {
                    is_match = false;
                    break;
                }
            }
        }

        if is_match {
            // Extract Sue number
            let sue_number: usize = _sue.trim_start_matches("Sue ").parse().unwrap();
            return sue_number;
        }
    }
    0
}

// #[aoc(day16, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn example_1() {
    //     assert_eq!(part1("foo"), 0);
    // }
}
