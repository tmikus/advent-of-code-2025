use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
enum ManifoldEntry {
    Empty,
    Emitter,
    Splitter,
}

struct BeamSimulation<'t> {
    manifold: &'t Vec<Vec<ManifoldEntry>>,
    steps: Vec<BeamSimulationStep>,
}

impl<'t> BeamSimulation<'t> {
    fn new(manifold: &'t Vec<Vec<ManifoldEntry>>, (emitter_x, emitter_y): (usize, usize)) -> Self {
        let first_step = BeamSimulationStep::new(emitter_y, HashSet::from([emitter_x]));
        Self {
            manifold,
            steps: vec![first_step],
        }
    }

    fn get_split_count(&self) -> usize {
        self.steps.iter().map(|step| step.split_count).sum()
    }

    fn simulate_until_end(&mut self) {
        loop {
            let step = self.steps.last().unwrap();
            if step.y >= self.manifold.len() {
                break;
            }
            self.steps.push(step.simulate_tick(&self.manifold));
        }
    }
}

struct BeamSimulationStep {
    beams_x: HashSet<usize>,
    split_count: usize,
    y: usize,
}

impl BeamSimulationStep {
    fn new(y: usize, beams_x: HashSet<usize>) -> Self {
        Self {
            y,
            beams_x,
            split_count: 0,
        }
    }

    fn simulate_tick(&self, manifold: &Vec<Vec<ManifoldEntry>>) -> Self {
        let mut new_beams_x = HashSet::new();
        let manifold_row = &manifold[self.y];
        let mut split_count = 0;
        for x in self.beams_x.iter() {
            if manifold_row[*x] == ManifoldEntry::Splitter {
                split_count += 1;
                new_beams_x.insert(*x + 1);
                new_beams_x.insert(*x - 1);
            } else {
                new_beams_x.insert(*x);
            }
        }
        Self {
            y: self.y + 1,
            beams_x: new_beams_x,
            split_count,
        }
    }
}

fn find_emitter_position(manifold: &Vec<Vec<ManifoldEntry>>) -> Option<(usize, usize)> {
    for (y, row) in manifold.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            if *entry == ManifoldEntry::Emitter {
                return Some((x, y));
            }
        }
    }
    None
}

fn parse_manifold(input: &str) -> Vec<Vec<ManifoldEntry>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    'S' => ManifoldEntry::Emitter,
                    '^' => ManifoldEntry::Splitter,
                    _ => ManifoldEntry::Empty,
                })
                .collect()
        })
        .collect()
}

fn simulate_beam(
    manifold: &Vec<Vec<ManifoldEntry>>,
    (beam_x, beam_y): (usize, usize),
    previous_paths: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if beam_y >= manifold.len() {
        return 1;
    }
    let row = &manifold[beam_y];
    if beam_x >= row.len() {
        return 1;
    }
    if previous_paths.contains_key(&(beam_x, beam_y)) {
        return *previous_paths.get(&(beam_x, beam_y)).unwrap();
    }
    let entry = &row[beam_x];
    let result = match entry {
        ManifoldEntry::Splitter => {
            let right = simulate_beam(manifold, (beam_x + 1, beam_y + 1), previous_paths);
            let left = if beam_x > 0 {
                simulate_beam(manifold, (beam_x - 1, beam_y + 1), previous_paths)
            } else {
                0
            };
            left + right
        }
        _ => simulate_beam(manifold, (beam_x, beam_y + 1), previous_paths),
    };
    previous_paths.insert((beam_x, beam_y), result);
    result
}

pub fn solve_puzzle(input: &str) {
    let manifold = parse_manifold(input);
    let (emitter_x, emitter_y) = find_emitter_position(&manifold).unwrap();
    let mut simulation = BeamSimulation::new(&manifold, (emitter_x, emitter_y));
    simulation.simulate_until_end();
    println!("Part 1: {}", simulation.get_split_count());

    let mut previous_paths = HashMap::new();
    let beam_split_count =
        simulate_beam(&manifold, (emitter_x, emitter_y + 1), &mut previous_paths);
    println!("Part 2: {}", beam_split_count);
}
