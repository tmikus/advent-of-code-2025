mod puzzle_1;
mod puzzle_2;
mod puzzle_3;
mod puzzle_4;
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
    ];
    println!("Which puzzle would you like to run? [1-{}]", puzzles.len());
    let puzzle_number = read_number();
    if puzzle_number <= 0 || puzzle_number > puzzles.len() as i32 {
        panic!("Invalid puzzle number");
    }
    let (solve_puzzle, path) = puzzles[(puzzle_number - 1) as usize];
    let puzzle_input = read_puzzle_input(path);
    solve_puzzle(&puzzle_input);
}
