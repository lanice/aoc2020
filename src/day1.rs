/// Parses each line to be an i32
#[aoc_generator(day1)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i32]) -> i32 {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            if input[i] + input[j] == 2020 {
                return input[i] * input[j];
            }
        }
    }
    return 0;
}

#[aoc(day1, part2)]
fn part2(input: &[i32]) -> i32 {
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            for k in (j + 1)..input.len() {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    return 0;
}

#[cfg(test)]
pub mod tests {
    use super::{part1, part2};

    #[test]
    fn day1_part1() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&input), 514579);
    }

    #[test]
    fn day1_part2() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(&input), 241861950);
    }
}
