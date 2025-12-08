use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn new(x: u64, y: u64, z: u64) -> Self {
        Self { x, y, z }
    }

    fn distance_to(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }

    fn parse(input: &str) -> Self {
        let components = input
            .trim()
            .split(',')
            .map(|v| v.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        if components.len() != 3 {
            panic!("Invalid input");
        }
        Self::new(components[0], components[1], components[2])
    }
}

impl<'a> Hash for &'a JunctionBox {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use std::ptr::hash to hash the reference by its address.
        // `*self` is the &Position, and we pass it to the function.
        std::ptr::hash(*self, state);
    }
}

fn parse_junction_boxes(input: &str) -> Vec<JunctionBox> {
    input
        .trim()
        .lines()
        .map(|line| JunctionBox::parse(line))
        .collect()
}

#[derive(Debug)]
struct Circuit<'t> {
    junction_boxes: HashSet<&'t JunctionBox>,
}

impl<'t> Circuit<'t> {
    fn new() -> Self {
        Self {
            junction_boxes: HashSet::new(),
        }
    }

    fn add(&mut self, jb: &'t JunctionBox) {
        self.junction_boxes.insert(jb);
    }

    fn contains(&self, jb: &JunctionBox) -> bool {
        self.junction_boxes.contains(&jb)
    }

    fn join_with(&mut self, other: Self) {
        self.junction_boxes.extend(other.junction_boxes);
    }
}

struct CircuitManager<'t> {
    circuits: Vec<Circuit<'t>>,
}

impl<'t> CircuitManager<'t> {
    fn new(junction_boxes: &'t Vec<JunctionBox>) -> Self {
        Self {
            circuits: junction_boxes
                .iter()
                .map(|jb| {
                    let mut circuit = Circuit::new();
                    circuit.add(jb);
                    circuit
                })
                .collect(),
        }
    }

    fn connect_junction_boxes(&mut self, from: &'t JunctionBox, to: &'t JunctionBox) {
        let mut from_circuit = self.remove_circuit_containing_junction_box(from);
        if from_circuit.contains(to) {
            self.circuits.push(from_circuit);
            return;
        }
        let to_circuit = self.remove_circuit_containing_junction_box(to);
        from_circuit.join_with(to_circuit);
        self.circuits.push(from_circuit);
    }

    fn remove_circuit_containing_junction_box(&mut self, jb: &'t JunctionBox) -> Circuit<'t> {
        let index = self
            .circuits
            .iter()
            .position(|circuit| circuit.contains(jb))
            .unwrap();
        self.circuits.remove(index)
    }
}

pub fn solve_puzzle(input: &str) {
    let junction_boxes = parse_junction_boxes(input);
    let mut connections = Vec::<(u64, &JunctionBox, &JunctionBox)>::new();
    for (i, jb1) in junction_boxes.iter().enumerate() {
        for jb2 in junction_boxes.iter().skip(i + 1) {
            if jb1 == jb2 {
                continue;
            }
            let distance = jb1.distance_to(jb2);
            connections.push((distance, jb1, jb2));
        }
    }
    connections.sort_by(|(dist1, _, _), (dist2, _, _)| dist1.cmp(dist2));
    let mut manager = CircuitManager::new(&junction_boxes);
    for (_, source, destination) in connections.into_iter().take(junction_boxes.len()) {
        manager.connect_junction_boxes(source, destination);
    }
    let mut sorted_circuits = manager
        .circuits
        .iter()
        .map(|circuit| (circuit.junction_boxes.len(), circuit))
        .collect::<Vec<_>>();
    sorted_circuits.sort_by(|(size1, _), (size2, _)| size2.cmp(size1));
    let result: i64 = sorted_circuits
        .into_iter()
        .take(3)
        .map(|(size, _)| size as i64)
        .product();
    println!("Part 1: {}", result);
}
