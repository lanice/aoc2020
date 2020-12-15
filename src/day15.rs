use std::{collections::HashMap, convert::TryInto};

#[aoc_generator(day15)]
fn generator_input(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn determine_nth_spoken_number(input: &[i32], n: usize) -> i32 {
    let mut memory = HashMap::new();
    let mut i = 0;

    while i < input.len() - 1 {
        memory.insert(input[i], i);
        i += 1;
    }

    let mut last_spoken_number = input.last().unwrap().to_owned();

    while i < (n - 1) {
        let new_last_spoken_number: i32 = match memory.get(&last_spoken_number) {
            Some(turn) => (i - *turn).try_into().unwrap(),
            None => 0,
        };
        memory.insert(last_spoken_number, i);
        last_spoken_number = new_last_spoken_number;
        i += 1;
    }

    last_spoken_number
}

#[aoc(day15, part1)]
fn part1(input: &[i32]) -> i32 {
    determine_nth_spoken_number(input, 2020)
}

#[aoc(day15, part2)]
fn part2(input: &[i32]) -> i32 {
    determine_nth_spoken_number(input, 30000000)
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT_RAW: &str = r#"0,3,6"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(input, vec![0, 3, 6]);
    }

    #[test]
    fn day15_part1() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part1(&input), 436);
    }

    #[test]
    fn day15_part2() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part2(&input), 175594);
    }
}
