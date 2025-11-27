#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    let num_matrix = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    num_matrix
        .iter()
        .map(|row| {
            let min = row.iter().min().unwrap();
            let max = row.iter().max().unwrap();
            max - min
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    let num_matrix = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num_str| num_str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();
    num_matrix
        .iter()
        .map(|row| {
            for (i, &num1) in row.iter().enumerate() {
                for &num2 in row.iter().skip(i + 1) {
                    if num1 % num2 == 0 {
                        return num1 / num2;
                    } else if num2 % num1 == 0 {
                        return num2 / num1;
                    }
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "5 1 9 5
7 5 3
2 4 6 8"
            ),
            18
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                "5 9 2 8
9 4 7 3
3 8 6 5"
            ),
            9
        );
    }
}
