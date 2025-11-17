#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    let screen = render(input.trim(), 50, 6);
    count_lit_pixels(&screen)
}

// #[aoc(day8, part2)]
// pub fn part2(input: &str) -> usize {
// }

fn render(instructions: &str, width: usize, height: usize) -> String {
    let mut screen = vec![vec!['.'; width]; height];

    for line in instructions.trim().lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "rect" => {
                let dims: Vec<usize> = parts[1].split('x').map(|x| x.parse().unwrap()).collect();
                let (w, h) = (dims[0], dims[1]);
                for y in 0..h {
                    for x in 0..w {
                        screen[y][x] = '#';
                    }
                }
            }
            "rotate" => {
                let index_part: Vec<&str> = parts[2].split('=').collect();
                let index: usize = index_part[1].parse().unwrap();
                let by: usize = parts[4].parse().unwrap();
                match parts[1] {
                    "row" => {
                        let row = &mut screen[index];
                        let mut new_row = vec!['.'; width];
                        for x in 0..width {
                            new_row[(x + by) % width] = row[x];
                        }
                        screen[index] = new_row;
                    }
                    "column" => {
                        let mut new_col = vec!['.'; height];
                        for y in 0..height {
                            new_col[(y + by) % height] = screen[y][index];
                        }
                        for y in 0..height {
                            screen[y][index] = new_col[y];
                        }
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }
            _ => {
                unreachable!()
            }
        }
    }

    screen
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn count_lit_pixels(screen: &str) -> usize {
    screen.chars().filter(|&c| c == '#').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            render("rect 3x2", 7, 3),
            "###....
###....
......."
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            render("rect 3x2\nrotate column x=1 by 1", 7, 3),
            "#.#....
###....
.#....."
        );
    }
    #[test]
    fn example_3() {
        assert_eq!(
            render(
                "rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4",
                7,
                3
            ),
            "....#.#
###....
.#....."
        );
    }
    #[test]
    fn example_4() {
        assert_eq!(
            render(
                "rect 3x2\nrotate column x=1 by 1\nrotate row y=0 by 4\nrotate column x=1 by 1",
                7,
                3
            ),
            ".#..#.#
#.#....
.#....."
        );
    }
}
