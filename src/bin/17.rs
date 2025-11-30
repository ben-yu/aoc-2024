advent_of_code::solution!(17);

struct Computer {
    reg_a: i64,
    reg_b: i64,
    reg_c: i64,
    program: Vec<u8>,
    ip: usize,
    output: Vec<u8>,
}

impl Computer {
    fn new(reg_a: i64, reg_b: i64, reg_c: i64, program: Vec<u8>) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            program,
            ip: 0,
            output: Vec::new(),
        }
    }

    fn combo_value(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Invalid combo operand: {}", operand),
        }
    }

    fn execute_instruction(&mut self) -> bool {
        if self.ip >= self.program.len() {
            return false;
        }

        let opcode = self.program[self.ip];
        let operand = self.program[self.ip + 1];

        match opcode {
            0 => {
                // adv: A = A / 2^combo_operand
                let denominator = 1 << self.combo_value(operand);
                self.reg_a /= denominator;
                self.ip += 2;
            }
            1 => {
                // bxl: B = B XOR literal_operand
                self.reg_b ^= operand as i64;
                self.ip += 2;
            }
            2 => {
                // bst: B = combo_operand % 8
                self.reg_b = self.combo_value(operand) % 8;
                self.ip += 2;
            }
            3 => {
                // jnz: if A != 0, jump to literal_operand
                if self.reg_a != 0 {
                    self.ip = operand as usize;
                } else {
                    self.ip += 2;
                }
            }
            4 => {
                // bxc: B = B XOR C (ignores operand)
                self.reg_b ^= self.reg_c;
                self.ip += 2;
            }
            5 => {
                // out: output combo_operand % 8
                let value = (self.combo_value(operand) % 8) as u8;
                self.output.push(value);
                self.ip += 2;
            }
            6 => {
                // bdv: B = A / 2^combo_operand
                let denominator = 1 << self.combo_value(operand);
                self.reg_b = self.reg_a / denominator;
                self.ip += 2;
            }
            7 => {
                // cdv: C = A / 2^combo_operand
                let denominator = 1 << self.combo_value(operand);
                self.reg_c = self.reg_a / denominator;
                self.ip += 2;
            }
            _ => panic!("Invalid opcode: {}", opcode),
        }

        true
    }

    fn run(&mut self) {
        while self.execute_instruction() {}
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn parse_input(input: &str) -> (i64, i64, i64, Vec<u8>) {
    let lines: Vec<&str> = input.lines().collect();

    let reg_a = lines[0]
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let reg_b = lines[1]
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let reg_c = lines[2]
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();

    let program = lines[4]
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    (reg_a, reg_b, reg_c, program)
}

pub fn part_one(input: &str) -> Option<String> {
    let (reg_a, reg_b, reg_c, program) = parse_input(input);
    let mut computer = Computer::new(reg_a, reg_b, reg_c, program);
    computer.run();
    Some(computer.get_output())
}

fn find_min_a_for_quine(program: &[u8]) -> Option<i64> {
    let n = program.len();

    // Start by finding values that produce just the last output
    let mut candidates = Vec::new();
    for a in 1..8 {
        let mut comp = Computer::new(a, 0, 0, program.to_vec());
        comp.run();
        if comp.output.len() == 1 && comp.output[0] == program[n - 1] {
            candidates.push(a);
        }
    }

    // Work backwards through the program, building up A
    for i in (0..n - 1).rev() {
        let mut next_candidates = Vec::new();

        for &prev_a in &candidates {
            // Try all 8 possible values for the next 3 bits
            for k in 0..8 {
                let test_a = prev_a * 8 + k;
                let mut comp = Computer::new(test_a, 0, 0, program.to_vec());
                comp.run();

                // Check if outputs match program[i..n]
                let expected_len = n - i;
                if comp.output.len() == expected_len {
                    let matches = comp.output.iter()
                        .zip(&program[i..])
                        .all(|(a, b)| a == b);

                    if matches {
                        next_candidates.push(test_a);
                    }
                }
            }
        }

        candidates = next_candidates;
        if candidates.is_empty() {
            return None;
        }
    }

    candidates.into_iter().min()
}

pub fn part_two(input: &str) -> Option<u64> {
    let (_, _, _, program) = parse_input(input);
    find_min_a_for_quine(&program).map(|v| v as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(117440));
    }
}
