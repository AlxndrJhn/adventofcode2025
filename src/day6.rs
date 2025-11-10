#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let mut lights = [[false; 1000]; 1000];
    input.lines().for_each(|line| {
        let (action, rest) = if line.starts_with("turn on ") {
            ("turn on", &line[8..])
        } else if line.starts_with("turn off ") {
            ("turn off", &line[9..])
        } else if line.starts_with("toggle ") {
            ("toggle", &line[7..])
        } else {
            panic!("invalid input");
        };
        let parts: Vec<&str> = rest.split(" through ").collect();
        let start: Vec<usize> = parts[0].split(',').map(|s| s.parse().unwrap()).collect();
        let end: Vec<usize> = parts[1].split(',').map(|s| s.parse().unwrap()).collect();

        for x in start[0]..=end[0] {
            for y in start[1]..=end[1] {
                match action {
                    "turn on" => lights[x][y] = true,
                    "turn off" => lights[x][y] = false,
                    "toggle" => lights[x][y] = !lights[x][y],
                    _ => panic!("invalid action"),
                }
            }
        }
    });
    lights.iter().flat_map(|r| r.iter()).filter(|&&l| l).count()
}

// #[aoc(day6, part2)]
// pub fn part2(input: &str) -> usize {
//     input.lines().filter(|l| is_really_nice(l.trim())).count()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("turn on 0,0 through 999,999"), 1000000);
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("toggle 0,0 through 999,0"), 1000);
    }
    #[test]
    fn example_3() {
        assert_eq!(
            part1("turn on 0,0 through 999,999\nturn off 499,499 through 500,500"),
            1000000 - 4
        );
    }
}
