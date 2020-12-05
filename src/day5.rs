use core::panic;

static NUM_ROWS: i32 = 128;
static NUM_COLS: i32 = 8;

#[aoc_generator(day5)]
fn generator_input(input: &str) -> Vec<String> {
    input.lines().map(|a| a.to_string()).collect()
}

fn calculate_row(row_info: &[char]) -> i32 {
    search_binary(row_info, (0, NUM_ROWS - 1))
}

fn calculate_col(col_info: &[char]) -> i32 {
    search_binary(col_info, (0, NUM_COLS - 1))
}

fn calculate_seat_id(boarding_pass: &str) -> i32 {
    let row = calculate_row(&boarding_pass[..7].chars().collect::<Vec<_>>());
    let col = calculate_col(&boarding_pass[7..].chars().collect::<Vec<_>>());
    row * 8 + col
}

fn search_binary(partitioning: &[char], seats: (i32, i32)) -> i32 {
    if partitioning.len() == 0 {
        assert_eq!(seats.0, seats.1);
        return seats.0;
    }
    let half = seats.0 + (seats.1 - seats.0) / 2;
    match partitioning.first().unwrap() {
        'F' | 'L' => search_binary(&partitioning[1..], (seats.0, half)),
        'B' | 'R' => search_binary(&partitioning[1..], (half + 1, seats.1)),
        _ => panic!("This can't happen :O Abort mission, prepare for emergency landing!"),
    }
}

#[aoc(day5, part1)]
fn part1(input: &[String]) -> i32 {
    input
        .iter()
        .map(|pass| calculate_seat_id(pass))
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
fn part2(input: &[String]) -> i32 {
    let mut ids = input
        .iter()
        .map(|pass| calculate_seat_id(pass))
        .collect::<Vec<_>>();
    ids.sort();
    let mut my_id = ids[1];
    for id in ids[1..].iter() {
        if &my_id != id {
            return my_id;
        }
        my_id += 1;
    }
    panic!("This can't happen :O Abort mission, prepare for emergency landing!");
}

#[cfg(test)]
pub mod tests {
    use super::{calculate_col, calculate_row, calculate_seat_id, generator_input, part1};
    static INPUT_RAW: &str = r#"BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL"#;

    #[test]
    fn generator() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(input, vec!["BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"]);
    }

    #[test]
    fn test_calculate_row() {
        let row_info = "FBFBBFF";
        let chars = row_info.chars().collect::<Vec<_>>();
        assert_eq!(calculate_row(&chars), 44);
    }

    #[test]
    fn test_calculate_col() {
        let col_info = "RLR";
        let chars = col_info.chars().collect::<Vec<_>>();
        assert_eq!(calculate_col(&chars), 5);
    }

    #[test]
    fn test_calculate_seat_id() {
        let boarding_pass = "FBFBBFFRLR";
        assert_eq!(calculate_seat_id(boarding_pass), 357);
    }

    #[test]
    fn day5_part1() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(part1(&input), 820);
    }
}
