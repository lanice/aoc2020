use std::{
    collections::{HashMap, HashSet},
    vec,
};

#[derive(Debug, PartialEq, Eq)]
struct Rule {
    key: String,
    valids: Vec<i32>,
}

impl Rule {
    #[allow(dead_code)]
    fn new(key: &str, valids: &[i32]) -> Self {
        Rule {
            key: key.to_string(),
            valids: valids.to_owned(),
        }
    }

    fn from_str(str: &str) -> Self {
        let mut split = str.split(":");
        let key = split.next().unwrap().to_string();
        let ranges = split.next().unwrap().split(" or ");
        let mut valids = vec![];
        for range in ranges {
            let mut lowhigh = range.trim().split("-");
            let low = lowhigh.next().unwrap().parse::<i32>().unwrap();
            let high = lowhigh.next().unwrap().parse::<i32>().unwrap();
            for i in low..high + 1 {
                valids.push(i);
            }
        }
        Rule { key: key, valids }
    }
}

#[aoc_generator(day16)]
fn generator_input(input: &str) -> (Vec<Rule>, Vec<i32>, Vec<Vec<i32>>) {
    let mut rules = vec![];
    // let mut my_ticket = vec![];
    let mut nearby_tickets = vec![];
    let mut split = input.split("\n\n");
    for s in split.next().unwrap().lines() {
        rules.push(Rule::from_str(s));
    }

    let my_ticket = split
        .next()
        .unwrap()
        .lines()
        .last()
        .unwrap()
        .split(",")
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for ticket in split.next().unwrap().lines().skip(1) {
        nearby_tickets.push(
            ticket
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    (rules, my_ticket, nearby_tickets)
}

#[aoc(day16, part1)]
fn part1((rules, _, nearby_tickets): &(Vec<Rule>, Vec<i32>, Vec<Vec<i32>>)) -> i32 {
    let mut invalids = vec![];
    let all_valids = rules
        .iter()
        .flat_map(|r| r.valids.to_owned())
        .collect::<Vec<_>>();
    for ticket in nearby_tickets {
        for num in ticket {
            if !all_valids.contains(num) {
                invalids.push(*num);
            }
        }
    }
    invalids.iter().sum()
}

fn is_valid_ticket(ticket: &[i32], valid_nums: &[i32]) -> bool {
    for num in ticket {
        if !valid_nums.contains(num) {
            return false;
        }
    }
    true
}

#[aoc(day16, part2)]
fn part2((rules, my_ticket, nearby_tickets): &(Vec<Rule>, Vec<i32>, Vec<Vec<i32>>)) -> u64 {
    let valid_nums = rules
        .iter()
        .flat_map(|r| r.valids.to_owned())
        .collect::<Vec<_>>();
    let nearby_tickets = nearby_tickets
        .iter()
        .filter(|t| is_valid_ticket(t, &valid_nums))
        .collect::<Vec<_>>();

    let mut ordered_fields: HashMap<usize, &str> = HashMap::new();
    let mut used_fields: HashSet<String> = HashSet::new();

    while used_fields.len() != rules.len() {
        for i in 0..my_ticket.len() {
            if ordered_fields.contains_key(&i) {
                continue;
            }
            let mut potentials = vec![];
            for rule in rules {
                if used_fields.contains(&rule.key) {
                    continue;
                }
                let tickets_with_correct_position_count = nearby_tickets
                    .iter()
                    .filter(|t| rule.valids.contains(&t[i]))
                    .count();
                if tickets_with_correct_position_count == nearby_tickets.len() {
                    potentials.push(&rule.key);
                }
            }
            if potentials.len() == 1 {
                ordered_fields.insert(i, potentials[0]);
                used_fields.insert(potentials[0].to_string());
            }
        }
    }

    assert_eq!(ordered_fields.len(), rules.len());

    let mut result = 1_u64;

    for (i, field) in ordered_fields {
        if field.starts_with("departure") {
            result *= my_ticket[i] as u64;
        }
    }

    result
}

#[cfg(test)]
pub mod tests {
    use std::vec;

    use super::{generator_input, is_valid_ticket, part1, Rule};

    static INPUT_RAW: &str = r#"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12"#;

    #[test]
    fn generator() {
        let (rules, my_ticket, nearby_tickets) = generator_input(INPUT_RAW);
        assert_eq!(
            rules,
            vec![
                Rule::new("class", &vec![1, 2, 3, 5, 6, 7]),
                Rule::new(
                    "row",
                    &vec![6, 7, 8, 9, 10, 11, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44]
                ),
                Rule::new(
                    "seat",
                    &vec![
                        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
                        32, 33, 34, 35, 36, 37, 38, 39, 40, 45, 46, 47, 48, 49, 50
                    ]
                )
            ]
        );
        assert_eq!(my_ticket, vec![7, 1, 14]);
        assert_eq!(
            nearby_tickets,
            vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12]
            ]
        );
    }

    #[test]
    fn test_is_valid_ticket() {
        let (rules, my_ticket, nearby_tickets) = generator_input(INPUT_RAW);
        let all_valids = rules
            .iter()
            .flat_map(|r| r.valids.clone())
            .collect::<Vec<_>>();
        assert!(is_valid_ticket(&my_ticket, &all_valids));
        assert!(is_valid_ticket(&nearby_tickets[0], &all_valids));
        assert!(!is_valid_ticket(&nearby_tickets[1], &all_valids));
        assert!(!is_valid_ticket(&nearby_tickets[2], &all_valids));
        assert!(!is_valid_ticket(&nearby_tickets[3], &all_valids));
    }

    #[test]
    fn day16_part1() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part1(&input), 71);
    }
}
