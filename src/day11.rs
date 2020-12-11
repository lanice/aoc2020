use core::panic;

#[derive(PartialEq, Clone, Debug)]
enum Status {
    EMPTY,
    OCCUPIED,
    FLOOR,
}

fn parse_status(char: &char) -> Status {
    match char {
        'L' => Status::EMPTY,
        '#' => Status::OCCUPIED,
        '.' => Status::FLOOR,
        _ => panic!("Wait, that's illegal!"),
    }
}

#[aoc_generator(day11)]
fn generator_input(input: &str) -> Vec<Vec<Status>> {
    input
        .lines()
        .map(|a| a.chars().map(|c| parse_status(&c)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn apply_rule_part1(grid: &Vec<Vec<Status>>, row: usize, col: usize) -> Status {
    if grid[row][col] == Status::FLOOR {
        return Status::FLOOR;
    }

    let mut neighbor_count = 0;

    let sub_row = match row {
        0 => 0,
        _ => 1,
    };

    let sub_col = match col {
        0 => 0,
        _ => 1,
    };

    for i in row - sub_row..=row + 1 {
        if i >= grid.len() {
            continue;
        }
        for j in col - sub_col..=col + 1 {
            if j >= grid[i].len() || i == row && j == col {
                continue;
            }
            neighbor_count += match grid[i][j] {
                Status::OCCUPIED => 1,
                _ => 0,
            }
        }
    }

    return match (grid[row][col].clone(), neighbor_count) {
        (Status::EMPTY, 0) => Status::OCCUPIED,
        (Status::OCCUPIED, 4) => Status::EMPTY,
        (Status::OCCUPIED, 5) => Status::EMPTY,
        (Status::OCCUPIED, 6) => Status::EMPTY,
        (Status::OCCUPIED, 7) => Status::EMPTY,
        (Status::OCCUPIED, 8) => Status::EMPTY,
        (s, _) => s,
    };
}

fn apply_rule_part2(grid: &Vec<Vec<Status>>, row: usize, col: usize) -> Status {
    if grid[row][col] == Status::FLOOR {
        return Status::FLOOR;
    }
    let length = grid[0].len();

    let mut neighbor_count = 0;

    // Check North
    let (mut i, j) = (row, col);
    while i > 0 {
        i -= 1;
        match grid[i][j] {
            Status::OCCUPIED => {
                neighbor_count += 1;
                break;
            }
            Status::EMPTY => {
                break;
            }
            Status::FLOOR => {}
        }
    }

    // Check North-East
    let (mut i, mut j) = (row, col);
    while i > 0 && j < length - 1 {
        i -= 1;
        j += 1;
        match grid[i][j] {
            Status::OCCUPIED => {
                neighbor_count += 1;
                break;
            }
            Status::EMPTY => {
                break;
            }
            Status::FLOOR => {}
        }
    }

    // Check East
    let (i, mut j) = (row, col);
    while j < length - 1 {
        j += 1;
        match grid[i][j] {
            Status::OCCUPIED => {
                neighbor_count += 1;
                break;
            }
            Status::EMPTY => {
                break;
            }
            Status::FLOOR => {}
        }
    }

    // Check South-East
    let (mut i, mut j) = (row, col);
    while i < grid.len() - 1 && j < length - 1 {
        i += 1;
        j += 1;
        match grid[i][j] {
            Status::OCCUPIED => {
                neighbor_count += 1;
                break;
            }
            Status::EMPTY => {
                break;
            }
            Status::FLOOR => {}
        }
    }

    // Check South
    let (mut i, j) = (row, col);
    while i < grid.len() - 1 {
        i += 1;
        match grid[i][j] {
            Status::OCCUPIED => {
                neighbor_count += 1;
                break;
            }
            Status::EMPTY => {
                break;
            }
            Status::FLOOR => {}
        }
    }

    // Check South-West
    let (mut i, mut j) = (row, col);
    while i < grid.len() - 1 && j > 0 {
        i += 1;
        j -= 1;
        match grid[i][j] {
            Status::OCCUPIED => {
                neighbor_count += 1;
                break;
            }
            Status::EMPTY => {
                break;
            }
            Status::FLOOR => {}
        }
    }

    // Check West
    let (i, mut j) = (row, col);
    while j > 0 {
        j -= 1;
        match grid[i][j] {
            Status::OCCUPIED => {
                neighbor_count += 1;
                break;
            }
            Status::EMPTY => {
                break;
            }
            Status::FLOOR => {}
        }
    }

    // Check North-West
    let (mut i, mut j) = (row, col);
    while i > 0 && j > 0 {
        i -= 1;
        j -= 1;
        match grid[i][j] {
            Status::OCCUPIED => {
                neighbor_count += 1;
                break;
            }
            Status::EMPTY => {
                break;
            }
            Status::FLOOR => {}
        }
    }

    return match (grid[row][col].clone(), neighbor_count) {
        (Status::EMPTY, 0) => Status::OCCUPIED,
        (Status::OCCUPIED, 5) => Status::EMPTY,
        (Status::OCCUPIED, 6) => Status::EMPTY,
        (Status::OCCUPIED, 7) => Status::EMPTY,
        (Status::OCCUPIED, 8) => Status::EMPTY,
        (s, _) => s,
    };
}

fn apply_rules(grid: &Vec<Vec<Status>>, is_part1: bool) -> Vec<Vec<Status>> {
    let mut new_grid: Vec<Vec<Status>> = vec![];

    for i in 0..grid.len() {
        let mut row: Vec<Status> = vec![];
        for j in 0..grid[i].len() {
            if is_part1 {
                row.push(apply_rule_part1(&grid, i, j));
            } else {
                row.push(apply_rule_part2(&grid, i, j));
            }
        }
        new_grid.push(row);
    }

    new_grid
}

fn count_occupied_seats(grid: &Vec<Vec<Status>>) -> i32 {
    grid.iter()
        .map(|r| {
            r.iter()
                .map(|f| match f {
                    Status::OCCUPIED => 1,
                    _ => 0,
                })
                .sum::<i32>()
        })
        .sum::<i32>()
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Vec<Status>>) -> i32 {
    let mut old_grid = input.to_owned();

    loop {
        let new_grid = apply_rules(&old_grid, true);
        if new_grid == *old_grid {
            return count_occupied_seats(&new_grid);
        }
        old_grid = new_grid;
    }
}

#[aoc(day11, part2)]
fn part2(input: &Vec<Vec<Status>>) -> i32 {
    let mut old_grid = input.to_owned();

    loop {
        let new_grid = apply_rules(&old_grid, false);
        if new_grid == *old_grid {
            return count_occupied_seats(&new_grid);
        }
        old_grid = new_grid;
    }
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2, Status};

    static INPUT_RAW: &str = r#"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(input[0][0], Status::EMPTY);
        assert_eq!(input[0][1], Status::FLOOR);
        assert_eq!(input[1][0], Status::EMPTY);
        assert_eq!(input[1][1], Status::EMPTY);
        assert_eq!(input[2][1], Status::FLOOR);
    }

    #[test]
    fn day11_part1() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part1(&input), 37);
    }

    #[test]
    fn day11_part2() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part2(&input), 26);
    }
}
