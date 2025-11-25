struct Disc {
    positions: usize,
    start_position: usize,
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> usize {
    let mut disc_states = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let positions: usize = parts[3].parse().unwrap();
        let start_position: usize = parts[11].trim_end_matches('.').parse().unwrap();
        disc_states.push(Disc {
            positions,
            start_position,
        });
    }
    for time in 0.. {
        if disc_states
            .iter()
            .enumerate()
            .all(|(i, disc)| (disc.start_position + time + i + 1) % disc.positions == 0)
        {
            return time;
        }
    }
    unreachable!()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> usize {
    let mut disc_states = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let positions: usize = parts[3].parse().unwrap();
        let start_position: usize = parts[11].trim_end_matches('.').parse().unwrap();
        disc_states.push(Disc {
            positions,
            start_position,
        });
    }
    disc_states.push(Disc {
        positions: 11,
        start_position: 0,
    });
    for time in 0.. {
        if disc_states
            .iter()
            .enumerate()
            .all(|(i, disc)| (disc.start_position + time + i + 1) % disc.positions == 0)
        {
            return time;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.
"
            ),
            5
        );
    }
}
