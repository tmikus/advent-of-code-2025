use good_lp::{constraint, default_solver, variable, Expression, Solution, SolverModel};
use std::collections::VecDeque;

#[derive(Debug)]
struct Input {
    desired_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Input {
    fn parse(input: &str) -> Input {
        let pairs = input.split(" ").collect::<Vec<&str>>();
        Input {
            desired_lights: Self::parse_desired_lights(pairs[0]),
            buttons: Self::parse_buttons(&pairs[1..pairs.len() - 1]),
            joltage: Self::parse_joltage(pairs[pairs.len() - 1]),
        }
    }

    fn parse_buttons(buttons: &[&str]) -> Vec<Vec<usize>> {
        buttons
            .iter()
            .map(|s| {
                let s = s.trim();
                s[1..s.len() - 1]
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            })
            .collect()
    }

    fn parse_desired_lights(input: &str) -> Vec<bool> {
        input
            .chars()
            .filter(|c| c != &'[' && c != &']')
            .map(|c| c == '#')
            .collect::<Vec<bool>>()
    }

    fn parse_joltage(input: &str) -> Vec<usize> {
        let input = input.trim();
        input[1..input.len() - 1]
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
    }
}

struct LightState<'t> {
    desired_lights: &'t Vec<bool>,
    lights: Vec<bool>,
    press_count: usize,
}

impl<'t> LightState<'t> {
    fn new(desired_lights: &'t Vec<bool>) -> Self {
        Self {
            desired_lights,
            lights: vec![false; desired_lights.len()],
            press_count: 0,
        }
    }

    fn is_done(&self) -> bool {
        self.lights == *self.desired_lights
    }

    fn toggle_lights(&self, light_indices: &Vec<usize>) -> Vec<bool> {
        let mut lights = self.lights.clone();
        for i in light_indices {
            lights[*i] = !lights[*i];
        }
        lights
    }

    fn press_button(&self, button: &Vec<usize>) -> Self {
        LightState {
            desired_lights: self.desired_lights,
            lights: self.toggle_lights(button),
            press_count: self.press_count + 1,
        }
    }
}

fn get_min_button_press_count_for_lights(input: &Input) -> usize {
    let mut next_states = VecDeque::from([LightState::new(&input.desired_lights)]);
    loop {
        let state = next_states.pop_front().unwrap();
        for button in &input.buttons {
            let next_state = state.press_button(button);
            if next_state.is_done() {
                return next_state.press_count;
            }
            next_states.push_back(next_state);
        }
    }
}

fn solve_part_1(inputs: &Vec<Input>) {
    let mut result = 0;
    for input in inputs {
        result += get_min_button_press_count_for_lights(&input);
    }
    println!("Part 1: {}", result);
}

fn get_min_button_press_count_for_joltage(input: &Input) -> usize {
    let mut problem_vars = vec![];
    let mut problem = good_lp::ProblemVariables::new();
    for _ in 0..input.buttons.len() {
        problem_vars.push(problem.add(variable().integer().min(0)));
    }
    let objective: Expression = problem_vars.iter().sum();
    let mut model = problem.minimise(&objective).using(default_solver);
    for (counter_idx, &target_val) in input.joltage.iter().enumerate() {
        let mut expr = Expression::from(0);
        for (btn_idx, affected_indices) in input.buttons.iter().enumerate() {
            if affected_indices.contains(&counter_idx) {
                expr += problem_vars[btn_idx];
            }
        }
        model.add_constraint(constraint!(expr == (target_val as i32)));
    }
    match model.solve() {
        Ok(solution) => solution.eval(&objective) as usize,
        Err(_) => {
            panic!("Failed to solve model");
        }
    }
}

fn solve_part_2(inputs: &Vec<Input>) {
    let mut result = 0;
    for input in inputs {
        result += get_min_button_press_count_for_joltage(&input);
    }
    println!("Part 2: {}", result);
}

pub fn solve_puzzle(input: &str) {
    let inputs = input
        .lines()
        .map(|l| Input::parse(l.trim()))
        .collect::<Vec<_>>();
    solve_part_1(&inputs);
    solve_part_2(&inputs);
}
