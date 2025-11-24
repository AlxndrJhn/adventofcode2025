use std::collections::{HashMap, VecDeque};

struct Location {
    x: usize,
    y: usize,
}

fn distance(map: &Vec<Vec<char>>, from: &Location, to: &Location) -> usize {
    let height = map.len();
    let width = map[0].len();
    let mut visited = vec![vec![false; width]; height];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((from.x, from.y, 0));
    visited[from.y][from.x] = true;
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    while let Some((x, y, dist)) = queue.pop_front() {
        if x == to.x && y == to.y {
            return dist;
        }
        for (dx, dy) in &directions {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;

            if new_x >= 0 && new_x < width as isize && new_y >= 0 && new_y < height as isize {
                let (nx, ny) = (new_x as usize, new_y as usize);
                if !visited[ny][nx] && map[ny][nx] != '#' {
                    visited[ny][nx] = true;
                    queue.push_back((nx, ny, dist + 1));
                }
            }
        }
    }
    usize::MAX
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let locations_of_interest: HashMap<isize, Location> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &ch)| {
                if ch.is_digit(10) {
                    Some((ch.to_digit(10).unwrap() as isize, Location { x, y }))
                } else {
                    None
                }
            })
        })
        .collect();
    let mut distances_of_interest: HashMap<(isize, isize), usize> = HashMap::new();
    for (&from_key, from_loc) in &locations_of_interest {
        for (&to_key, to_loc) in &locations_of_interest {
            if from_key != to_key {
                let dist = distance(&map, from_loc, to_loc);
                distances_of_interest.insert((from_key, to_key), dist);
            }
        }
    }
    let mut lowest_total_distance = usize::MAX;
    let mut queue = VecDeque::new();
    // (visited_keys, current_key, current_distance)
    queue.push_back((vec![0], 0, 0));
    while let Some((visited_keys, current_key, current_distance)) = queue.pop_front() {
        if visited_keys.len() == locations_of_interest.len() {
            if current_distance < lowest_total_distance {
                lowest_total_distance = current_distance;
            }
            continue;
        }
        for (&next_key, _) in &locations_of_interest {
            if !visited_keys.contains(&next_key) {
                let dist = distances_of_interest.get(&(current_key, next_key)).unwrap();
                let mut new_visited_keys = visited_keys.clone();
                new_visited_keys.push(next_key);
                queue.push_back((new_visited_keys, next_key, current_distance + dist));
            }
        }
    }
    lowest_total_distance
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> usize {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let locations_of_interest: HashMap<isize, Location> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &ch)| {
                if ch.is_digit(10) {
                    Some((ch.to_digit(10).unwrap() as isize, Location { x, y }))
                } else {
                    None
                }
            })
        })
        .collect();
    let mut distances_of_interest: HashMap<(isize, isize), usize> = HashMap::new();
    for (&from_key, from_loc) in &locations_of_interest {
        for (&to_key, to_loc) in &locations_of_interest {
            if from_key != to_key {
                let dist = distance(&map, from_loc, to_loc);
                distances_of_interest.insert((from_key, to_key), dist);
            }
        }
    }
    let mut lowest_total_distance = usize::MAX;
    let mut queue = VecDeque::new();
    // (visited_keys, current_key, current_distance)
    queue.push_back((vec![0], 0, 0));
    while let Some((visited_keys, current_key, current_distance)) = queue.pop_front() {
        if visited_keys.len() == locations_of_interest.len() + 1 {
            if current_distance < lowest_total_distance {
                lowest_total_distance = current_distance;
            }
            continue;
        }
        for (&next_key, _) in &locations_of_interest {
            if !visited_keys.contains(&next_key)
                || (visited_keys.len() == locations_of_interest.len() && next_key == 0)
            {
                let dist = distances_of_interest.get(&(current_key, next_key)).unwrap();
                let mut new_visited_keys = visited_keys.clone();
                new_visited_keys.push(next_key);
                queue.push_back((new_visited_keys, next_key, current_distance + dist));
            }
        }
    }
    lowest_total_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "###########
#0.1.....2#
#.#######.#
#4.......3#
###########
"
            ),
            14
        );
    }
    // #[test]
    // fn example_2() {
    //     assert_eq!(part1("foo"), 0);
    // }
    // #[test]
    // fn example_3() {
    //     assert_eq!(part1("foo"), 0);
    // }
    // #[test]
    // fn example_4() {
    //     assert_eq!(part1("foo"), 0);
    // }
    // #[test]
    // fn example_5() {
    //     assert_eq!(part1("foo"), 0);
    // }
    // #[test]
    // fn example_6() {
    //     assert_eq!(part1("foo"), 0);
    // }
    // #[test]
    // fn example_7() {
    //     assert_eq!(part1("foo"), 0);
    // }
    // #[test]
    // fn example_8() {
    //     assert_eq!(part1("foo"), 0);
    // }
}
