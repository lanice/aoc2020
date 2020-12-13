#[aoc_generator(day13)]
fn generator_input(input: &str) -> (u64, Vec<(usize, u64)>) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse::<u64>().unwrap();
    let ids = lines
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, s)| s.parse::<u64>().is_ok())
        .map(|(i, s)| (i, s.parse::<u64>().unwrap()))
        .collect::<Vec<_>>();
    (timestamp, ids)
}

#[aoc(day13, part1)]
fn part1((timestamp, ids): &(u64, Vec<(usize, u64)>)) -> u64 {
    let (id, wait_time): (u64, u64) = *ids
        .iter()
        .map(|(_, id)| (*id, id - timestamp % id))
        .collect::<Vec<(u64, u64)>>()
        .iter()
        .min_by_key(|t| t.1)
        .unwrap();
    id * wait_time
}

#[aoc(day13, part2)]
fn part2((_, ids): &(u64, Vec<(usize, u64)>)) -> u64 {
    let (mut time, mut multiplier) = (1, 1);
    for (delay, id) in ids {
        while (time + *delay as u64) % id != 0 {
            time += multiplier;
        }
        multiplier *= id;
    }
    time
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT_RAW: &str = r#"939
7,13,x,x,59,x,31,19"#;

    #[test]
    fn generator() {
        let (timestamp, ids) = generator_input(INPUT_RAW);
        assert_eq!(timestamp, 939);
        assert_eq!(ids, vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)]);
    }

    #[test]
    fn day11_part1() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part1(&input), 295);
    }

    #[test]
    fn day11_part2() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part2(&input), 1068781);
    }
}
