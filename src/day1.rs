#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut start_location = (0, 0);
    let mut direction = 0; // 0 = north, 1 = east, 2 = south, 3 = west

    for instruction in input.split(", ") {
        let (turn, distance) = instruction.split_at(1);
        let distance: isize = distance.parse().unwrap();

        direction = match turn {
            "R" => (direction + 1) % 4,
            "L" => (direction + 3) % 4,
            _ => panic!("Invalid turn"),
        };

        match direction {
            0 => start_location.1 += distance,
            1 => start_location.0 += distance,
            2 => start_location.1 -= distance,
            3 => start_location.0 -= distance,
            _ => unreachable!(),
        }
    }

    (start_location.0.abs() + start_location.1.abs()) as usize
}

// #[aoc(day1, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(part1("R2, L3"), 5);
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("R2, R2, R2"), 2);
    }
    #[test]
    fn example_3() {
        assert_eq!(part1("R5, L5, R5, R3"), 12);
    }
}
