use onig::EncodedChars;
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

fn parse_input(input: &str) -> Vec<Numbers> {
    let lines: Vec<&str> = input.lines().collect();
    let number_rows = lines.iter().take(lines.len() - 1).collect::<Vec<_>>();
    let operation_row = lines.iter().last().unwrap();
    let numbers = transpose(number_rows
        .into_iter()
        .map(|row| row.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>());
    let operations = operation_row.split_whitespace().map(|op| op.into()).collect::<Vec<Operation>>();
    operations.into_iter().zip(numbers).map(|(op, numbers)| Numbers { numbers, operation: op }).collect()
}

pub fn solve_puzzle(input: &str) {
    let numbers = parse_input(input);
    let part_1: i64 = numbers.iter().map(|n| n.operation.apply(&n.numbers)).sum();
    println!("Part 1: {}", part_1);
}

