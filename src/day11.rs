use std::collections::HashMap;

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

fn create_heuristic(
    target: &str,
    connections: &std::collections::HashMap<&str, Vec<&str>>,
) -> std::collections::HashMap<String, Option<usize>> {
    let mut node_dist_heuristic: std::collections::HashMap<String, Option<usize>> =
        std::collections::HashMap::new();
    for &node in connections.keys() {
        let mut stack = vec![(node, 0)];
        let mut visited = std::collections::HashSet::new();
        let mut dist_to_target = None;
        while let Some((current_node, dist)) = stack.pop() {
            if !visited.insert(current_node) {
                continue;
            }
            if current_node == target {
                dist_to_target = Some(dist);
                break;
            }
            if let Some(neighbors) = connections.get(current_node) {
                for &neighbor in neighbors {
                    stack.push((neighbor, dist + 1));
                }
            }
        }
        if let Some(dist) = dist_to_target {
            node_dist_heuristic.insert(node.to_string(), Some(dist));
        } else {
            node_dist_heuristic.insert(node.to_string(), None);
        }
    }
    node_dist_heuristic.insert(target.to_string(), Some(0));
    node_dist_heuristic
}

fn recursive_path_count(
    node: &str,
    target: &str,
    avoid: Option<&str>,
    connections: &std::collections::HashMap<&str, Vec<&str>>,
    heuristics: &HashMap<&str, HashMap<String, Option<usize>>>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if node == target {
        return 1;
    }
    if let Some(&count) = memo.get(&node.to_string()) {
        return count;
    }
    let mut total_paths = 0;
    if let Some(neighbors) = connections.get(node) {
        for &neighbor in neighbors {
            if Some(neighbor) == avoid {
                continue;
            }
            if heuristics
                .get(target)
                .and_then(|h| h.get(neighbor))
                .is_none()
            {
                continue;
            }
            total_paths +=
                recursive_path_count(neighbor, target, avoid, connections, heuristics, memo);
        }
    }
    memo.insert(node.to_string(), total_paths);
    total_paths
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    let connections: std::collections::HashMap<_, Vec<_>> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let node = parts.next().unwrap();
            let edges = parts.next().unwrap().split_whitespace().collect::<Vec<_>>();
            (node, edges)
        })
        .collect();

    let mut heuristics: HashMap<&str, HashMap<String, Option<usize>>> = HashMap::new();
    heuristics.insert("fft", create_heuristic("fft", &connections));
    heuristics.insert("dac", create_heuristic("dac", &connections));
    heuristics.insert("out", create_heuristic("out", &connections));

    let paths_via_fft_first = recursive_path_count(
        "svr",
        "fft",
        Some("dac"),
        &connections,
        &heuristics,
        &mut HashMap::new(),
    ) * recursive_path_count(
        "fft",
        "dac",
        None,
        &connections,
        &heuristics,
        &mut HashMap::new(),
    ) * recursive_path_count(
        "dac",
        "out",
        None,
        &connections,
        &heuristics,
        &mut HashMap::new(),
    );
    let paths_via_dac_first = recursive_path_count(
        "svr",
        "dac",
        Some("fft"),
        &connections,
        &heuristics,
        &mut HashMap::new(),
    ) * recursive_path_count(
        "dac",
        "fft",
        None,
        &connections,
        &heuristics,
        &mut HashMap::new(),
    ) * recursive_path_count(
        "fft",
        "out",
        None,
        &connections,
        &heuristics,
        &mut HashMap::new(),
    );
    paths_via_fft_first + paths_via_dac_first
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
    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out"
            ),
            2
        );
    }
}
