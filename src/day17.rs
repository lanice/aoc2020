use core::panic;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone, Copy)]
enum CubeState {
    ACTIVE,
    INACTIVE,
}

type GameState = HashMap<(i32, i32, i32, i32), CubeState>;

#[aoc_generator(day17)]
fn generator_input(input: &str) -> GameState {
    let mut state = GameState::new();
    let lines = input.lines().collect::<Vec<_>>();
    let y_len = lines.len();
    for y in 0..y_len {
        let cubes = lines[y].chars().collect::<Vec<_>>();
        let x_len = cubes.len();
        for x in 0..x_len {
            state.insert(
                (
                    x as i32 - (x_len / 2) as i32,
                    y as i32 - (y_len / 2) as i32,
                    0,
                    0,
                ),
                match cubes[x] {
                    '.' => CubeState::INACTIVE,
                    '#' => CubeState::ACTIVE,
                    _ => panic!("Invalid cube state char!"),
                },
            );
        }
    }
    state
}

fn get_neighbor_coords(
    curr_x: i32,
    curr_y: i32,
    curr_z: i32,
    curr_w: i32,
    ignore_w: bool,
) -> Vec<(i32, i32, i32, i32)> {
    let mut neighbors = vec![];

    for x in curr_x - 1..=curr_x + 1 {
        for y in curr_y - 1..=curr_y + 1 {
            for z in curr_z - 1..=curr_z + 1 {
                if ignore_w {
                    if x == curr_x && y == curr_y && z == curr_z {
                        continue;
                    }
                    neighbors.push((x, y, z, 0));
                    continue;
                }
                for w in curr_w - 1..=curr_w + 1 {
                    if x == curr_x && y == curr_y && z == curr_z && w == curr_w {
                        continue;
                    }
                    neighbors.push((x, y, z, w));
                }
            }
        }
    }

    neighbors
}

fn get_neighbor_states(
    state: &GameState,
    curr_x: i32,
    curr_y: i32,
    curr_z: i32,
    curr_w: i32,
    ignore_w: bool,
) -> Vec<CubeState> {
    let mut neighbors = vec![];

    for (x, y, z, w) in get_neighbor_coords(curr_x, curr_y, curr_z, curr_w, ignore_w) {
        neighbors.push(match state.get(&(x, y, z, w)) {
            Some(&cube_state) => cube_state,
            None => CubeState::INACTIVE,
        });
    }

    neighbors
}

fn simulate_cycle(state: &GameState, ignore_w: bool) -> GameState {
    let mut new_state = GameState::new();
    let mut new_coords = HashSet::<(i32, i32, i32, i32)>::new();
    for (x, y, z, w) in state.keys() {
        for coords in get_neighbor_coords(*x, *y, *z, *w, ignore_w) {
            new_coords.insert(coords);
        }
    }
    for (x, y, z, w) in new_coords {
        let old_cube_state = match state.get(&(x, y, z, w)) {
            Some(&cube_state) => cube_state,
            None => CubeState::INACTIVE,
        };
        let active_neighbor_count = get_neighbor_states(state, x, y, z, w, ignore_w)
            .iter()
            .filter(|&&s| s == CubeState::ACTIVE)
            .count();
        new_state.insert(
            (x, y, z, w),
            match (old_cube_state, active_neighbor_count) {
                (CubeState::ACTIVE, 2) => CubeState::ACTIVE,
                (CubeState::ACTIVE, 3) => CubeState::ACTIVE,
                (CubeState::INACTIVE, 3) => CubeState::ACTIVE,
                _ => CubeState::INACTIVE,
            },
        );
    }
    new_state
}

#[aoc(day17, part1)]
fn part1(input: &GameState) -> usize {
    let mut input = simulate_cycle(&input, true);
    for _ in 0..5 {
        input = simulate_cycle(&input, true);
    }
    input.values().filter(|&&s| s == CubeState::ACTIVE).count()
}

#[aoc(day17, part2)]
fn part2(input: &GameState) -> usize {
    let mut input = simulate_cycle(&input, false);
    for _ in 0..5 {
        input = simulate_cycle(&input, false);
    }
    input.values().filter(|&&s| s == CubeState::ACTIVE).count()
}

#[cfg(test)]
pub mod tests {
    use std::vec;

    use super::{generator_input, get_neighbor_coords, part1, part2, CubeState};

    static INPUT_RAW: &str = r#".#.
..#
###"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(input[&(-1, -1, 0, 0)], CubeState::INACTIVE);
        assert_eq!(input[&(-1, 0, 0, 0)], CubeState::INACTIVE);
        assert_eq!(input[&(-1, 1, 0, 0)], CubeState::ACTIVE);
        assert_eq!(input[&(0, -1, 0, 0)], CubeState::ACTIVE);
        assert_eq!(input[&(0, 0, 0, 0)], CubeState::INACTIVE);
        assert_eq!(input[&(0, 1, 0, 0)], CubeState::ACTIVE);
        assert_eq!(input[&(1, -1, 0, 0)], CubeState::INACTIVE);
        assert_eq!(input[&(1, 0, 0, 0)], CubeState::ACTIVE);
        assert_eq!(input[&(1, 1, 0, 0)], CubeState::ACTIVE);
    }

    #[test]
    fn test_get_neighbor_coords() {
        assert_eq!(
            get_neighbor_coords(0, 0, 0, 0, false)[0..9],
            vec![
                (-1, -1, -1, -1),
                (-1, -1, -1, 0),
                (-1, -1, -1, 1),
                (-1, -1, 0, -1),
                (-1, -1, 0, 0),
                (-1, -1, 0, 1),
                (-1, -1, 1, -1),
                (-1, -1, 1, 0),
                (-1, -1, 1, 1),
            ]
        );
    }

    #[test]
    fn day17_part1() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part1(&input), 112);
    }

    #[test]
    fn day17_part2() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part2(&input), 848);
    }
}
