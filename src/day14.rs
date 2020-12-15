use core::panic;
use std::{collections::HashMap, vec};

struct Program {
    bits_0: u64,
    bits_1: u64,
    bits_x: u64,
    operations: Vec<(u64, u64)>,
}

impl Program {
    fn from_str(input: &str) -> Self {
        let mut lines = input.lines();
        let mask = lines.next().unwrap();
        let mut bits_0 = 0;
        let mut bits_1 = 0;
        let mut bits_x = 0;
        for (i, char) in mask.chars().enumerate() {
            match char {
                '0' => {
                    bits_0 += 2_u64.pow(35 - i as u32);
                }
                '1' => {
                    bits_1 += 2_u64.pow(35 - i as u32);
                }
                'X' => {
                    bits_x += 2_u64.pow(35 - i as u32);
                }
                _ => {
                    panic!("Impossibruh!");
                }
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
                .parse::<u64>()
                .unwrap();
            let value = split.next().unwrap().parse::<u64>().unwrap();
            operations.push((address, value));
        }

        Program {
            bits_0,
            bits_1,
            bits_x,
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
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for program in input {
        for (address, value) in program.operations.iter() {
            memory.insert(*address, mask_value(*value, program.bits_0, program.bits_1));
        }
    }
    memory.values().sum::<u64>()
}

fn get_bit_at(input: u64, n: u8) -> Result<bool, ()> {
    if n < 64 {
        Ok(input & (1 << n) != 0)
    } else {
        Err(())
    }
}

fn mutate_masks(input: Vec<u64>, bitmask: u64) -> Vec<u64> {
    let mut masks = vec![];
    for mask in input {
        // Insert bitmask that sets the floating bit
        masks.push(mask | bitmask);
        // Insert bitmask that unsets the floating bit
        masks.push(mask & !bitmask);
    }
    masks
}

fn generate_bitmasks(input: u64, bits_x: u64) -> Vec<u64> {
    let mut masks = vec![];
    masks.push(input);
    for i in (0..64).rev() {
        if get_bit_at(bits_x, i).unwrap() {
            masks = mutate_masks(masks, 2_u64.pow(i as u32));
        }
    }
    masks
}

fn mask_address(address: u64, bits_1: u64, bits_x: u64) -> Vec<u64> {
    let masked = address | bits_1;
    generate_bitmasks(masked, bits_x)
}

#[aoc(day14, part2)]
fn part2(input: &[Program]) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for program in input {
        for (address, value) in program.operations.iter() {
            let masked_addresses = mask_address(*address, program.bits_1, program.bits_x);
            for masked_address in masked_addresses {
                memory.insert(masked_address, *value);
            }
        }
    }
    memory.values().sum::<u64>()
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, get_bit_at, part1, part2};

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
        let input_raw = r#"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"#;
        let input = generator_input(input_raw);
        assert_eq!(part2(&input), 208);
    }

    #[test]
    fn test_get_bit_at() {
        let n = 69_u64;
        assert!(get_bit_at(n, 0).unwrap());
        assert!(!get_bit_at(n, 1).unwrap());
        assert!(get_bit_at(n, 2).unwrap());
        assert!(!get_bit_at(n, 3).unwrap());
        assert!(!get_bit_at(n, 4).unwrap());
        assert!(!get_bit_at(n, 5).unwrap());
        assert!(get_bit_at(n, 6).unwrap());
    }
}
