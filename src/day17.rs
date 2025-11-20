use std::collections::HashSet;

// #########
// #S| | | #
// #-#-#-#-#
// # | | | #
// #-#-#-#-#
// # | | | #
// #-#-#-#-#
// # | | |
// ####### V
fn hash(input: &str) -> String {
    let mut hasher = md5::Context::new();
    hasher.consume(input);
    let hex_str = format!("{:x}", hasher.finalize());
    hex_str
}

enum Directions {
    Up,
    Down,
    Left,
    Right,
}

fn dir_to_vec(dir: &Directions) -> (isize, isize) {
    match dir {
        Directions::Up => (0, -1),
        Directions::Down => (0, 1),
        Directions::Left => (-1, 0),
        Directions::Right => (1, 0),
    }
}

fn is_out_of_bounds(pos: (isize, isize)) -> bool {
    pos.0 >= 4 || pos.1 >= 4 || pos.0 < 0 || pos.1 < 0
}

fn dir_char(dir: &Directions) -> char {
    match dir {
        Directions::Up => 'U',
        Directions::Down => 'D',
        Directions::Left => 'L',
        Directions::Right => 'R',
    }
}

fn hash_to_door_is_open(hash: &str) -> Vec<bool> {
    hash.chars()
        .take(4)
        .map(|c| matches!(c, 'b' | 'c' | 'd' | 'e' | 'f'))
        .collect()
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let start = (0, 0);
    let end = (3, 3);
    let mut queue: Vec<((isize, isize), String)> = vec![];
    queue.push((start, input.to_string()));
    let mut visited: HashSet<String> = HashSet::new();
    let mut shortest_path: Option<String> = None;
    while !queue.is_empty() {
        let (pos, path) = queue.remove(0);
        if pos == end {
            if shortest_path.is_none() || path.len() < shortest_path.as_ref().unwrap().len() {
                shortest_path = Some(path.clone());
            }
            continue;
        }
        let hash = hash(&path);
        let door_states = hash_to_door_is_open(&hash);
        for (i, dir) in [
            Directions::Up,
            Directions::Down,
            Directions::Left,
            Directions::Right,
        ]
        .iter()
        .enumerate()
        {
            if door_states[i] {
                let movement = dir_to_vec(dir);
                let new_pos = (pos.0 + movement.0, pos.1 + movement.1);
                if is_out_of_bounds(new_pos) {
                    continue;
                }
                let mut new_path = path.clone();
                new_path.push(dir_char(dir));
                if !visited.contains(&new_path) {
                    visited.insert(new_path.clone());
                    queue.push((new_pos, new_path));
                }
            }
        }
    }
    if shortest_path.is_none() {
        return "".to_string();
    }
    // return the shortest path without the input prefix
    let shortest_path = shortest_path.unwrap();
    shortest_path[input.len()..].to_string()
}

// #[aoc(day17, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("ihgpwlah"), "DDRRRD");
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("hijkl"), "");
    }
    #[test]
    fn example_3() {
        assert_eq!(part1("kglvqrro"), "DDUDRLRRUDRD");
    }
    #[test]
    fn example_4() {
        assert_eq!(part1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    }
}
