use std::collections::{HashSet, VecDeque};

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    let regex_pattern = r"^\[([.#]+)\] ((?:\(\d+(?:,\d+)*\) )+)\{((?:\d+,?)+)\}$";
    let re = regex::Regex::new(regex_pattern).unwrap();
    let mut button_presses = 0;
    for line in input.lines() {
        if let Some(captures) = re.captures(line) {
            let is_on_lamps = &captures[1].chars().map(|c| c == '#').collect::<Vec<bool>>();

            let buttons: Vec<Vec<usize>> = captures[2]
                .trim()
                .split(' ')
                .map(|group| {
                    group
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .filter_map(|num| num.parse::<usize>().ok())
                        .collect()
                })
                .collect();

            button_presses += bfs(is_on_lamps.clone(), &buttons);
        }
    }
    button_presses
}

fn bfs(is_on_lamps: Vec<bool>, buttons: &Vec<Vec<usize>>) -> usize {
    let target_state = vec![false; is_on_lamps.len()];

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((is_on_lamps.clone(), 0));
    visited.insert(is_on_lamps.clone());

    while let Some((current_state, presses)) = queue.pop_front() {
        if *current_state == target_state {
            return presses;
        }

        for button in buttons {
            let mut next_state = current_state.clone();
            for &lamp_index in button {
                next_state[lamp_index] = !next_state[lamp_index];
            }

            if visited.insert(next_state.clone()) {
                queue.push_back((next_state, presses + 1));
            }
        }
    }

    usize::MAX
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            ),
            7
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}"), 2);
    }
    #[test]
    fn example_3() {
        assert_eq!(
            part1("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}"),
            3
        );
    }
    #[test]
    fn example_4() {
        assert_eq!(
            part1("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"),
            2
        );
    }
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
