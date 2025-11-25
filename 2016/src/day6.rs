#[aoc(day6, part1)]
pub fn part1(input: &str) -> String {
    let input = input.trim();
    let message_length = input.lines().next().unwrap().len();
    // get most common char in each column
    let mut message = String::new();
    for col in 0..message_length {
        let mut counts = std::collections::HashMap::new();
        for line in input.lines() {
            let c = line.trim().chars().nth(col).unwrap();
            *counts.entry(c).or_insert(0) += 1;
        }
        let (&most_common_char, _) = counts.iter().max_by_key(|&(_, count)| count).unwrap();
        message.push(most_common_char);
    }
    message
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> String {
    let input = input.trim();
    let message_length = input.lines().next().unwrap().len();
    // get most common char in each column
    let mut message = String::new();
    for col in 0..message_length {
        let mut counts = std::collections::HashMap::new();
        for line in input.lines() {
            let c = line.trim().chars().nth(col).unwrap();
            *counts.entry(c).or_insert(0) += 1;
        }
        let (&least_common_char, _) = counts.iter().min_by_key(|&(_, count)| count).unwrap();
        message.push(least_common_char);
    }
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "eedadn
                drvtee
                eandsr
                raavrd
                atevrs
                tsrnev
                sdttsa
                rasrtv
                nssdts
                ntnada
                svetve
                tesnvt
                vntsnd
                vrdear
                dvrsen
                enarar
"
            ),
            "easter"
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                "eedadn
                drvtee
                eandsr
                raavrd
                atevrs
                tsrnev
                sdttsa
                rasrtv
                nssdts
                ntnada
                svetve
                tesnvt
                vntsnd
                vrdear
                dvrsen
                enarar
"
            ),
            "advent"
        );
    }
}
