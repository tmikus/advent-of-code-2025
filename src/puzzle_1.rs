fn extract_lines(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn parse_dial_movement(line: &str) -> i32 {
    let direction = line.trim().chars().next().unwrap();
    let distance = line.trim().chars().skip(1).collect::<String>().parse::<i32>().unwrap();
    distance * match direction {
        'R' => 1,
        'L' => -1,
        _ => panic!("Invalid direction"),
    }
}

fn solve_part_1(lines: &[&str]) {
    let mut position = 50;
    let mut count = 0;
    for line in lines {
        position += parse_dial_movement(line);
        position %= 100;
        if position == 0 {
            count += 1;
        }
    }
    println!("Part 1: {}", count);
}

fn solve_part_2(lines: &[&str]) {
    let mut position = 50;
    let mut count = 0;
    for line in lines {
        let dial_movement = parse_dial_movement(line);
        if dial_movement > 0 {
            for _ in 0..dial_movement {
                position += 1;
                position %= 100;
                if position == 0 {
                    count += 1;
                }
            }
        } else {
            for _ in 0..dial_movement.abs() {
                position -= 1;
                position %= 100;
                if position == 0 {
                    count += 1;
                }
            }
        }
    }
    println!("Part 2: {}", count);
}

pub fn solve_puzzle(input: &str) {
    let lines = extract_lines(input);
    solve_part_1(&lines);
    solve_part_2(&lines);
}


