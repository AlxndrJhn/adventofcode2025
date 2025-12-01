#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut times_at_zero = 0;
    let mut current = 50;
    for line in input.lines() {
        let step = line[1..].parse::<isize>().unwrap();
        match line[0..1].as_ref() {
            "L" => {
                current -= step;
            }
            "R" => {
                current += step;
            }
            _ => panic!("invalid input"),
        }
        current = (current + 100) % 100;
        if current == 0 {
            times_at_zero += 1;
        }
    }
    times_at_zero
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut times_at_zero = 0;
    let mut current = 50;
    for line in input.lines() {
        let step = line[1..].parse::<isize>().unwrap();
        let residue = step % 100;
        let cycles = step / 100;
        times_at_zero += cycles as usize;
        match line[0..1].as_ref() {
            "L" => {
                if residue >= current && current != 0 {
                    times_at_zero += 1;
                }
                current -= residue;
            }
            "R" => {
                if residue >= 100 - current && current != 0 {
                    times_at_zero += 1;
                }
                current += residue;
            }
            _ => panic!("invalid input"),
        }
        current = (current + 100) % 100;
    }
    times_at_zero
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
            ),
            3
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(
            part2(
                "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"
            ),
            6
        );
    }
}
