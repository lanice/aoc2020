use std::vec;

type Expression = Vec<char>;

#[aoc_generator(day18)]
fn generator_input(input: &str) -> Vec<Expression> {
    input
        .lines()
        .map(|a| a.chars().filter(|&c| c != ' ').collect::<Expression>())
        .collect::<Vec<_>>()
}

fn op(left: u64, op: char, right: u64) -> u64 {
    match op {
        '+' => left + right,
        '*' => left * right,
        _ => panic!("Invalid operator!"),
    }
}

// The commented out code below was a failed attempt at solving part 1. I'm not entirely sure why it did not work,
// but at some point I just decided to rewrite it with a slightly different approach, which worked.

// fn eval_expr(expr: &Expression) -> u64 {
//     let mut expr = expr.clone();
//     expr.reverse();
//     // println!("{:?}", expr);
//     let mut result_stack: Vec<u64> = vec![];
//     let mut op_stack: Vec<char> = vec![];
//     let mut open_parenthesis = false;

//     while let Some(op_or_digit) = expr.pop() {
//         // println!(
//         //     "{:?} - {:?} - {:?}",
//         //     result_stack, op_stack, open_parenthesis
//         // );
//         if let Some(digit) = op_or_digit.to_digit(10) {
//             if let Some(left) = result_stack.pop() {
//                 if open_parenthesis {
//                     result_stack.push(left);
//                     result_stack.push(digit as u64);
//                     open_parenthesis = false;
//                 } else {
//                     let operator = op_stack.pop().unwrap();
//                     result_stack.push(op(left, operator, digit as u64));
//                 }
//             } else {
//                 result_stack.push(digit as u64);
//                 open_parenthesis = false;
//             }
//         } else {
//             match op_or_digit {
//                 '(' => {
//                     open_parenthesis = true;
//                     // result_stack.push(0);
//                 }
//                 ')' => {
//                     if let Some(operator) = op_stack.pop() {
//                         let right = result_stack.pop().unwrap();
//                         let left = result_stack.pop().unwrap();
//                         result_stack.push(op(left, operator, right));
//                     }
//                 }
//                 operator => {
//                     op_stack.push(operator);
//                 }
//             }
//         }
//     }

//     // println!("{:?} - {:?}", result_stack, op_stack);
//     assert_eq!(result_stack.len(), 1);
//     assert_eq!(op_stack.len(), 0);
//     result_stack.pop().unwrap()
// }

fn resolve_parenthesis(expr: &Expression, eval_func: fn(&Expression) -> u64) -> Vec<String> {
    let mut expr = expr.clone();
    expr.reverse();

    let mut clean_expr = vec![];

    while let Some(item) = expr.pop() {
        match item {
            '(' => {
                let mut open_count = 1;
                let mut sub_expr = vec![];
                while open_count > 0 {
                    let next = expr.pop().unwrap();
                    sub_expr.push(next);
                    match next {
                        '(' => {
                            open_count += 1;
                        }
                        ')' => {
                            open_count -= 1;
                        }
                        _ => {}
                    }
                }
                sub_expr.pop();
                clean_expr.push(eval_func(&sub_expr).to_string());
            }
            op_or_digit => {
                clean_expr.push(op_or_digit.to_string());
            }
        }
    }

    clean_expr
}

fn eval_expr(expr: &Expression) -> u64 {
    let mut clean_expr = resolve_parenthesis(expr, eval_expr);

    clean_expr.reverse();

    let mut result: u64 = clean_expr.pop().unwrap().parse::<u64>().unwrap();
    let mut curr_op: Option<char> = None;

    while let Some(item) = clean_expr.pop() {
        match item.as_str() {
            "+" => curr_op = Some('+'),
            "*" => curr_op = Some('*'),
            number => {
                result = op(result, curr_op.unwrap(), number.parse::<u64>().unwrap());
            }
        }
    }

    result
}

fn eval_expr_part_2(expr: &Expression) -> u64 {
    let mut clean_expr = resolve_parenthesis(expr, eval_expr_part_2);

    clean_expr.reverse();

    let mut numbers: Vec<u64> = vec![];
    let mut sum_next = false;

    while let Some(item) = clean_expr.pop() {
        match item.as_str() {
            "+" => {
                sum_next = true;
            }
            "*" => {}
            number => {
                if sum_next {
                    let left = numbers.pop().unwrap();
                    let right = number.parse::<u64>().unwrap();
                    numbers.push(op(left, '+', right));
                    sum_next = false;
                } else {
                    numbers.push(number.parse::<u64>().unwrap());
                }
            }
        }
    }

    numbers.iter().product()
}

#[aoc(day18, part1)]
fn part1(input: &[Expression]) -> u64 {
    input.iter().map(|e| eval_expr(e)).sum::<u64>()
}

#[aoc(day18, part2)]
fn part2(input: &[Expression]) -> u64 {
    input.iter().map(|e| eval_expr_part_2(e)).sum::<u64>()
}

#[cfg(test)]
pub mod tests {
    use super::{generator_input, part1, part2};

    static INPUT_RAW: &str = r#"2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"#;

    #[test]
    fn generator() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(input[0], vec!['2', '*', '3', '+', '(', '4', '*', '5', ')']);
        assert_eq!(
            input[1],
            vec!['5', '+', '(', '8', '*', '3', '+', '9', '+', '3', '*', '4', '*', '3', ')']
        );
    }

    #[test]
    fn day18_part1() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part1(&input), 26 + 437 + 12240 + 13632);
    }

    #[test]
    fn day18_part2() {
        let input = generator_input(INPUT_RAW);
        assert_eq!(part2(&input), 46 + 1445 + 669060 + 23340);
    }
}
