#[derive(PartialEq)]
struct Node {
    x: usize,
    y: usize,
    size: usize,
    used: usize,
    avail: usize,
    use_perc: usize,
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> usize {
    let nodes = input
        .lines()
        .skip(2)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 5 {
                return None;
            }
            let coords: Vec<usize> = parts[0]
                .split('-')
                .skip(1)
                .map(|s| s[1..].parse().unwrap())
                .collect();
            Some(Node {
                x: coords[0],
                y: coords[1],
                size: parts[1][..parts[1].len() - 1].parse().unwrap(),
                used: parts[2][..parts[2].len() - 1].parse().unwrap(),
                avail: parts[3][..parts[3].len() - 1].parse().unwrap(),
                use_perc: parts[4][..parts[4].len() - 1].parse().unwrap(),
            })
        })
        .collect::<Vec<Node>>();
    let mut viable_pairs = 0;
    for a in &nodes {
        for b in &nodes {
            if a == b {
                continue;
            }
            if a.used > 0 && a.used <= b.avail {
                viable_pairs += 1;
            }
        }
    }
    viable_pairs
}

// #[aoc(day22, part2)]
// pub fn part2(input: &str) -> usize {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "root@ebhq-gridcenter# df -h
Filesystem              Size  Used  Avail  Use%
/dev/grid/node-x0-y0     92T   68T    24T   73%
/dev/grid/node-x0-y1     87T   73T    14T   83%
/dev/grid/node-x0-y2     89T   64T    25T   71%"
            ),
            0
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_3() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_4() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_5() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_6() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_7() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_8() {
        assert_eq!(part1("foo"), 0);
    }
}
