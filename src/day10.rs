use std::collections::HashMap;

#[aoc_generator(day10)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

#[aoc(day10, part1)]
fn part1(input: &[i32]) -> i32 {
    let mut input = input.to_vec();
    input.sort();
    let mut last_joltage = 0;
    let mut diff_1_count = 0;
    let mut diff_3_count = 1; // That last difference will always be three, as it is to our device
    for joltage in input.iter() {
        match joltage - last_joltage {
            3 => {
                diff_3_count += 1;
            }
            1 => {
                diff_1_count += 1;
            }
            _ => {}
        }
        last_joltage = *joltage;
    }
    diff_1_count * diff_3_count
}

fn calc_number_of_arrangements_if_possible(
    input: &[i32],
    i: usize,
    j: usize,
    memo: &mut HashMap<usize, i32>,
) -> i32 {
    if input[j] - input[i] <= 3 {
        return calc_number_of_arrangements(&input, j, memo);
    }
    0
}

fn calc_number_of_arrangements(input: &[i32], i: usize, memo: &mut HashMap<usize, i32>) -> i32 {
    if memo.contains_key(&i) {
        return memo[&i];
    }

    let count = match input.len() - i {
        0 => 0,
        1 => 1,
        2 => calc_number_of_arrangements_if_possible(input, i, i + 1, memo),
        3 => {
            calc_number_of_arrangements_if_possible(input, i, i + 1, memo)
                + calc_number_of_arrangements_if_possible(input, i, i + 2, memo)
        }
        _ => {
            calc_number_of_arrangements_if_possible(input, i, i + 1, memo)
                + calc_number_of_arrangements_if_possible(input, i, i + 2, memo)
                + calc_number_of_arrangements_if_possible(input, i, i + 3, memo)
        }
    };

    memo.insert(i, count);
    count
}

#[aoc(day10, part2)]
fn part2(input: &[i32]) -> i32 {
    let mut input = input.to_vec();
    input.push(0);
    input.sort();
    input.push(input.last().unwrap() + 3);
    calc_number_of_arrangements(&input, 0, &mut HashMap::<usize, i32>::new())
}

#[cfg(test)]
pub mod tests {
    use super::{part1, part2};

    #[test]
    fn day10_part1_small() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(part1(&input), 35);
    }

    #[test]
    fn day10_part1_large() {
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(part1(&input), 220);
    }

    #[test]
    fn day10_part2_small() {
        let input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        assert_eq!(part2(&input), 8);
    }

    #[test]
    fn day10_part2_large() {
        let input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        assert_eq!(part2(&input), 19208);
    }
}
