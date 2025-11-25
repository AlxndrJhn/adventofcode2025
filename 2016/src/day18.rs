#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    count_safe_tiles(input.trim(), 40)
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> usize {
    count_safe_tiles(input.trim(), 400000)
}

fn count_safe_tiles(initial_row: &str, total_rows: usize) -> usize {
    let mut current_row: Vec<char> = initial_row.chars().collect();
    let mut row_count = 1;
    let mut safe_tile_count = current_row.iter().filter(|&&c| c == '.').count();
    loop {
        let mut next_row: Vec<char> = Vec::with_capacity(current_row.len());
        for i in 0..current_row.len() {
            let left = if i == 0 { '.' } else { current_row[i - 1] };
            let center = current_row[i];
            let right = if i == current_row.len() - 1 {
                '.'
            } else {
                current_row[i + 1]
            };

            let is_trap = (left == '^' && center == '^' && right == '.')
                || (left == '.' && center == '^' && right == '^')
                || (left == '^' && center == '.' && right == '.')
                || (left == '.' && center == '.' && right == '^');

            if is_trap {
                next_row.push('^');
            } else {
                next_row.push('.');
                safe_tile_count += 1;
            }
        }
        current_row = next_row;
        row_count += 1;
        if row_count >= total_rows {
            break;
        }
    }
    safe_tile_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(count_safe_tiles(".^^.^.^^^^", 10), 38);
    }
}
