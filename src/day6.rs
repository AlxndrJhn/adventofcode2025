#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let input = input.trim();
    let inputs: Vec<Vec<isize>> = input
        .lines()
        .take(input.lines().count() - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<isize>().unwrap())
                .collect()
        })
        .collect();
    let operations: Vec<char> = input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|op| op.chars().next().unwrap())
        .collect();
    assert_eq!(inputs[0].len(), operations.len());

    let mut sum = 0;
    for (i, operation) in operations.iter().enumerate() {
        let mut nums: Vec<isize> = Vec::new();
        for row in &inputs {
            nums.push(row[i]);
        }
        let result = match *operation {
            '+' => nums.iter().sum::<isize>(),
            '*' => nums.iter().product::<isize>(),
            _ => panic!("Unknown operation"),
        };
        sum += result;
    }
    sum as usize
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let input: &str = input.trim();
    let matrix = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut index_cols = Vec::new();
    for col in 0..matrix[0].len() {
        if matrix.iter().all(|row| row[col].is_whitespace()) {
            index_cols.push(col);
        }
    }

    let inputs: Vec<Vec<String>> = matrix
        .iter()
        .take(matrix.len() - 1)
        .map(|row| {
            let mut new_row = Vec::new();
            let mut start = 0;
            for &col in &index_cols {
                let slice: String = row[start..col].iter().collect();
                new_row.push(slice.to_string());
                start = col + 1;
            }
            new_row.push(row[start..].iter().collect());
            new_row
        })
        .collect();

    let operations: Vec<char> = input
        .lines()
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .map(|op| op.chars().next().unwrap())
        .collect();
    assert_eq!(inputs[0].len(), operations.len());

    let mut sum = 0;
    index_cols.push(matrix[0].len());
    for (i, operation) in operations.iter().enumerate() {
        let mut width = index_cols[i];
        if i > 0 {
            width -= index_cols[i - 1] + 1;
        }
        let mut nums: Vec<isize> = Vec::new();

        for j in 0..width {
            let mut num_str = String::new();
            for row in &inputs {
                let c = row[i].chars().nth(j).unwrap();
                num_str.push(c);
            }
            let num = num_str.trim().parse::<isize>().unwrap();
            nums.push(num);
        }

        let result = match *operation {
            '+' => nums.iter().sum::<isize>(),
            '*' => nums.iter().product::<isize>(),
            _ => panic!("Unknown operation"),
        };
        sum += result;
    }
    sum as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "   123 328  51 64 
                     45 64  387 23 
                      6 98  215 314
                    *   +   *   +  
"
            ),
            4277556
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
            ),
            3263827
        );
    }
}
