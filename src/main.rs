mod puzzle_1;
mod puzzle_10;
mod puzzle_11;
mod puzzle_12;
mod puzzle_2;
mod puzzle_3;
mod puzzle_4;
mod puzzle_5;
mod puzzle_6;
mod puzzle_7;
mod puzzle_8;
mod puzzle_9;
mod utils;

fn read_number() -> i32 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap()
}

fn read_puzzle_input(path: &str) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn main() {
    let puzzles: Vec<(fn(&str), &str)> = vec![
        (puzzle_1::solve_puzzle, "inputs/puzzle_1.txt"),
        (puzzle_2::solve_puzzle, "inputs/puzzle_2.txt"),
        (puzzle_3::solve_puzzle, "inputs/puzzle_3.txt"),
        (puzzle_4::solve_puzzle, "inputs/puzzle_4.txt"),
        (puzzle_5::solve_puzzle, "inputs/puzzle_5.txt"),
        (puzzle_6::solve_puzzle, "inputs/puzzle_6.txt"),
        (puzzle_7::solve_puzzle, "inputs/puzzle_7.txt"),
        (puzzle_8::solve_puzzle, "inputs/puzzle_8.txt"),
        (puzzle_9::solve_puzzle, "inputs/puzzle_9.txt"),
        (puzzle_10::solve_puzzle, "inputs/puzzle_10.txt"),
        (puzzle_11::solve_puzzle, "inputs/puzzle_11.txt"),
        (puzzle_12::solve_puzzle, "inputs/puzzle_12.txt"),
    ];
    println!("Which puzzle would you like to run? [1-{}]", puzzles.len());
    let puzzle_number = read_number();
    if puzzle_number <= 0 || puzzle_number > puzzles.len() as i32 {
        panic!("Invalid puzzle number");
    }
    let (solve_puzzle, path) = puzzles[(puzzle_number - 1) as usize];
    let puzzle_input = read_puzzle_input(path);
    let start_time = std::time::Instant::now();
    solve_puzzle(&puzzle_input);
    println!("Time elapsed: {:?}", start_time.elapsed());
}
