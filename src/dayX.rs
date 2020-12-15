#[aoc_generator(dayX)]
fn generator_input(input: &str) -> Vec<i32> {
    input.lines().map(|a| a.parse::<i32>().unwrap()).collect()
}

#[aoc(dayX, part1)]
fn part1(input: &[i32]) -> i32 {
    0
}

#[aoc(dayX, part2)]
fn part2(input: &[i32]) -> i32 {
    0
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT_RAW: &str = r#""#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(input, vec![0]);
    }

    #[test]
    fn dayX_part1() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part1(&input), 0);
    }

    #[test]
    fn dayX_part2() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part2(&input), 0);
    }
}
