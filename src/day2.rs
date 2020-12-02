#[derive(Debug)]
struct PasswordsWithPolicy {
    min: i32,
    max: i32,
    char: char,
    pw: String,
}

impl std::cmp::PartialEq for PasswordsWithPolicy {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min
            && self.max == other.max
            && self.char == other.char
            && self.pw == other.pw
    }
}

fn parse_pw_line(input: &str) -> PasswordsWithPolicy {
    let parts = input.split(' ').collect::<Vec<&str>>();
    let minmax = parts[0].split('-').collect::<Vec<&str>>();
    let min = minmax[0].parse::<i32>().unwrap();
    let max = minmax[1].parse::<i32>().unwrap();
    let char = parts[1].chars().next().unwrap();
    let pw = parts[2].to_string();
    PasswordsWithPolicy { min, max, char, pw }
}

/// Parses each line to be an i32
#[aoc_generator(day2)]
fn generator_input(input: &str) -> Vec<PasswordsWithPolicy> {
    input
        .lines()
        .map(|a| parse_pw_line(a))
        .collect::<Vec<PasswordsWithPolicy>>()
}

#[aoc(day2, part1)]
fn part1(input: &[PasswordsWithPolicy]) -> i32 {
    let mut valid_pw_count = 0;
    for password in input {
        let mut char_count = 0;
        for char in password.pw.chars() {
            if char == password.char {
                char_count += 1
            }
        }
        if char_count >= password.min && char_count <= password.max {
            valid_pw_count += 1;
        }
    }
    return valid_pw_count;
}

#[aoc(day2, part2)]
fn part2(input: &[PasswordsWithPolicy]) -> i32 {
    let mut valid_pw_count = 0;
    for password in input {
        let mut occurrences = 0;
        for pos in &[password.min, password.max] {
            if password.pw.chars().nth((*pos - 1) as usize).unwrap() == password.char {
                occurrences += 1
            }
        }
        if occurrences == 1 {
            valid_pw_count += 1
        }
    }
    return valid_pw_count;
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2, PasswordsWithPolicy};

    #[test]
    fn generator() {
        let input_raw = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let input = generator_input(&input_raw);
        let first_line = PasswordsWithPolicy {
            min: 1,
            max: 3,
            char: 'a',
            pw: "abcde".to_string(),
        };
        assert_eq!(input[0], first_line);
    }

    #[test]
    fn day2_part1() {
        let input_raw = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let input = generator_input(&input_raw);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn day2_part2() {
        let input_raw = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        let input = generator_input(&input_raw);
        assert_eq!(part2(&input), 1);
    }
}
