use core::panic;

#[derive(Debug, PartialEq)]
enum Action {
    NORTH,
    EAST,
    SOUTH,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

impl Action {
    fn new(char: char) -> Self {
        match char {
            'N' => Action::NORTH,
            'E' => Action::EAST,
            'S' => Action::SOUTH,
            'W' => Action::WEST,
            'L' => Action::LEFT,
            'R' => Action::RIGHT,
            'F' => Action::FORWARD,
            _ => panic!("Impossible!"),
        }
    }
}

struct Instruction {
    action: Action,
    value: i32,
}

impl Instruction {
    fn new(s: &str) -> Self {
        let (action, value) = s.split_at(1);
        Instruction {
            action: Action::new(action.chars().next().unwrap()),
            value: value.parse::<i32>().unwrap(),
        }
    }
}

#[aoc_generator(day12)]
fn generator_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|i| Instruction::new(i))
        .collect::<Vec<_>>()
}

fn get_rotation_count(degree: i32) -> i32 {
    match degree {
        0 => 0,
        90 => 1,
        180 => 2,
        270 => 3,
        _ => panic!("Invalid rotation!"),
    }
}

#[aoc(day12, part1)]
fn part1(input: &Vec<Instruction>) -> i32 {
    let (mut x, mut y) = (0, 0);
    let mut direction = 0;
    for instruction in input {
        match instruction.action {
            Action::NORTH => {
                y += instruction.value;
            }
            Action::EAST => {
                x += instruction.value;
            }
            Action::SOUTH => {
                y -= instruction.value;
            }
            Action::WEST => {
                x -= instruction.value;
            }
            Action::LEFT => {
                direction = (direction + 3 * get_rotation_count(instruction.value)) % 4;
            }
            Action::RIGHT => {
                direction = (direction + get_rotation_count(instruction.value)) % 4;
            }
            Action::FORWARD => match direction {
                0 => {
                    x += instruction.value;
                }
                1 => {
                    y -= instruction.value;
                }
                2 => {
                    x -= instruction.value;
                }
                3 => {
                    y += instruction.value;
                }
                _ => panic!("Invalid direction!"),
            },
        }
    }
    x.abs() + y.abs()
}

#[aoc(day12, part2)]
fn part2(input: &Vec<Instruction>) -> i32 {
    let (mut ship_x, mut ship_y, mut way_x, mut way_y) = (0, 0, 10, 1);
    for instruction in input {
        match instruction.action {
            Action::NORTH => {
                way_y += instruction.value;
            }
            Action::EAST => {
                way_x += instruction.value;
            }
            Action::SOUTH => {
                way_y -= instruction.value;
            }
            Action::WEST => {
                way_x -= instruction.value;
            }
            Action::LEFT => match get_rotation_count(instruction.value) {
                0 => {}
                1 => {
                    let (new_x, new_y) = (-way_y, way_x);
                    way_x = new_x;
                    way_y = new_y;
                }
                2 => {
                    let (new_x, new_y) = (-way_x, -way_y);
                    way_x = new_x;
                    way_y = new_y;
                }
                3 => {
                    let (new_x, new_y) = (way_y, -way_x);
                    way_x = new_x;
                    way_y = new_y;
                }
                _ => panic!("Invalid rotation!"),
            },
            Action::RIGHT => match get_rotation_count(instruction.value) {
                0 => {}
                1 => {
                    let (new_x, new_y) = (way_y, -way_x);
                    way_x = new_x;
                    way_y = new_y;
                }
                2 => {
                    let (new_x, new_y) = (-way_x, -way_y);
                    way_x = new_x;
                    way_y = new_y;
                }
                3 => {
                    let (new_x, new_y) = (-way_y, way_x);
                    way_x = new_x;
                    way_y = new_y;
                }
                _ => panic!("Invalid rotation!"),
            },
            Action::FORWARD => {
                ship_x += way_x * instruction.value;
                ship_y += way_y * instruction.value;
            }
        }
    }
    ship_x.abs() + ship_y.abs()
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2, Action};

    static INPUT_RAW: &str = r#"F10
N3
F7
R90
F11"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(input[0].action, Action::FORWARD);
        assert_eq!(input[0].value, 10);
        assert_eq!(input[3].action, Action::RIGHT);
        assert_eq!(input[3].value, 90);
    }

    #[test]
    fn day11_part1() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part1(&input), 25);
    }

    #[test]
    fn day11_part2() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part2(&input), 286);
    }
}
