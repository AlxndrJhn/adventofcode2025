#[aoc(day13, part1)]
pub fn part1(input: &str) -> usize {
    let puzzle_input: usize = input.trim().parse().unwrap();
    find_shortest_steps(puzzle_input, 31, 39)
}

// #[aoc(day13, part2)]
// pub fn part2(input: &str) -> usize {
// }

fn is_wall(x: usize, y: usize, puzzle_input: usize) -> bool {
    let value = x * x + 3 * x + 2 * x * y + y + y * y + puzzle_input;
    value.count_ones() % 2 == 1
}

fn find_shortest_steps(puzzle_input: usize, target_x: isize, target_y: isize) -> usize {
    let start = (1, 1);
    let target = (target_x, target_y);
    let mut queue = std::collections::VecDeque::new();
    let mut visited = std::collections::HashSet::new();
    queue.push_back((start, 0));
    visited.insert(start);
    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == target {
            return steps;
        }
        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in directions {
            let new_x = x + dx;
            let new_y = y + dy;
            if new_x >= 0 && new_y >= 0 {
                if !is_wall(new_x as usize, new_y as usize, puzzle_input) {
                    let neighbor = (new_x, new_y);
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_back((neighbor, steps + 1));
                    }
                }
            }
        }
    }
    unreachable!()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_shortest_steps(10, 7, 4), 11);
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_3() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_4() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_5() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_6() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_7() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_8() {
        assert_eq!(part1("foo"), 0);
    }
}
