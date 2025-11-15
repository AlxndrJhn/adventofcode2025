pub fn process_input(input: &str, start_a: usize, return_b: bool) -> usize {
    let mut a: usize = start_a;
    let mut b: usize = 0;
    let instructions: Vec<&str> = input.trim().lines().collect();
    let mut pc: isize = 0;

    while (pc as usize) < instructions.len() {
        let line = instructions[pc as usize].trim();
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "hlf" => {
                let reg = parts[1];
                match reg {
                    "a" => a /= 2,
                    "b" => b /= 2,
                    _ => panic!("Unknown register"),
                }
                pc += 1;
            }
            "tpl" => {
                let reg = parts[1];
                match reg {
                    "a" => a *= 3,
                    "b" => b *= 3,
                    _ => panic!("Unknown register"),
                }
                pc += 1;
            }
            "inc" => {
                let reg = parts[1];
                match reg {
                    "a" => a += 1,
                    "b" => b += 1,
                    _ => panic!("Unknown register"),
                }
                pc += 1;
            }
            "jmp" => {
                let offset: isize = parts[1].parse().unwrap();
                pc += offset;
            }
            "jie" => {
                let reg = parts[1].trim_end_matches(',');
                let offset: isize = parts[2].parse().unwrap();
                match reg {
                    "a" => {
                        if a % 2 == 0 {
                            pc += offset;
                        } else {
                            pc += 1;
                        }
                    }
                    "b" => {
                        if b % 2 == 0 {
                            pc += offset;
                        } else {
                            pc += 1;
                        }
                    }
                    _ => panic!("Unknown register"),
                }
            }
            "jio" => {
                let reg = parts[1].trim_end_matches(',');
                let offset: isize = parts[2].parse().unwrap();
                match reg {
                    "a" => {
                        if a == 1 {
                            pc += offset;
                        } else {
                            pc += 1;
                        }
                    }
                    "b" => {
                        if b == 1 {
                            pc += offset;
                        } else {
                            pc += 1;
                        }
                    }
                    _ => panic!("Unknown register"),
                }
            }
            _ => panic!("Unknown instruction"),
        }
    }
    if return_b {
        b
    } else {
        a
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    process_input(input, 0, true)
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> usize {
    process_input(input, 1, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            process_input(
                "inc a
                jio a, +2
                tpl a
                inc a",
                0,
                false
            ),
            2
        );
    }
}
