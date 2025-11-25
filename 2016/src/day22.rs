#[derive(PartialEq, Clone, Hash, Eq)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
    use_perc: usize,
}
fn get_nodes(input: &str) -> Vec<Node> {
    input
        .lines()
        .skip(2)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 5 {
                return None;
            }
            let coords: Vec<usize> = parts[0]
                .split('-')
                .skip(1)
                .map(|s| s[1..].parse().unwrap())
                .collect();

            Some(Node {
                x: coords[0],
                y: coords[1],
                size: parts[1][..parts[1].len() - 1].parse().unwrap(),
                used: parts[2][..parts[2].len() - 1].parse().unwrap(),
                avail: parts[3][..parts[3].len() - 1].parse().unwrap(),
                use_perc: parts[4][..parts[4].len() - 1].parse().unwrap(),
            })
        })
        .collect::<Vec<Node>>()
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let nodes = get_nodes(input);
    let mut viable_pairs = 0;
    for a in &nodes {
        for b in &nodes {
            if a == b {
                continue;
            }
            if a.used > 0 && a.used <= b.avail {
                viable_pairs += 1;
            }
        }
    }
    viable_pairs
}

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Eq, PartialEq, Hash)]
struct SearchNode {
    open_pos: (usize, usize), // Position of the empty node
    goal_pos: (usize, usize), // Position of the goal data
}

#[derive(Eq, PartialEq)]
struct PQNode {
    cost: usize,
    heuristic: usize,
    state: SearchNode,
}

impl Ord for PQNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap
        (other.cost + other.heuristic).cmp(&(self.cost + self.heuristic))
    }
}

impl PartialOrd for PQNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn mdist(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize
}

fn astar(
    start: SearchNode,
    blocked: &HashSet<(usize, usize)>,
    max_x: usize,
    max_y: usize,
) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();

    let initial_h = mdist(start.open_pos, start.goal_pos) + mdist(start.goal_pos, (0, 0));
    heap.push(PQNode {
        cost: 0,
        heuristic: initial_h,
        state: start.clone(),
    });
    visited.insert(start, 0);

    while let Some(PQNode { cost, state, .. }) = heap.pop() {
        // Goal check: data is at (0, 0)
        if state.goal_pos == (0, 0) {
            return Some(cost);
        }

        // If we've already found a better path to this state, skip
        if let Some(&best_cost) = visited.get(&state) {
            if cost > best_cost {
                continue;
            }
        }

        // Check if current open position is blocked (shouldn't happen in valid states)
        if blocked.contains(&state.open_pos) {
            continue;
        }

        // Generate neighbors
        let (oy, ox) = state.open_pos;
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dy, dx) in directions.iter() {
            let ny = oy as i32 + dy;
            let nx = ox as i32 + dx;

            if nx < 0 || nx >= max_x as i32 || ny < 0 || ny >= max_y as i32 {
                continue;
            }

            let new_pos = (ny as usize, nx as usize);

            let new_state = if state.goal_pos == new_pos {
                // Moving empty node to goal position means goal data moves to old open position
                SearchNode {
                    open_pos: state.goal_pos,
                    goal_pos: state.open_pos,
                }
            } else {
                // Just moving the empty node
                SearchNode {
                    open_pos: new_pos,
                    goal_pos: state.goal_pos,
                }
            };

            let new_cost = cost + 1;

            if let Some(&prev_cost) = visited.get(&new_state) {
                if new_cost >= prev_cost {
                    continue;
                }
            }

            visited.insert(new_state.clone(), new_cost);
            let h =
                mdist(new_state.open_pos, new_state.goal_pos) + mdist(new_state.goal_pos, (0, 0));

            heap.push(PQNode {
                cost: new_cost,
                heuristic: h,
                state: new_state,
            });
        }
    }

    None
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> usize {
    // solution from https://www.reddit.com/r/adventofcode/comments/5jor9q/comment/dbhvxwo/
    let nodes = get_nodes(input);

    // Find grid dimensions
    let max_x = nodes.iter().map(|n| n.x).max().unwrap() + 1;
    let max_y = nodes.iter().map(|n| n.y).max().unwrap() + 1;

    // Find blocked nodes (size > 120) and empty node (used < 10%)
    let mut blocked = HashSet::new();
    let mut empty_pos = (0, 0);

    for node in &nodes {
        if node.size > 120 {
            blocked.insert((node.y, node.x));
        }
        if node.use_perc < 10 {
            empty_pos = (node.y, node.x);
        }
    }

    // Start state: empty node at its position, goal data at top-right
    let start = SearchNode {
        open_pos: empty_pos,
        goal_pos: (0, max_x - 1),
    };

    astar(start, &blocked, max_x, max_y).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y0     92T   68T    24T   73%
/dev/grid/node-x0-y1     87T   73T    14T   83%
/dev/grid/node-x0-y2     89T   64T    25T   71%"
            ),
            0
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                "\nFilesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%"
            ),
            7
        );
    }
}
