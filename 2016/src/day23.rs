fn target_to_index(target: &str) -> usize {
    match target {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        _ => panic!("Unknown register {}", target),
    }
}

pub fn solve_program(input: &str, initial_a: isize) -> usize {
    let mut pc: isize = 0;
    let mut registers = [0; 4];
    registers[0] = initial_a;
    let mut program = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    loop {
        let current_line = match program.get(pc as usize) {
            Some(line) => line,
            None => break,
        };

        // optimization: detect loop
        // match this pattern:
        // #cpy b c
        // #inc a    \
        // #dec c     > inc_by a c
        // #jnz c -2 /
        // #dec d
        // #jnz d -5
        // replace with
        // # a += b * d; d = 0; c = 0 (b might be reg or imm)
        let this_and_next_five_lines = program
            .get(pc as usize..(pc + 6) as usize)
            .unwrap_or(&[])
            .to_vec();
        if this_and_next_five_lines.len() == 6 {
            if let [cpy_line, inc_line, dec_c_line, jnz_c_line, dec_d_line, jnz_d_line] =
                &this_and_next_five_lines[..]
            {
                let cpy_parts: Vec<&str> = cpy_line.split_whitespace().collect();
                let inc_parts: Vec<&str> = inc_line.split_whitespace().collect();
                let dec_c_parts: Vec<&str> = dec_c_line.split_whitespace().collect();
                let jnz_c_parts: Vec<&str> = jnz_c_line.split_whitespace().collect();
                let dec_d_parts: Vec<&str> = dec_d_line.split_whitespace().collect();
                let jnz_d_parts: Vec<&str> = jnz_d_line.split_whitespace().collect();
                if cpy_parts[0] == "cpy"
                    && inc_parts[0] == "inc"
                    && dec_c_parts[0] == "dec"
                    && jnz_c_parts[0] == "jnz"
                    && jnz_c_parts[1] == dec_c_parts[1]
                    && jnz_c_parts[2] == "-2"
                    && dec_d_parts[0] == "dec"
                    && jnz_d_parts[0] == "jnz"
                    && jnz_d_parts[1] == dec_d_parts[1]
                    && jnz_d_parts[2] == "-5"
                {
                    let b_value = if let Ok(num) = cpy_parts[1].parse::<isize>() {
                        num
                    } else {
                        registers[target_to_index(cpy_parts[1])]
                    };
                    let d_target = dec_d_parts[1];
                    let d_value = registers[target_to_index(d_target)];
                    let inc_target = inc_parts[1];
                    registers[target_to_index(inc_target)] += b_value * d_value;
                    registers[target_to_index(d_target)] = 0;
                    let c_target = dec_c_parts[1];
                    registers[target_to_index(c_target)] = 0;
                    pc += 6;
                    continue;
                }
            }
        }

        let parts: Vec<&str> = current_line.split_whitespace().collect();
        match parts[0] {
            "tgl" => {
                let target = parts[1];
                let target_index = if let Ok(num) = target.parse::<isize>() {
                    pc + num
                } else {
                    pc + registers[target_to_index(target)]
                };
                if let Some(line) = program.get(target_index as usize) {
                    let line_parts: Vec<&str> = line.split_whitespace().collect();
                    let new_instruction = match line_parts[0] {
                        "inc" => format!("dec {}", line_parts[1]),
                        "dec" => format!("inc {}", line_parts[1]),
                        "tgl" => format!("inc {}", line_parts[1]),
                        "jnz" => {
                            let both_params_are_numbers = line_parts[1].parse::<isize>().is_ok()
                                && line_parts[2].parse::<isize>().is_ok();
                            if !both_params_are_numbers {
                                format!("cpy {} {}", line_parts[1], line_parts[2])
                            } else {
                                line.to_string()
                            }
                        }
                        "cpy" => {
                            format!("jnz {} {}", line_parts[1], line_parts[2])
                        }
                        _ => line.to_string(),
                    };
                    program[target_index as usize] = new_instruction;
                }
                pc += 1;
            }
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

                let offset = if let Ok(num) = parts[2].parse::<isize>() {
                    num
                } else {
                    registers[target_to_index(parts[2])]
                };

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

#[aoc(day23, part1)]
pub fn part1(input: &str) -> usize {
    solve_program(input, 7)
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> usize {
    solve_program(input, 12)
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
        assert_eq!(
            part1(
                "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a
"
            ),
            3
        );
    }
}
