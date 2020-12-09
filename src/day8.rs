use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq)]
enum InstructionType {
    ACC,
    JMP,
    NOP,
}

impl InstructionType {
    pub fn new(instr: &str) -> InstructionType {
        match instr {
            "acc" => InstructionType::ACC,
            "jmp" => InstructionType::JMP,
            "nop" => InstructionType::NOP,
            _ => panic!("Wtf just happened here :O"),
        }
    }
}

#[derive(Clone)]
struct Instruction {
    instruction: InstructionType,
    value: i32,
}

fn parse_instruction(line: &str) -> Instruction {
    let split = line.split_whitespace().collect::<Vec<_>>();
    let instruction = InstructionType::new(split[0]);
    let value = split[1].parse::<i32>().unwrap();
    Instruction { instruction, value }
}

struct State {
    acc: i32,
    line: usize,
}

impl State {
    fn get_line(&self) -> usize {
        self.line
    }

    fn step(&mut self, instruction: &Instruction) {
        match instruction.instruction {
            InstructionType::ACC => {
                self.acc += instruction.value;
                self.line += 1;
            }
            InstructionType::JMP => {
                self.line = (self.line as i32 + instruction.value) as usize;
            }
            InstructionType::NOP => {
                self.line += 1;
            }
        }
    }
}

#[aoc_generator(day8)]
fn generator_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|l| parse_instruction(l))
        .collect::<Vec<_>>()
}

#[aoc(day8, part1)]
fn part1(input: &[Instruction]) -> i32 {
    let mut state = State { acc: 0, line: 0 };
    let mut seen = HashSet::new();
    while !seen.contains(&state.get_line()) {
        seen.insert(state.get_line());
        state.step(&input[state.get_line()]);
    }
    state.acc
}

fn step_to_termination(
    state: &mut State,
    seen: &mut HashSet<usize>,
    instructions: &[Instruction],
) -> Option<State> {
    if state.get_line() == instructions.len() {
        return Some(State {
            acc: state.acc,
            line: state.line,
        });
    } else if seen.contains(&state.get_line()) {
        return None;
    }
    seen.insert(state.get_line());

    state.step(&instructions[state.get_line()]);

    step_to_termination(state, seen, instructions)
}

#[aoc(day8, part2)]
fn part2(input: &[Instruction]) -> i32 {
    for i in 0..input.len() {
        // This is probably terrible but I've no idea how to do this without the cloning.
        // I'm fighting more with the compiler than the actual problem :D
        let mut changed = input.iter().cloned().collect::<Vec<Instruction>>();
        match input[i].instruction {
            InstructionType::JMP => {
                changed[i].instruction = InstructionType::NOP;
            }
            InstructionType::NOP => {
                changed[i].instruction = InstructionType::JMP;
            }
            _ => {}
        }
        let mut state = State { acc: 0, line: 0 };
        let mut seen = HashSet::new();
        match step_to_termination(&mut state, &mut seen, &changed) {
            Some(state) => return state.acc,
            None => {}
        }
    }
    0
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2, InstructionType};

    static INPUT_RAW: &str = r#"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"#;

    #[test]
    fn generator() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(input[0].instruction, InstructionType::NOP);
        assert_eq!(input[0].value, 0);
        assert_eq!(input[1].instruction, InstructionType::ACC);
        assert_eq!(input[1].value, 1);
        assert_eq!(input[2].instruction, InstructionType::JMP);
        assert_eq!(input[2].value, 4);
        assert_eq!(input[5].instruction, InstructionType::ACC);
        assert_eq!(input[5].value, -99);
    }

    #[test]
    fn day8_part1() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(part1(&input), 5);
    }

    #[test]
    fn day8_part2() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(part2(&input), 8);
    }
}
