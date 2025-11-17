use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
fn get_element_to_floor(input: &str) -> HashMap<String, String> {
    let mut element_to_floor = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let floor = "F".to_string() + &(i + 1).to_string();
        let line_parts = line.split_whitespace();
        for (j, word) in line_parts.clone().enumerate() {
            if word.ends_with("-compatible") {
                let element = word.trim_end_matches("-compatible").to_string();
                let chip = element + "M";
                element_to_floor.insert(chip, floor.clone());
            } else if word.starts_with("generator") {
                let element = line_parts.clone().nth(j - 1).unwrap();
                let gen = element.to_string() + "G";
                element_to_floor.insert(gen, floor.clone());
            }
        }
    }
    element_to_floor.insert("E".to_string(), "F1".to_string());
    element_to_floor
}
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
fn dfs(
    element_to_floor: HashMap<String, String>,
    step_count: usize,
    min_step_count: &mut usize,
    state_to_min_step_count: &mut HashMap<u64, usize>,
    current_sequence: String,
    best_sequence: &mut HashMap<u64, String>,
    valid_states: &mut HashMap<u64, bool>,
    do_store_path: bool,
) {
    let mut items: Vec<String> = element_to_floor
        .iter()
        .map(|(k, v)| format!("{}:{}", k, v))
        .collect();
    items.sort();
    let items_string = items.join("|");
    let items_string_hash = calculate_hash(&items_string);
    if valid_states.contains_key(&items_string_hash) {
        if !valid_states[&items_string_hash] {
            return;
        }
    } else {
        let is_valid = is_valid_state(&element_to_floor);
        valid_states.insert(items_string_hash, is_valid);
        if !is_valid {
            return;
        }
    }
    if state_to_min_step_count.contains_key(&items_string_hash) {
        if step_count >= state_to_min_step_count[&items_string_hash] {
            return;
        }
    }
    state_to_min_step_count.insert(items_string_hash, step_count);
    if do_store_path {
        best_sequence.insert(items_string_hash, current_sequence.clone());
    }
    if step_count >= *min_step_count {
        return;
    }
    let all_on_fourth = element_to_floor.values().all(|floor| floor == "F4");
    if all_on_fourth {
        println!("Best sequence found: {}", step_count);
        *min_step_count = step_count;
        return;
    }

    let mut potential_next_states: Vec<HashMap<String, String>> = Vec::new();
    let mut potential_next_sequences: Vec<String> = Vec::new();
    let elevator_floor = element_to_floor.get("E").unwrap().clone();
    let directions = {
        let mut dirs = Vec::new();
        if elevator_floor != "F4" {
            dirs.push(1); // up
        }
        if elevator_floor != "F1" {
            dirs.push(-1); // down
        }
        dirs
    };
    let items_on_floor: Vec<String> = element_to_floor
        .iter()
        .filter(|(_, floor)| *floor == &elevator_floor)
        .map(|(item, _)| item.clone())
        .collect();
    for &dir in &directions {
        let new_floor_number =
            elevator_floor.chars().nth(1).unwrap().to_digit(10).unwrap() as i32 + dir;
        let new_floor = "F".to_string() + &new_floor_number.to_string();
        for i in 0..items_on_floor.len() {
            // move one item
            if items_on_floor[i] == "E" {
                continue;
            }
            let mut new_state = element_to_floor.clone();
            new_state.insert("E".to_string(), new_floor.clone());
            new_state.insert(items_on_floor[i].clone(), new_floor.clone());
            potential_next_states.push(new_state);
            if do_store_path {
                let new_best_sequence_for_this = format!(
                    "\n{}->{}(bring {}) ",
                    elevator_floor, new_floor, items_on_floor[i]
                );
                potential_next_sequences
                    .push(current_sequence.clone() + &new_best_sequence_for_this);
            }

            for j in i + 1..items_on_floor.len() {
                if items_on_floor[j] == "E" {
                    continue;
                }
                // move two items
                let mut new_state = element_to_floor.clone();
                new_state.insert("E".to_string(), new_floor.clone());
                new_state.insert(items_on_floor[i].clone(), new_floor.clone());
                new_state.insert(items_on_floor[j].clone(), new_floor.clone());
                potential_next_states.push(new_state);
                if do_store_path {
                    let new_best_sequence_for_this = format!(
                        "\n{}->{}(bring {}, {}) ",
                        elevator_floor, new_floor, items_on_floor[i], items_on_floor[j]
                    );
                    potential_next_sequences
                        .push(current_sequence.clone() + &new_best_sequence_for_this);
                }
            }
        }
    }
    // sort potential_next_states by number of items on higher floors
    let sorted_potential_next_states: Vec<HashMap<String, String>> = {
        let mut states_with_scores: Vec<(HashMap<String, String>, usize)> = potential_next_states
            .into_iter()
            .map(|state| {
                let score: usize = state
                    .values()
                    .map(|floor| match floor.as_str() {
                        "F1" => 0,
                        "F2" => 1,
                        "F3" => 2,
                        "F4" => 3,
                        _ => 0,
                    })
                    .sum();
                (state, score)
            })
            .collect();
        states_with_scores.sort_by(|a, b| b.1.cmp(&a.1));
        states_with_scores
            .into_iter()
            .map(|(state, _)| state)
            .collect()
    };
    if do_store_path {
        for (next_state, next_sequence) in sorted_potential_next_states
            .iter()
            .zip(potential_next_sequences.iter())
        {
            dfs(
                next_state.clone(),
                step_count + 1,
                min_step_count,
                state_to_min_step_count,
                next_sequence.clone(),
                best_sequence,
                valid_states,
                do_store_path,
            );
        }
    } else {
        for next_state in sorted_potential_next_states.iter() {
            dfs(
                next_state.clone(),
                step_count + 1,
                min_step_count,
                state_to_min_step_count,
                current_sequence.clone(),
                best_sequence,
                valid_states,
                do_store_path,
            );
        }
    }
}

fn is_valid_state(state: &HashMap<String, String>) -> bool {
    let mut floor_to_items: HashMap<String, Vec<String>> = HashMap::new();
    for (item, floor) in state.iter() {
        if item == "E" {
            continue;
        }
        floor_to_items
            .entry(floor.clone())
            .or_insert_with(Vec::new)
            .push(item.clone());
    }
    for items in floor_to_items.values() {
        let generators: Vec<&String> = items.iter().filter(|item| item.ends_with("G")).collect();
        if !generators.is_empty() {
            for item in items {
                if item.ends_with("M") {
                    let element = item.trim_end_matches("M");
                    let corresponding_gen = element.to_string() + "G";
                    if !items.contains(&corresponding_gen) {
                        return false;
                    }
                }
            }
        }
    }
    true
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let element_to_floor = get_element_to_floor(input);
    let mut min_step_count = usize::MAX;
    let mut state_to_min_step_count = HashMap::new();
    let mut best_sequence = HashMap::new();
    let current_sequence = String::new();
    let mut valid_states = HashMap::new();
    dfs(
        element_to_floor,
        0,
        &mut min_step_count,
        &mut state_to_min_step_count,
        current_sequence,
        &mut best_sequence,
        &mut valid_states,
        false,
    );
    min_step_count
}

// #[aoc(day11, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.
"), 11);
    }
}
