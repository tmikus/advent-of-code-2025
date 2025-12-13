fn count_neighbours(map: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    if !map[y][x] {
        return 0;
    }
    let mut count = 0;
    for dy in -1_i32..=1 {
        let pos_y = y as i32 + dy;
        if pos_y < 0 || pos_y >= map.len() as i32 {
            continue;
        }
        let row = &map[pos_y as usize];
        for dx in -1_i32..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let pos_x = x as i32 + dx;
            if pos_x < 0 || pos_x >= row.len() as i32 {
                continue;
            }
            if row[pos_x as usize] {
                count += 1;
            }
        }
    }
    count
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|c| c == '@').collect())
        .collect()
}

fn solve_part_1(map: &Vec<Vec<bool>>) {
    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] && count_neighbours(map, x, y) < 4 {
                result += 1;
            }
        }
    }
    println!("Part 1: {}", result);
}

fn clear_accessible_spaces(map: &mut Vec<Vec<bool>>) -> usize {
    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] && count_neighbours(map, x, y) < 4 {
                result += 1;
                map[y][x] = false;
            }
        }
    }
    result
}

fn solve_part_2(mut map: Vec<Vec<bool>>) {
    let mut total_result = 0;
    loop {
        let result = clear_accessible_spaces(&mut map);
        if result == 0 {
            break;
        }
        total_result += result;
    }
    println!("Part 2: {}", total_result);
}

pub fn solve_puzzle(input: &str) {
    let map = parse_input(input);
    solve_part_1(&map);
    solve_part_2(map);
}
