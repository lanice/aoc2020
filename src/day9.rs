#[aoc_generator(day9)]
fn generator_input(input: &str) -> Vec<usize> {
    input.lines().map(|a| a.parse::<usize>().unwrap()).collect()
}

fn number_is_weak(number: usize, preamble: &[usize]) -> bool {
    for i in 0..preamble.len() - 1 {
        if preamble[i] <= number {
            for j in i + 1..preamble.len() {
                if preamble[i] + preamble[j] == number {
                    return false;
                }
            }
        }
    }
    true
}

fn find_weakness(input: &[usize], preamble_length: usize) -> Option<usize> {
    for i in preamble_length..input.len() {
        if number_is_weak(input[i], &input[i - preamble_length..i]) {
            return Some(input[i]);
        }
    }
    None
}

#[aoc(day9, part1)]
fn part1(input: &[usize]) -> usize {
    find_weakness(input, 25).unwrap()
}

fn find_contiguous_sum(input: &[usize], target: usize) -> usize {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if input[i..j].iter().sum::<usize>() == target {
                return input[i..j].iter().max().unwrap() + input[i..j].iter().min().unwrap();
            }
        }
    }
    return 0;
}

#[aoc(day9, part2)]
fn part2(input: &[usize]) -> usize {
    find_contiguous_sum(input, find_weakness(input, 25).unwrap())
}

#[cfg(test)]
pub mod tests {
    use super::{find_contiguous_sum, find_weakness};

    #[test]
    fn test_find_weakness() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(find_weakness(&input, 5).unwrap(), 127);
    }

    #[test]
    fn test_find_contiguous_sum() {
        let input = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(
            find_contiguous_sum(&input, find_weakness(&input, 5).unwrap()),
            62
        );
    }
}
