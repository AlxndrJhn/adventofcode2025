use std::collections::HashMap;

fn connect(input: &str, connections: Option<usize>, part2_flag: bool) -> usize {
    let input_coords = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    let mut distances = Vec::new();
    for i in 0..input_coords.len() - 1 {
        for j in i + 1..input_coords.len() {
            let coord1 = &input_coords[i];
            let coord2 = &input_coords[j];
            let dx = coord1[0].abs_diff(coord2[0]);
            let dy = coord1[1].abs_diff(coord2[1]);
            let dz = coord1[2].abs_diff(coord2[2]);
            let dist = dx.pow(2) + dy.pow(2) + dz.pow(2);
            distances.push((dist, (i, j)));
        }
    }
    distances.sort_by_key(|&(dist, _)| dist);
    let mut node_to_cluster = HashMap::new();
    let mut cluster_count = 0;
    let mut cluster_to_count = HashMap::new();
    let mut used_connections = 0;
    for (_, (i, j)) in distances {
        if let Some(max_connections) = connections {
            if used_connections >= max_connections {
                break;
            }
        }
        let coords_i = &input_coords[i];
        let coords_j = &input_coords[j];
        let cluster_i = node_to_cluster.get(&i).cloned();
        let cluster_j = node_to_cluster.get(&j).cloned();
        match (cluster_i, cluster_j) {
            (Some(ci), Some(cj)) => {
                if ci != cj {
                    if part2_flag
                        && cluster_to_count[&ci] + cluster_to_count[&cj] >= input_coords.len()
                    {
                        return coords_i[0] * coords_j[0];
                    } else {
                        let count_j = cluster_to_count.remove(&cj).unwrap();
                        *cluster_to_count.get_mut(&ci).unwrap() += count_j;
                        for (_node, cluster) in node_to_cluster.iter_mut() {
                            if *cluster == cj {
                                *cluster = ci;
                            }
                        }
                    }
                }
                used_connections += 1;
            }
            (Some(ci), None) => {
                node_to_cluster.insert(j, ci);
                *cluster_to_count.get_mut(&ci).unwrap() += 1;
                if part2_flag && cluster_to_count[&ci] >= input_coords.len() {
                    return coords_i[0] * coords_j[0];
                }
                used_connections += 1;
            }
            (None, Some(cj)) => {
                node_to_cluster.insert(i, cj);
                *cluster_to_count.get_mut(&cj).unwrap() += 1;
                if part2_flag && cluster_to_count[&cj] >= input_coords.len() {
                    return coords_i[0] * coords_j[0];
                }
                used_connections += 1;
            }
            (None, None) => {
                node_to_cluster.insert(i, cluster_count);
                node_to_cluster.insert(j, cluster_count);
                cluster_to_count.insert(cluster_count, 2);
                cluster_count += 1;
                used_connections += 1;
            }
        }
    }
    let mut sorted_cluster_sizes: Vec<usize> = cluster_to_count.values().cloned().collect();
    sorted_cluster_sizes.sort_unstable_by(|a, b| b.cmp(a));
    sorted_cluster_sizes.iter().take(3).product()
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> usize {
    connect(input, Some(1000), false)
}

#[aoc(day8, part2)]
pub fn part2(input: &str) -> usize {
    connect(input, None, true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            connect(
                "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
",
                Some(10),
                false
            ),
            40
        );
    }
    #[test]
    fn example_2e() {
        assert_eq!(
            connect(
                "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
",
                None,
                true
            ),
            25272
        );
    }
}
