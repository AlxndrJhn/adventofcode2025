use core::panic;
use good_lp::{
    default_solver, variable, Expression, ProblemVariables, Solution, SolverModel, Variable,
};
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
    let regex_pattern = r"^\[([.#]+)\] ((?:\(\d+(?:,\d+)*\) )+)\{((?:\d+,?)+)\}$";
    let re = regex::Regex::new(regex_pattern).unwrap();
    let mut button_presses = 0;
    for line in input.lines() {
        if let Some(captures) = re.captures(line) {
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

            let joltage_ratings: Vec<usize> = captures[3]
                .trim()
                .split(',')
                .filter_map(|num| num.parse::<usize>().ok())
                .collect();

            button_presses += lp(&buttons, &joltage_ratings);
        }
    }
    button_presses
}

fn lp(buttons: &Vec<Vec<usize>>, target_state: &Vec<usize>) -> usize {
    let d = target_state.len();
    let n = buttons.len();

    // map buttons to input vectors of dimension d
    let mut inputs: Vec<Vec<f64>> = vec![vec![0.0; d]; n];
    for (j, button) in buttons.iter().enumerate() {
        for &lamp_index in button {
            inputs[j][lamp_index] = 1.0;
        }
    }

    let mut vars = ProblemVariables::new();
    let x: Vec<Variable> = (0..n)
        .map(|_| vars.add(variable().integer().min(0)))
        .collect();

    let mut objective = Expression::from(0.0);
    for xi in &x {
        objective += *xi;
    }

    let mut problem = vars.minimise(objective).using(default_solver);

    // Constraints: for each dimension k: sum_j inputs[j][k] * x_j = target[k]
    for k in 0..d {
        let mut expr = Expression::from(0.0);
        for j in 0..n {
            let coeff = inputs[j][k];
            if coeff != 0.0 {
                expr += coeff * x[j];
            }
        }
        problem = problem.with(expr.eq(target_state[k] as f64));
    }

    let solution = problem.solve();
    if solution.is_err() {
        panic!("No solution found");
    }
    let solution = solution.unwrap();

    let result: Vec<f64> = x.iter().map(|xi| solution.value(*xi)).collect();
    result.iter().sum::<f64>() as usize
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
    #[test]
    fn example_5() {
        assert_eq!(
            part2(
                "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"
            ),
            33
        );
    }
}
