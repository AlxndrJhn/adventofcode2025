use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    elevator: usize,
    floors: Vec<Vec<Item>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Item {
    Generator(String),
    Microchip(String),
}

impl State {
    fn is_valid(&self) -> bool {
        for floor in &self.floors {
            let generators: Vec<_> = floor
                .iter()
                .filter_map(|item| match item {
                    Item::Generator(element) => Some(element),
                    _ => None,
                })
                .collect();
            if generators.is_empty() {
                continue;
            }
            for item in floor {
                if let Item::Microchip(element) = item {
                    if !generators.contains(&element) {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn is_goal(&self) -> bool {
        self.floors[0].is_empty() && self.floors[1].is_empty() && self.floors[2].is_empty()
    }
    fn normalize(&self) -> State {
        let mut element_map = HashMap::new();
        let mut next_id = 0;
        let mut normalized_floors = vec![Vec::new(); 4];
        for (floor_idx, floor) in self.floors.iter().enumerate() {
            for item in floor {
                let element = match item {
                    Item::Generator(e) | Item::Microchip(e) => e,
                };
                let id = element_map.entry(element.clone()).or_insert_with(|| {
                    let id = next_id;
                    next_id += 1;
                    id
                });
                let normalized_item = match item {
                    Item::Generator(_) => Item::Generator(id.to_string()),
                    Item::Microchip(_) => Item::Microchip(id.to_string()),
                };
                normalized_floors[floor_idx].push(normalized_item);
            }
        }
        for floor in &mut normalized_floors {
            floor.sort();
        }
        State {
            elevator: self.elevator,
            floors: normalized_floors,
        }
    }
}

fn parse_input(input: &str) -> State {
    let mut floors = vec![Vec::new(); 4];
    for (idx, line) in input.lines().enumerate() {
        if line.contains("nothing relevant") {
            continue;
        }
        let parts: Vec<&str> = line.split(" contains ").collect();
        if parts.len() < 2 {
            continue;
        }
        let items_str = parts[1].trim_end_matches('.');
        let words: Vec<&str> = items_str.split_whitespace().collect();
        for i in 0..words.len() {
            if words[i].ends_with("-compatible") {
                let material = words[i].trim_end_matches("-compatible").to_string();
                floors[idx].push(Item::Microchip(material));
            } else if words[i].starts_with("generator") {
                floors[idx].push(Item::Generator(words[i - 1].to_string()));
            }
        }
    }
    State {
        elevator: 0,
        floors,
    }
}

fn solve(initial_state: State) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((initial_state.clone(), 0));
    visited.insert(initial_state.normalize());
    while let Some((state, steps)) = queue.pop_front() {
        if state.is_goal() {
            return steps;
        }
        let current_floor = state.elevator;
        let items = &state.floors[current_floor];
        let mut moves = Vec::new();
        for i in 0..items.len() {
            moves.push(vec![i]);
        }
        for i in 0..items.len() {
            for j in i + 1..items.len() {
                moves.push(vec![i, j]);
            }
        }
        let mut directions = Vec::new();
        if current_floor < 3 {
            directions.push(1);
        }
        if current_floor > 0 {
            let has_items_below = (0..current_floor).any(|f| !state.floors[f].is_empty());
            if has_items_below {
                directions.push(-1);
            }
        }
        for direction in directions {
            let new_floor = (current_floor as i32 + direction) as usize;
            for move_indices in &moves {
                let mut new_state = state.clone();
                new_state.elevator = new_floor;
                let mut moved_items = Vec::new();
                for &idx in move_indices.iter().rev() {
                    moved_items.push(new_state.floors[current_floor].remove(idx));
                }
                for item in moved_items {
                    new_state.floors[new_floor].push(item);
                }
                if new_state.is_valid() {
                    let normalized = new_state.normalize();
                    if !visited.contains(&normalized) {
                        visited.insert(normalized);
                        queue.push_back((new_state, steps + 1));
                    }
                }
            }
        }
    }
    0
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let state = parse_input(input);
    solve(state)
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let mut state = parse_input(input);
    state.floors[0].push(Item::Generator("elerium".to_string()));
    state.floors[0].push(Item::Microchip("elerium".to_string()));
    state.floors[0].push(Item::Generator("dilithium".to_string()));
    state.floors[0].push(Item::Microchip("dilithium".to_string()));
    solve(state)
}

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
