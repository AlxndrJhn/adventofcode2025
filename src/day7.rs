use core::panic;
use std::collections::{HashMap, HashSet};

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let mut beams: HashSet<usize> = HashSet::new();
    let mut split_count = 0;
    for (y, line) in lines.iter().enumerate() {
        if y == 0 {
            beams.insert(line.find('S').unwrap());
            continue;
        }
        let line_chars: Vec<char> = line.chars().collect();
        let mut new_beams = HashSet::new();
        for &x in &beams {
            match line_chars[x] {
                '^' => {
                    new_beams.insert(x - 1);
                    new_beams.insert(x + 1);
                    split_count += 1;
                }
                '.' => {
                    new_beams.insert(x);
                }
                _ => {
                    panic!("Unexpected character");
                }
            }
        }
        beams = new_beams;
    }
    split_count
}

fn recursive_count(
    lines: &Vec<&str>,
    y: usize,
    x: usize,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if y == lines.len() - 1 {
        return 1;
    }
    if let Some(&count) = memo.get(&(y, x)) {
        return count;
    }
    let line_chars: Vec<char> = lines[y].chars().collect();
    let count = match line_chars[x] {
        '^' => {
            let left_count = recursive_count(lines, y + 1, x - 1, memo);
            let right_count = recursive_count(lines, y + 1, x + 1, memo);
            left_count + right_count
        }
        '.' => recursive_count(lines, y + 1, x, memo),
        _ => {
            panic!("Unexpected character");
        }
    };
    memo.insert((y, x), count);
    count
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let lines = input.trim().lines().collect::<Vec<&str>>();
    let start_x = lines[0].find('S').unwrap();
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    recursive_count(&lines, 1, start_x, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            ),
            21
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."
            ),
            40
        );
    }
}
