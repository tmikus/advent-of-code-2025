use crate::utils;
use std::cell::RefCell;
use std::collections::HashMap;

struct NumberFinder {
    cache: RefCell<HashMap<(usize, u32), i64>>,
    numbers: Vec<i64>,
}

impl NumberFinder {
    pub fn new(numbers: Vec<i64>) -> Self {
        Self { cache: RefCell::new(HashMap::new()), numbers }
    }

    pub fn find_highest_number(&self, start_index: usize, digits: u32) -> i64 {
        if digits == 0 {
            return 0;
        }
        if let Some(value) = self.cache.borrow().get(&(start_index, digits)) {
            return *value
        }
        let multiplier = 10_i64.pow(digits - 1);
        let mut highest_number = 0;
        for (number_index, high_number) in self.numbers[start_index..=self.numbers.len() - digits as usize].iter().enumerate() {
            let lower_digits = self.find_highest_number(start_index + number_index + 1, digits - 1);
            let number = (*high_number * multiplier) + lower_digits;
            if number > highest_number {
                highest_number = number;
            }
        }
        self.cache.borrow_mut().insert((start_index, digits), highest_number);
        highest_number
    }
}

fn parse_line(line: &str) -> Vec<i64> {
    line.trim().chars().map(|c| c.to_string().parse::<i64>().unwrap()).collect()
}

pub fn solve_puzzle(input: &str) {
    let lines = utils::extract_lines(input);
    let battery_banks = lines.into_iter().map(parse_line).collect::<Vec<_>>();
    let mut result_1 = 0;
    let mut result_2 = 0;
    for bank in battery_banks {
        let finder = NumberFinder::new(bank.clone());
        result_1 += finder.find_highest_number(0, 2);
        result_2 += finder.find_highest_number(0, 12);
    }
    println!("Part 1: {}", result_1);
    println!("Part 2: {}", result_2);
}
