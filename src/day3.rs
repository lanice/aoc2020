#[derive(Debug, PartialEq)]
enum TobogganField {
    Open,
    Tree,
}

impl TobogganField {
    fn new(char: char) -> Self {
        match char {
            '.' => TobogganField::Open,
            '#' => TobogganField::Tree,
            _ => panic!("This can't be! :O"),
        }
    }
}

struct TobogganMap {
    width: usize,
    height: usize,
    fields: Vec<TobogganField>,
}

#[aoc_generator(day3)]
fn generator_input(input: &str) -> TobogganMap {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let fields = input
        .lines()
        .map(|a| a.chars())
        .flat_map(|c| c.map(|f| TobogganField::new(f)))
        .collect::<Vec<TobogganField>>();
    TobogganMap {
        width,
        height,
        fields,
    }
}

fn count_trees_in_slope(map: &TobogganMap, step_size_x: usize, step_size_y: usize) -> i32 {
    let mut x = 0;
    let mut tree_count = 0;
    for y in (0..map.height).step_by(step_size_y) {
        if map.fields[y * map.width + x] == TobogganField::Tree {
            tree_count += 1;
        }
        x = (x + step_size_x) % map.width;
    }
    return tree_count;
}

#[aoc(day3, part1)]
fn part1(input: &TobogganMap) -> i32 {
    count_trees_in_slope(input, 3, 1)
}

#[aoc(day3, part2)]
fn part2(input: &TobogganMap) -> i32 {
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|s| count_trees_in_slope(input, s.0, s.1))
        .product()
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2, TobogganField};
    static INPUT_RAW: &str = "..##.......\n#...#...#..\n.#....#..#.\n..#.#...#.#\n.#...##..#.\n..#.##.....\n.#.#.#....#\n.#........#\n#.##...#...\n#...##....#\n.#..#...#.#\n";

    #[test]
    fn generator() {
        let map = generator_input(&INPUT_RAW);
        assert_eq!(map.width, 11);
        assert_eq!(map.height, 11);
        assert_eq!(map.fields[0], TobogganField::Open);
        assert_eq!(map.fields[1], TobogganField::Open);
        assert_eq!(map.fields[2], TobogganField::Tree);
        assert_eq!(map.fields[3], TobogganField::Tree);
        assert_eq!(map.fields[4], TobogganField::Open);
        assert_eq!(map.fields[11], TobogganField::Tree);
        assert_eq!(map.fields[12], TobogganField::Open);
    }

    #[test]
    fn day3_part1() {
        let map = generator_input(&INPUT_RAW);
        assert_eq!(part1(&map), 7);
    }

    #[test]
    fn day3_part2() {
        let map = generator_input(&INPUT_RAW);
        assert_eq!(part2(&map), 336);
    }
}
