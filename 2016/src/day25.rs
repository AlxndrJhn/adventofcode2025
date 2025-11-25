#[aoc(day25, part1)]
pub fn part1(_input: &str) -> usize {
    let magic_number = 362 * 7; // derived from the first lines of the input program
    for i in 0.. {
        let candidate = i + magic_number;
        // check that the binary representation of candidate is alternating 0s and 1s
        let binary_rep = format!("{:b}", candidate);
        if binary_rep
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .all(|w| w[0] != w[1])
        {
            return i;
        }
    }
    unreachable!()
}
