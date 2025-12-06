use crate::utils::transpose;

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn apply(&self, numbers: &[i64]) -> i64 {
        match self {
            Operation::Add => numbers.iter().sum(),
            Operation::Multiply => numbers.iter().product(),
        }
    }
}

impl Into<Operation> for &str {
    fn into(self) -> Operation {
        match self {
            "+" => Operation::Add,
            "*" => Operation::Multiply,
            _ => panic!("Invalid operation"),
        }
    }
}

#[derive(Debug)]
struct Numbers {
    numbers: Vec<i64>,
    operation: Operation,
}

fn parse_input_rows<'t>(lines: &Vec<&'t str>) -> (Vec<&'t str>, &'t str) {
    let number_rows = lines
        .iter()
        .take(lines.len() - 1)
        .map(|s| *s)
        .collect::<Vec<_>>();
    let operation_row = lines.iter().last().unwrap();
    (number_rows, operation_row)
}

fn parse_part_1_numbers(number_rows: &Vec<&str>, operation_row: &str) -> Vec<Numbers> {
    let numbers = transpose(
        number_rows
            .into_iter()
            .map(|row| {
                row.split_whitespace()
                    .map(|n| n.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    let operations = operation_row
        .split_whitespace()
        .map(|op| op.into())
        .collect::<Vec<Operation>>();
    operations
        .into_iter()
        .zip(numbers)
        .map(|(op, numbers)| Numbers {
            numbers,
            operation: op,
        })
        .collect()
}

fn parse_part_2_numbers(number_rows: &Vec<&str>, operation_row: &str) -> Vec<Numbers> {
    let character_rows = transpose(
        number_rows
            .iter()
            .map(|c| c.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    );
    let digit_rows = character_rows
        .into_iter()
        .map(|row| {
            row.iter()
                .filter(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let numbers = digit_rows
        .into_iter()
        .map(|digits| {
            digits
                .into_iter()
                .rev()
                .enumerate()
                .fold(0, |acc, (i, digit)| acc + digit * 10_i64.pow(i as u32))
        })
        .collect::<Vec<_>>();
    let number_groups = numbers.into_iter().fold(Vec::new(), |mut acc, x| {
        if acc.is_empty() || x == 0 {
            acc.push(Vec::new());
            if x == 0 {
                return acc;
            }
        }
        acc.last_mut().unwrap().push(x);
        acc
    });
    let operations = operation_row
        .split_whitespace()
        .map(|op| op.into())
        .collect::<Vec<Operation>>();
    operations
        .into_iter()
        .zip(number_groups)
        .map(|(op, numbers)| Numbers {
            numbers,
            operation: op,
        })
        .collect()
}

fn solve_part_1(number_rows: &Vec<&str>, operation_row: &str) {
    let numbers = parse_part_1_numbers(number_rows, operation_row);
    let part_1: i64 = numbers.iter().map(|n| n.operation.apply(&n.numbers)).sum();
    println!("Part 1: {}", part_1);
}

fn solve_part_2(number_rows: &Vec<&str>, operation_row: &str) {
    let numbers = parse_part_2_numbers(number_rows, operation_row);
    let part_1: i64 = numbers.iter().map(|n| n.operation.apply(&n.numbers)).sum();
    println!("Part 2: {}", part_1);
}

pub fn solve_puzzle(input: &str) {
    let lines = input.lines().collect();
    let (number_rows, operation_row) = parse_input_rows(&lines);
    solve_part_1(&number_rows, operation_row);
    solve_part_2(&number_rows, operation_row);
}
