use std::collections::HashSet;

#[derive(Debug, PartialEq)]
enum ManifoldEntry {
    Empty,
    Emitter,
    Splitter,
}

struct BeamSimulation {
    manifold: Vec<Vec<ManifoldEntry>>,
    steps: Vec<BeamSimulationStep>,
}

impl BeamSimulation {
    fn new(manifold: Vec<Vec<ManifoldEntry>>, (emitter_x, emitter_y): (usize, usize)) -> Self {
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

pub fn solve_puzzle(input: &str) {
    let manifold = parse_manifold(input);
    let emitter_position = find_emitter_position(&manifold).unwrap();
    let mut simulation = BeamSimulation::new(manifold, emitter_position);
    simulation.simulate_until_end();
    println!("Split count: {}", simulation.get_split_count());
}
