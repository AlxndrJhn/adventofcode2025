#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {
    let possible_triangles = input.lines().filter(|line| {
        let mut sides: Vec<usize> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();
        sides.sort_unstable();
        sides[0] + sides[1] > sides[2]
    });
    possible_triangles.count()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {
    let mut possible_triangles = 0;
    let lines: Vec<&str> = input.lines().collect();
    for chunk in lines.chunks(3) {
        let mut sides: Vec<Vec<usize>> = vec![vec![], vec![], vec![]];
        for line in chunk {
            for (i, num) in line.split_whitespace().enumerate() {
                sides[i].push(num.parse().unwrap());
            }
        }
        for side in sides {
            let mut sorted_side = side.clone();
            sorted_side.sort_unstable();
            if sorted_side[0] + sorted_side[1] > sorted_side[2] {
                possible_triangles += 1;
            }
        }
    }
    possible_triangles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("5 10 25"), 0);
    }
}
