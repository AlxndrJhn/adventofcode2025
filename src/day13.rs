#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    use regex::Regex;
    use std::collections::HashMap;

    let re =
        Regex::new(r"^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).$")
            .unwrap();
    let mut happiness: HashMap<(&str, &str), isize> = HashMap::new();
    let mut people: Vec<&str> = Vec::new();

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let person1 = caps.get(1).unwrap().as_str();
            let gain_lose = caps.get(2).unwrap().as_str();
            let units: isize = caps.get(3).unwrap().as_str().parse().unwrap();
            let person2 = caps.get(4).unwrap().as_str();

            let value = if gain_lose == "gain" { units } else { -units };
            happiness.insert((person1, person2), value);

            if !people.contains(&person1) {
                people.push(person1);
            }
            if !people.contains(&person2) {
                people.push(person2);
            }
        }
    }

    fn calculate_total_happiness(
        arrangement: &[&str],
        happiness: &HashMap<(&str, &str), isize>,
    ) -> isize {
        let mut total = 0;
        for i in 0..arrangement.len() {
            let left = arrangement[i];
            let right = arrangement[(i + 1) % arrangement.len()];
            total += happiness.get(&(left, right)).unwrap_or(&0);
            total += happiness.get(&(right, left)).unwrap_or(&0);
        }
        total
    }

    let mut max_happiness = isize::MIN;
    let permutations = permutohedron::Heap::new(&mut people);
    for arrangement in permutations {
        let total_happiness = calculate_total_happiness(&arrangement, &happiness);
        if total_happiness > max_happiness {
            max_happiness = total_happiness;
        }
    }

    max_happiness as usize
}

// #[aoc(day13, part2)]
// pub fn part2(input: &str) -> String {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "
Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";
        assert_eq!(part1(input), 330);
    }
}
