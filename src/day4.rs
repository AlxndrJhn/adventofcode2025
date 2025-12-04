#[aoc(day4, part1)]
pub fn part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '@' {
                let mut adjacent = 0;
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }
                        let nr = r as isize + dr;
                        let nc = c as isize + dc;
                        if nr >= 0
                            && nr < rows as isize
                            && nc >= 0
                            && nc < cols as isize
                            && grid[nr as usize][nc as usize] == '@'
                        {
                            adjacent += 1;
                        }
                    }
                }
                if adjacent < 4 {
                    count += 1;
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut removed_count = 0;
    loop {
        let mut any_removed = false;
        let mut next_grid = grid.clone();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    let mut adjacent = 0;
                    for dr in -1..=1 {
                        for dc in -1..=1 {
                            if dr == 0 && dc == 0 {
                                continue;
                            }
                            let nr = r as isize + dr;
                            let nc = c as isize + dc;
                            if nr >= 0
                                && nr < rows as isize
                                && nc >= 0
                                && nc < cols as isize
                                && grid[nr as usize][nc as usize] == '@'
                            {
                                adjacent += 1;
                            }
                        }
                    }
                    if adjacent < 4 {
                        next_grid[r][c] = '.';
                        any_removed = true;
                        removed_count += 1;
                    }
                }
            }
        }
        if !any_removed {
            break;
        }
        grid = next_grid;
    }
    removed_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            ),
            13
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            ),
            43
        );
    }
}
