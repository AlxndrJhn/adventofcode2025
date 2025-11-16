pub fn get_winner_distance(input: &str, total_time: usize) -> usize {
    let re = regex::Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    let mut max_distance = 0;

    for cap in re.captures_iter(input) {
        let speed: usize = cap[2].parse().unwrap();
        let fly_time: usize = cap[3].parse().unwrap();
        let rest_time: usize = cap[4].parse().unwrap();

        let cycle_time = fly_time + rest_time;
        let full_cycles = total_time / cycle_time;
        let remaining_time = total_time % cycle_time;

        let distance = full_cycles * speed * fly_time + speed * remaining_time.min(fly_time);

        if distance > max_distance {
            max_distance = distance;
        }
    }

    max_distance
}

pub fn get_winner_points(input: &str, total_time: usize) -> usize {
    let re = regex::Regex::new(
        r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
    )
    .unwrap();

    struct Reindeer {
        speed: usize,
        fly_time: usize,
        rest_time: usize,
        distance: usize,
        points: usize,
        time_in_state: usize,
        flying: bool,
    }

    let mut reindeers: Vec<Reindeer> = re
        .captures_iter(input)
        .map(|cap| Reindeer {
            speed: cap[2].parse().unwrap(),
            fly_time: cap[3].parse().unwrap(),
            rest_time: cap[4].parse().unwrap(),
            distance: 0,
            points: 0,
            time_in_state: 0,
            flying: true,
        })
        .collect();

    for _ in 0..total_time {
        for reindeer in reindeers.iter_mut() {
            if reindeer.flying {
                reindeer.distance += reindeer.speed;
            }
            reindeer.time_in_state += 1;

            if reindeer.flying && reindeer.time_in_state == reindeer.fly_time {
                reindeer.flying = false;
                reindeer.time_in_state = 0;
            } else if !reindeer.flying && reindeer.time_in_state == reindeer.rest_time {
                reindeer.flying = true;
                reindeer.time_in_state = 0;
            }
        }

        let max_distance = reindeers.iter().map(|r| r.distance).max().unwrap();
        for reindeer in reindeers.iter_mut() {
            if reindeer.distance == max_distance {
                reindeer.points += 1;
            }
        }
    }

    reindeers.iter().map(|r| r.points).max().unwrap()
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> usize {
    get_winner_distance(input, 2503)
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> usize {
    get_winner_points(input, 2503)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            get_winner_distance(
                r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#,
                1000
            ),
            1120
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            get_winner_points(
                r#"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
        Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds."#,
                1000
            ),
            689
        );
    }
}
