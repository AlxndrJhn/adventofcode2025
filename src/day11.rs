#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let connections: std::collections::HashMap<_, Vec<_>> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let node = parts.next().unwrap();
            let edges = parts.next().unwrap().split_whitespace().collect::<Vec<_>>();
            (node, edges)
        })
        .collect();

    let mut visited = std::collections::HashSet::new();
    let mut stack = vec![("you", vec![])];
    let end_node = "out";
    let mut ways_to_end = 0;
    while let Some((node, path)) = stack.pop() {
        if !visited.insert((node, path.clone())) {
            continue;
        }
        if node == end_node {
            ways_to_end += 1;
            continue;
        }
        if let Some(neighbors) = connections.get(node) {
            for &neighbor in neighbors {
                let mut new_path = path.clone();
                new_path.push(node);
                stack.push((neighbor, new_path));
            }
        }
    }

    ways_to_end
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out"
            ),
            5
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
