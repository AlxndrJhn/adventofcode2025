use std::collections::HashSet;

#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    let coords = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: isize = parts.next().unwrap().parse().unwrap();
            let y: isize = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect::<Vec<(isize, isize)>>();

    let mut max_size = 0;
    for i in 0..coords.len() - 1 {
        for j in i + 1..coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];
            let size = ((1 + (x2 - x1).abs()) * (1 + (y2 - y1).abs())) as usize;
            if size > max_size {
                max_size = size;
            }
        }
    }
    max_size
}

fn is_inside_polygon(
    x: isize,
    y: isize,
    vertices: &Vec<(isize, isize)>,
    memo: &mut std::collections::HashMap<(isize, isize), bool>,
) -> bool {
    if let Some(&result) = memo.get(&(x, y)) {
        return result;
    }

    // Check if point is on an edge
    let n = vertices.len();
    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n];

        // Check if point is on the line segment between (x1, y1) and (x2, y2)
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);

        if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
            // Check collinearity using cross product
            let dx1 = x - x1;
            let dy1 = y - y1;
            let dx2 = x2 - x1;
            let dy2 = y2 - y1;
            if dx1 * dy2 == dy1 * dx2 {
                memo.insert((x, y), true);
                return true;
            }
        }
    }

    // Ray casting algorithm for interior points
    let mut inside = false;
    for i in 0..n {
        let (x1, y1) = vertices[i];
        let (x2, y2) = vertices[(i + 1) % n];
        if ((y1 > y) != (y2 > y)) && (x < (x2 - x1) * (y - y1) / (y2 - y1) + x1) {
            inside = !inside;
        }
    }
    memo.insert((x, y), inside);
    inside
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    let coords = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x: usize = parts.next().unwrap().parse().unwrap();
            let y: usize = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect::<Vec<(usize, usize)>>();
    let width = coords.iter().map(|(x, _)| *x).max().unwrap() + 1;
    let height = coords.iter().map(|(_, y)| *y).max().unwrap() + 1;

    let mut empty_rows: Vec<usize> = Vec::new();
    let mut empty_cols: Vec<usize> = Vec::new();
    let distinct_x: HashSet<usize> = coords.iter().map(|(x, _)| *x).collect();
    let distinct_y: HashSet<usize> = coords.iter().map(|(_, y)| *y).collect();
    for y in 1..height - 1 {
        if !distinct_y.contains(&y) {
            empty_rows.push(y);
        }
    }
    for x in 1..width - 1 {
        if !distinct_x.contains(&x) {
            empty_cols.push(x);
        }
    }
    let compressed_coords: Vec<(usize, usize)> = coords
        .iter()
        .map(|(x, y)| {
            let new_x = x - empty_cols.iter().filter(|&&col| col < *x).count();
            let new_y = y - empty_rows.iter().filter(|&&row| row < *y).count();
            (new_x, new_y)
        })
        .collect();
    let new_width = width - empty_cols.len() + 1;
    let new_height = height - empty_rows.len() + 1;

    let mut grid_is_red_green: Vec<Vec<bool>> = vec![vec![false; new_width]; new_height];
    for i in 0..compressed_coords.len() {
        let (x1, y1) = compressed_coords[i];
        let (x2, y2) = compressed_coords[(i + 1) % compressed_coords.len()];
        let min_x = x1.min(x2);
        let max_x = x1.max(x2);
        let min_y = y1.min(y2);
        let max_y = y1.max(y2);
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                grid_is_red_green[y][x] = true;
            }
        }
    }

    let mut grid_is_outside: Vec<Vec<bool>> = vec![vec![false; new_width]; new_height];
    grid_is_outside[0][0] = true;
    let mut front = vec![(0isize, 0isize)];
    while !front.is_empty() {
        let mut new_front = Vec::new();
        for (x, y) in front {
            let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
            for (nx, ny) in neighbors {
                if nx >= 0
                    && nx < new_width as isize
                    && ny >= 0
                    && ny < new_height as isize
                    && !grid_is_red_green[ny as usize][nx as usize]
                    && !grid_is_outside[ny as usize][nx as usize]
                {
                    grid_is_outside[ny as usize][nx as usize] = true;
                    new_front.push((nx, ny));
                }
            }
        }
        front = new_front;
    }
    for x in 0..new_width {
        for y in 0..new_height {
            if !grid_is_red_green[y][x] && !grid_is_outside[y][x] {
                grid_is_red_green[y][x] = true;
            }
        }
    }

    let mut max_area = 0;
    for i in 0..compressed_coords.len() - 1 {
        for j in i + 1..compressed_coords.len() {
            let (x1, y1) = coords[i];
            let (x2, y2) = coords[j];
            let size = (1 + x2.abs_diff(x1)) * (1 + y2.abs_diff(y1));
            if size <= max_area {
                continue;
            }

            let (x1, y1) = compressed_coords[i];
            let (x2, y2) = compressed_coords[j];
            let mut all_inside = true;
            for x in x1.min(x2)..=x1.max(x2) {
                for y in y1.min(y2)..=y1.max(y2) {
                    if !grid_is_red_green[y][x] {
                        all_inside = false;
                        break;
                    }
                }
                if !all_inside {
                    break;
                }
            }
            if all_inside {
                max_area = size;
            }
        }
    }
    max_area
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
            ),
            50
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"
            ),
            24
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
