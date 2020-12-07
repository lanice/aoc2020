use std::collections::HashMap;
use std::collections::HashSet;

type MapColorIsContainedIn = HashMap<String, HashSet<String>>;
type MapColorContainsOthers = HashMap<String, HashMap<String, i32>>;

#[aoc_generator(day7)]
fn generator_input(input: &str) -> (MapColorIsContainedIn, MapColorContainsOthers) {
    let mut color_is_contained_in = HashMap::new();
    let mut color_contains_others = HashMap::new();
    for line in input.lines() {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let color = words[..2].join(" ");
        let (is_contained_in, contains_others): (HashMap<String, String>, HashMap<String, i32>) =
            if words
                .clone()
                .into_iter()
                .rev()
                .take(3)
                .rev()
                .collect::<Vec<_>>()
                .join(" ")
                == "no other bags."
            {
                (HashMap::new(), HashMap::new())
            } else {
                (
                    words[4..]
                        .join(" ")
                        .split(",")
                        .map(|f| f.trim().split(" ").collect::<Vec<_>>()[1..3].join(" "))
                        .map(|inside_color| (inside_color, color.clone()))
                        .collect::<HashMap<_, _>>(),
                    words[4..]
                        .join(" ")
                        .split(",")
                        .map(|f| {
                            let inside_color_rule = f.trim().split(" ").collect::<Vec<_>>();
                            let inside_color = inside_color_rule[1..3].join(" ");
                            let inside_count = inside_color_rule[0].parse::<i32>().unwrap();
                            (inside_color, inside_count)
                        })
                        .collect::<HashMap<_, _>>(),
                )
            };
        for (inside_color, outside_color) in is_contained_in.into_iter() {
            color_is_contained_in
                .entry(inside_color)
                .or_insert({
                    let mut set = HashSet::new();
                    set.insert(outside_color.clone());
                    set
                })
                .insert(outside_color);
        }
        color_contains_others.insert(color, contains_others);
    }
    (color_is_contained_in, color_contains_others)
}

fn get_outside_bags(inside_bag: String, map: &MapColorIsContainedIn) -> HashSet<String> {
    let mut all_outside_bags = HashSet::new();
    match map.get(&inside_bag) {
        Some(outside_bags) => {
            for bag in outside_bags {
                all_outside_bags.insert(bag.clone());
                all_outside_bags.extend(get_outside_bags(bag.clone(), map));
            }
            all_outside_bags
        }
        None => HashSet::new(),
    }
}

#[aoc(day7, part1)]
fn part1((input, _): &(MapColorIsContainedIn, MapColorContainsOthers)) -> i32 {
    get_outside_bags("shiny gold".to_string(), &input).len() as i32
}

fn inside_bag_count(outside_bag: String, map: &MapColorContainsOthers) -> i32 {
    match map.get(&outside_bag) {
        Some(inside_bags_with_counts) => inside_bags_with_counts
            .iter()
            .map(|(color, count)| count + count * inside_bag_count(color.clone(), map))
            .sum(),
        None => 0,
    }
}

#[aoc(day7, part2)]
fn part2((_, input): &(MapColorIsContainedIn, MapColorContainsOthers)) -> i32 {
    inside_bag_count("shiny gold".to_string(), input)
}

#[cfg(test)]
pub mod tests {
    use std::collections::{HashMap, HashSet};

    use super::{generator_input, part1, part2};
    static INPUT_RAW: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags."#;
    static INPUT_RAW_SLIM: &str = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
faded blue bags contain no other bags.
bright white bags contain 1 shiny gold bag."#;

    #[test]
    fn generator() {
        let mut expected0 = HashMap::new();
        expected0.insert(
            "bright white".to_string(),
            ["light red", "dark orange"]
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
        );
        expected0.insert(
            "muted yellow".to_string(),
            ["light red", "dark orange"]
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
        );
        expected0.insert(
            "shiny gold".to_string(),
            ["bright white"]
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
        );
        let mut expected1 = HashMap::new();
        expected1.insert(
            "light red".to_string(),
            [("bright white", 1), ("muted yellow", 2)]
                .iter()
                .map(|(color, count)| (color.to_string(), count.clone()))
                .collect::<HashMap<String, i32>>(),
        );
        expected1.insert(
            "dark orange".to_string(),
            [("bright white", 3), ("muted yellow", 4)]
                .iter()
                .map(|(color, count)| (color.to_string(), count.clone()))
                .collect::<HashMap<String, i32>>(),
        );
        expected1.insert("faded blue".to_string(), HashMap::new());
        expected1.insert(
            "bright white".to_string(),
            [("shiny gold", 1)]
                .iter()
                .map(|(color, count)| (color.to_string(), count.clone()))
                .collect::<HashMap<String, i32>>(),
        );
        let input = generator_input(&INPUT_RAW_SLIM);
        assert_eq!(input.0, expected0);
        assert_eq!(input.1, expected1);
    }

    #[test]
    fn day7_part1() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(part1(&input), 4);
    }

    #[test]
    fn day7_part2() {
        let input = generator_input(&INPUT_RAW);
        assert_eq!(part2(&input), 32);
    }
}
