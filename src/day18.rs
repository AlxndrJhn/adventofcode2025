pub fn count_on_after_interations(input: &str, iterations: usize) -> usize {
    let grid_is_on: Vec<Vec<bool>> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|c| c == '#').collect())
        .collect();

    // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
    // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.

    let directions: Vec<(isize, isize)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut current_grid = grid_is_on;
    let height = current_grid.len() as isize;
    let width = current_grid[0].len() as isize;

    for _ in 0..iterations {
        let mut next_grid = current_grid.clone();
        for y in 0..height {
            for x in 0..width {
                let mut on_neighbors = 0;
                for (dy, dx) in &directions {
                    let ny = y + dy;
                    let nx = x + dx;
                    if ny >= 0 && ny < height && nx >= 0 && nx < width {
                        if current_grid[ny as usize][nx as usize] {
                            on_neighbors += 1;
                        }
                    }
                }
                if current_grid[y as usize][x as usize] {
                    // Light is currently on
                    if on_neighbors == 2 || on_neighbors == 3 {
                        next_grid[y as usize][x as usize] = true;
                    } else {
                        next_grid[y as usize][x as usize] = false;
                    }
                } else {
                    // Light is currently off
                    if on_neighbors == 3 {
                        next_grid[y as usize][x as usize] = true;
                    } else {
                        next_grid[y as usize][x as usize] = false;
                    }
                }
            }
        }
        current_grid = next_grid;
    }
    current_grid
        .iter()
        .map(|row| row.iter().filter(|&&cell| cell).count())
        .sum()
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    count_on_after_interations(input, 100)
}

// #[aoc(day18, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            count_on_after_interations(
                "   .#.#.#
                    ...##.
                    #....#
                    ..#...
                    #.#..#
                    ####..
",
                4
            ),
            4
        );
    }
}
