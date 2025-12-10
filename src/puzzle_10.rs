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

struct State<'t> {
    desired_lights: &'t Vec<bool>,
    lights: Vec<bool>,
    press_count: usize,
}

impl<'t> State<'t> {
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
        State {
            desired_lights: self.desired_lights,
            lights: self.toggle_lights(button),
            press_count: self.press_count + 1,
        }
    }
}

fn get_min_button_press_count(input: &Input) -> usize {
    let mut next_states = VecDeque::from([State::new(&input.desired_lights)]);
    loop {
        let state = next_states.pop_front().unwrap();
        for button in &input.buttons {
            let next_state = state.press_button(button);
            if next_state.is_done() {
                println!("Press count: {}", next_state.press_count);
                return next_state.press_count;
            }
            next_states.push_back(next_state);
        }
    }
}

pub fn solve_puzzle(input: &str) {
    let inputs = input
        .lines()
        .map(|l| Input::parse(l.trim()))
        .collect::<Vec<_>>();
    for input in &inputs {
        println!("{:?}", input);
    }
    let mut result = 0;
    for input in inputs {
        result += get_min_button_press_count(&input);
    }
    println!("Result: {}", result);
}
