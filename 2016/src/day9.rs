#[aoc(day9, part1)]
pub fn part1(input: &str) -> usize {
    decompress_length(input)
}

fn decompress_length(input: &str) -> usize {
    let mut index_read = 0;
    let mut decompressed_length = String::new();
    while index_read < input.len() {
        let current_char = &input[index_read..index_read + 1];
        if current_char == "(" {
            let marker_end = input[index_read..]
                .find(")")
                .map(|i| i + index_read)
                .unwrap();
            let marker_definition = &input[index_read + 1..marker_end];
            let marker_parts = marker_definition.split("x").collect::<Vec<&str>>();
            let chars_to_take: usize = marker_parts[0].parse().unwrap();
            let repeat_count: usize = marker_parts[1].parse().unwrap();
            let start_of_sequence = marker_end + 1;
            let end_of_sequence = start_of_sequence + chars_to_take;
            let sequence = &input[start_of_sequence..end_of_sequence];
            for _ in 0..repeat_count {
                decompressed_length.push_str(sequence);
            }
            index_read = end_of_sequence;
        } else {
            decompressed_length.push_str(current_char);
            index_read += 1;
        }
    }
    decompressed_length.len()
}

#[aoc(day9, part2)]
pub fn part2(input: &str) -> usize {
    decompress_length2(input)
}

fn decompress_length2(input: &str) -> usize {
    let mut output_length = 0;
    let next_open_paren = input.find("(");
    match next_open_paren {
        Some(index) => {
            output_length += index;
            let marker_end = input[index..].find(")").map(|i| i + index).unwrap();
            let marker_definition = &input[index + 1..marker_end];
            let marker_parts = marker_definition.split("x").collect::<Vec<&str>>();
            let chars_to_take: usize = marker_parts[0].parse().unwrap();
            let repeat_count: usize = marker_parts[1].parse().unwrap();
            let start_of_sequence = marker_end + 1;
            let end_of_sequence = start_of_sequence + chars_to_take;
            let sequence = &input[start_of_sequence..end_of_sequence];
            let sequence_length = decompress_length2(sequence);
            output_length += sequence_length * repeat_count;
            let rest_of_string = &input[end_of_sequence..];
            output_length += decompress_length2(rest_of_string);
        }
        None => {
            output_length += input.len();
        }
    }
    output_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(decompress_length("ADVENT"), 6);
    }
    #[test]
    fn example_2() {
        assert_eq!(decompress_length("A(1x5)BC"), 7);
    }
    #[test]
    fn example_3() {
        assert_eq!(decompress_length("(3x3)XYZ"), 9);
    }
    #[test]
    fn example_4() {
        assert_eq!(decompress_length("A(2x2)BCD(2x2)EFG"), 11);
    }
    #[test]
    fn example_5() {
        assert_eq!(decompress_length("(6x1)(1x3)A"), 6);
    }
    #[test]
    fn example_6() {
        assert_eq!(decompress_length("X(8x2)(3x3)ABCY"), 18);
    }
    #[test]
    fn example_7() {
        assert_eq!(decompress_length2("(3x3)XYZ"), 9);
    }
    #[test]
    fn example_8() {
        assert_eq!(decompress_length2("X(8x2)(3x3)ABCY"), 20);
    }
    #[test]
    fn example_9() {
        assert_eq!(
            decompress_length2("(27x12)(20x12)(13x14)(7x10)(1x12)A"),
            241920
        );
    }
    #[test]
    fn example_10() {
        assert_eq!(
            decompress_length2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            445
        );
    }
}
