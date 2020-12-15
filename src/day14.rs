use std::collections::HashMap;

struct Program {
    bits_0: u64,
    bits_1: u64,
    operations: Vec<(usize, u64)>,
}

impl Program {
    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        let mask = lines.next().unwrap();
        let mut bits_0 = 0;
        let mut bits_1 = 0;
        for (i, char) in mask.chars().enumerate() {
            match char {
                '0' => {
                    bits_0 += 2_u64.pow(35 - i as u32);
                }
                '1' => {
                    bits_1 += 2_u64.pow(35 - i as u32);
                }
                _ => {}
            }
        }

        let mut operations = vec![];
        for line in lines {
            let mut split = line.split(" = ");
            let address = split
                .next()
                .unwrap()
                .split("[")
                .nth(1)
                .unwrap()
                .split("]")
                .nth(0)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            let value = split.next().unwrap().parse::<u64>().unwrap();
            operations.push((address, value));
        }

        Program {
            bits_0,
            bits_1,
            operations,
        }
    }
}

#[aoc_generator(day14)]
fn generator_input(input: &str) -> Vec<Program> {
    input
        .split("mask = ")
        .skip(1)
        .map(|p| Program::from_str(p))
        .collect::<Vec<_>>()
}

fn mask_value(value: u64, bits_0: u64, bits_1: u64) -> u64 {
    let mut masked = value & !bits_0;
    masked |= bits_1;
    masked
}

#[aoc(day14, part1)]
fn part1(input: &[Program]) -> u64 {
    let mut memory: HashMap<usize, u64> = HashMap::new();
    for program in input {
        for (address, value) in program.operations.iter() {
            memory.insert(*address, mask_value(*value, program.bits_0, program.bits_1));
        }
    }
    memory.values().sum::<u64>()
}

#[aoc(day14, part2)]
fn part2(_input: &[Program]) -> i32 {
    // TODO
    0
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT_RAW: &str = r#"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(input[0].bits_0, 2);
        assert_eq!(input[0].bits_1, 64);
        assert_eq!(input[0].operations, vec![(8, 11), (7, 101), (8, 0)]);
    }

    #[test]
    fn day14_part1() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part1(&input), 165);
    }

    #[test]
    fn day14_part2() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part2(&input), 208);
    }
}
