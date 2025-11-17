use std::collections::HashMap;

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let responsibilities = get_responsibilities(input);
    *responsibilities.0.get(&(17, 61)).unwrap()
}

fn get_responsibilities(input: &str) -> (HashMap<(usize, usize), usize>, usize) {
    let mut responsibilities = HashMap::new();
    let mut robot_instructions = HashMap::new();
    let mut robot_states: HashMap<(&str, usize), Vec<usize>> = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts[0] == "value" {
            let value: usize = parts[1].parse().unwrap();
            let bot_id: usize = parts[5].parse().unwrap();
            robot_states.entry(("bot", bot_id)).or_default().push(value);
        } else if parts[0] == "bot" {
            let bot_id: usize = parts[1].parse().unwrap();
            let target_type_low = parts[5];
            let low_id: usize = parts[6].parse().unwrap();
            let high_id: usize = parts[11].parse().unwrap();
            let target_type_high = parts[10];
            robot_instructions.insert(bot_id, (target_type_low, low_id, target_type_high, high_id));
        }
    }
    loop {
        let robots_with_two_values: Vec<usize> = robot_states
            .iter()
            .filter_map(|(&(src_type, bot_id), values)| {
                if src_type == "bot" && values.len() >= 2 {
                    Some(bot_id)
                } else {
                    None
                }
            })
            .collect();
        if robots_with_two_values.is_empty() {
            break;
        }
        for bot_id in robots_with_two_values {
            let values = robot_states.get_mut(&("bot", bot_id)).unwrap();
            values.sort();
            let low_value = values.remove(0);
            let high_value = values.remove(0);
            responsibilities.insert((low_value, high_value), bot_id);
            if let Some(&(target_type_low, low_id, target_type_high, high_id)) =
                robot_instructions.get(&bot_id)
            {
                robot_states
                    .entry((target_type_low, low_id))
                    .or_default()
                    .push(low_value);
                robot_states
                    .entry((target_type_high, high_id))
                    .or_default()
                    .push(high_value);
            } else {
                unreachable!()
            }
        }
    }
    let output_product = robot_states
        .get(&("output", 0))
        .and_then(|vals| vals.get(0))
        .unwrap_or(&0)
        * robot_states
            .get(&("output", 1))
            .and_then(|vals| vals.get(0))
            .unwrap_or(&0)
        * robot_states
            .get(&("output", 2))
            .and_then(|vals| vals.get(0))
            .unwrap_or(&0);
    (responsibilities, output_product)
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let (_, result) = get_responsibilities(input);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            get_responsibilities(
                "   value 5 goes to bot 2
                    bot 2 gives low to bot 1 and high to bot 0
                    value 3 goes to bot 1
                    bot 1 gives low to output 1 and high to bot 0
                    bot 0 gives low to output 2 and high to output 0
                    value 2 goes to bot 2",
            )
            .0
            .get(&(2, 5)),
            Some(&2)
        );
    }
}
