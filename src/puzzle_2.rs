use onig::Regex;

fn get_number_pairs(input: &str) -> Vec<(i64, i64)> {
    input
        .trim()
        .split(",")
        .map(|id_pair| {
            let id_pair = id_pair.trim().split_once("-").unwrap();
            let (id1, id2) = id_pair;
            (id1.parse::<i64>().unwrap(), id2.parse::<i64>().unwrap())
        })
        .collect()
}

fn is_invalid_id_part_1(id: &str) -> bool {
    let regex = Regex::new(r"^(\d+)\1$").unwrap();
    regex.is_match(id)
}

fn is_invalid_id_part_2(id: &str) -> bool {
    let regex = Regex::new(r"^(\d+)\1+$").unwrap();
    regex.is_match(id)
}

fn solve_part_1(number_pairs: &[(i64, i64)]) {
    let mut result = 0;
    for (from, to) in number_pairs {
        for id in *from..=*to {
            if is_invalid_id_part_1(&id.to_string()) {
                result += id;
            }
        }
    }
    println!("Part 1: {}", result);
}

fn solve_part_2(number_pairs: &[(i64, i64)]) {
    let mut result = 0;
    for (from, to) in number_pairs {
        for id in *from..=*to {
            if is_invalid_id_part_2(&id.to_string()) {
                result += id;
            }
        }
    }
    println!("Part 2: {}", result);
}

pub fn solve_puzzle(input: &str) {
    let number_pairs = get_number_pairs(input);
    solve_part_1(&number_pairs);
    solve_part_2(&number_pairs)
}


