use crate::utils;

fn parse_line(line: &str) -> Vec<i64> {
    line.trim().chars().map(|c| c.to_string().parse::<i64>().unwrap()).collect()
}

fn find_highest_battery_combination(numbers: &[i64], digits: u32) -> i64 {
    if digits == 0 {
        return 0;
    }
    let multiplier = 10_i64.pow(digits - 1);
    let mut highest_number = 0;
    for (start_index, high_number) in numbers[..=numbers.len() - digits as usize].iter().enumerate() {
        let lower_digits = find_highest_battery_combination(&numbers[start_index + 1..], digits - 1);
        let number = (*high_number * multiplier) + lower_digits;
        if number > highest_number {
            highest_number = number;
        }
    }
    highest_number
}

pub fn solve_puzzle(input: &str) {
    let lines = utils::extract_lines(input);
    let battery_banks = lines.into_iter().map(parse_line).collect::<Vec<_>>();
    let mut result = 0;
    let mut result_2 = 0;
    for bank in battery_banks {
        result += find_highest_battery_combination(&bank, 2);
        result_2 += find_highest_battery_combination(&bank, 12);
    }
    println!("Part 1: {}", result);
    println!("Part 2: {}", result_2);
}
