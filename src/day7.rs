use std::collections::HashMap;

pub fn resolve_wire_signal(input: &str) -> HashMap<&str, usize> {
    let input_trimmed = input.trim();
    let mut signals: HashMap<&str, usize> = HashMap::new();
    let instructions: Vec<(&str, Vec<&str>)> = input_trimmed
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let instr_parts: Vec<&str> = parts[0].split_whitespace().collect();
            (parts[1], instr_parts)
        })
        .collect();

    while signals.len() < instructions.len() {
        for (wire, instr_parts) in &instructions {
            if signals.contains_key(wire) {
                continue;
            }
            let signal = match instr_parts.as_slice() {
                [value] => {
                    if let Ok(num) = value.parse::<usize>() {
                        num
                    } else {
                        match signals.get(value) {
                            Some(&sig) => sig,
                            None => continue,
                        }
                    }
                }
                ["NOT", operand] => {
                    let val = if let Ok(num) = operand.parse::<usize>() {
                        num
                    } else {
                        match signals.get(operand) {
                            Some(&sig) => sig,
                            None => continue,
                        }
                    };
                    !val & 0xFFFF
                }
                [left, "AND", right] => {
                    let left_val = if let Ok(num) = left.parse::<usize>() {
                        num
                    } else {
                        match signals.get(left) {
                            Some(&sig) => sig,
                            None => continue,
                        }
                    };
                    let right_val = if let Ok(num) = right.parse::<usize>() {
                        num
                    } else {
                        match signals.get(right) {
                            Some(&sig) => sig,
                            None => continue,
                        }
                    };
                    left_val & right_val
                }
                [left, "OR", right] => {
                    let left_val = if let Ok(num) = left.parse::<usize>() {
                        num
                    } else {
                        match signals.get(left) {
                            Some(&sig) => sig,
                            None => continue,
                        }
                    };
                    let right_val = if let Ok(num) = right.parse::<usize>() {
                        num
                    } else {
                        match signals.get(right) {
                            Some(&sig) => sig,
                            None => continue,
                        }
                    };
                    left_val | right_val
                }
                [operand, "LSHIFT", shift] => {
                    let val = if let Ok(num) = operand.parse::<usize>() {
                        num
                    } else {
                        match signals.get(operand) {
                            Some(&sig) => sig,
                            None => continue,
                        }
                    };
                    let shift_val: usize = shift.parse().unwrap();
                    val << shift_val
                }
                [operand, "RSHIFT", shift] => {
                    let val = if let Ok(num) = operand.parse::<usize>() {
                        num
                    } else {
                        match signals.get(operand) {
                            Some(&sig) => sig,
                            None => continue,
                        }
                    };
                    let shift_val: usize = shift.parse().unwrap();
                    val >> shift_val
                }
                _ => panic!("invalid instruction"),
            };
            signals.insert(wire, signal);
        }
    }
    return signals;
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> usize {
    let signals = resolve_wire_signal(input);
    *signals.get("a").unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> usize {
    let signals = resolve_wire_signal(input);
    let result = *signals.get("a").unwrap();
    let mut modified_input = String::new();
    for line in input.trim().lines() {
        if line.ends_with("-> b") {
            modified_input.push_str(&format!("{} -> b\n", result));
        } else {
            modified_input.push_str(line);
            modified_input.push('\n');
        }
    }
    let new_signals = resolve_wire_signal(&modified_input);
    *new_signals.get("a").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "
123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let result = resolve_wire_signal(input);
        assert_eq!(result["d"], 72);
        assert_eq!(result["e"], 507);
        assert_eq!(result["f"], 492);
        assert_eq!(result["g"], 114);
        assert_eq!(result["h"], 65412);
        assert_eq!(result["i"], 65079);
        assert_eq!(result["x"], 123);
        assert_eq!(result["y"], 456);
    }
}
