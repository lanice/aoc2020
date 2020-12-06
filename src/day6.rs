use std::collections::HashMap;

#[aoc_generator(day6)]
fn generator_input(input: &str) -> Vec<String> {
    input
        .split("\n\n")
        .map(|s| s.replace("\n", ","))
        .collect::<Vec<_>>()
}

#[aoc(day6, part1)]
fn part1(input: &[String]) -> i32 {
    input
        .iter()
        .map(|f| {
            let mut chars = f
                .chars()
                .filter(|p| match p {
                    ',' => false,
                    _ => true,
                })
                .collect::<Vec<_>>();
            chars.sort();
            chars.dedup();
            chars.len() as i32
        })
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &[String]) -> i32 {
    input
        .iter()
        .map(|f| {
            let answers = f.split(',').collect::<Vec<_>>();
            let mut yesses = HashMap::new();
            let mut member_count = 0;
            for answer in answers {
                member_count += 1;
                for choice in answer.chars() {
                    *yesses.entry(choice).or_insert(0) += 1;
                }
            }
            yesses
                .iter()
                .filter(|(_, &count)| count == member_count)
                .count() as i32
        })
        .sum()
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};
    static INPUT_RAW: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    #[test]
    fn generator() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(input, vec!["abc", "a,b,c", "ab,ac", "a,a,a,a", "b"]);
    }

    #[test]
    fn day6_part1() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn day6_part2() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(part2(&input), 6);
    }
}
