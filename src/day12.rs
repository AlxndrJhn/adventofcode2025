struct Tree {
    width: usize,
    height: usize,
    repeated_gifts: Vec<usize>,
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    let trees = input
        .lines()
        .skip(30)
        .map(|line| {
            let mut parts = line.split(':');
            let dimensions = parts
                .next()
                .unwrap()
                .split('x')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let expected_gifts = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut repeated_gifts = Vec::new();
            for (i, &count) in expected_gifts.iter().enumerate() {
                for _ in 0..count {
                    repeated_gifts.push(i);
                }
            }
            Tree {
                width: dimensions[0],
                height: dimensions[1],
                repeated_gifts,
            }
        })
        .collect::<Vec<Tree>>();
    let mut valid_trees_count = 0;
    for tree in trees {
        let area = tree.width * tree.height;
        if tree.repeated_gifts.len() * 9 <= area {
            valid_trees_count += 1;
        }
    }
    valid_trees_count
}
