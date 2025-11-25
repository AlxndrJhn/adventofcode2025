fn target_to_index(target: &str) -> usize {
    match target {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Unknown register {}", target),
    }
}

pub fn solve_program(input: &str, initial_c: isize) -> usize {
    let mut pc: isize = 0;
    let mut registers = [0; 4];
    registers[2] = initial_c;
    let program = input.lines().collect::<Vec<_>>();
    loop {
        let current_line = match program.get(pc as usize) {
            Some(line) => line,
            None => break,
        };
        let parts: Vec<&str> = current_line.split_whitespace().collect();
        match parts[0] {
            "cpy" => {
                let value_str = parts[1];
                let value = if let Ok(num) = value_str.parse::<isize>() {
                    num
                } else {
                    registers[target_to_index(parts[1])]
                };
                let target = parts[2];
                registers[target_to_index(target)] = value;
                pc += 1;
            }
            "inc" => {
                let target = parts[1];
                registers[target_to_index(target)] += 1;
                pc += 1;
            }
            "dec" => {
                let target = parts[1];
                registers[target_to_index(target)] -= 1;
                pc += 1;
            }
            "jnz" => {
                let value_str = parts[1];
                let value = if let Ok(num) = value_str.parse::<isize>() {
                    num
                } else {
                    registers[target_to_index(parts[1])]
                };
                let offset: isize = parts[2].parse().unwrap();
                if value != 0 {
                    pc = pc + offset;
                } else {
                    pc += 1;
                }
            }
            _ => panic!("Unknown instruction {}", parts[0]),
        }
    }
    registers[0] as usize
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {
    solve_program(input, 0)
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> usize {
    solve_program(input, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            part1(
                "cpy 41 a
inc a
inc a
dec a
jnz a 2
dec a
"
            ),
            42
        );
    }
    #[test]
    fn example_2() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_3() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_4() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_5() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_6() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_7() {
        assert_eq!(part1("foo"), 0);
    }
    #[test]
    fn example_8() {
        assert_eq!(part1("foo"), 0);
    }
}
