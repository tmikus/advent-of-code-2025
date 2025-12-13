use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone)]
struct Path<'t> {
    count: HashMap<(bool, bool), RefCell<Option<usize>>>,
    label: &'t str,
}

impl<'t> Path<'t> {
    fn new(label: &'t str) -> Self {
        Self {
            count: HashMap::from([
                ((false, false), RefCell::new(None)),
                ((false, true), RefCell::new(None)),
                ((true, false), RefCell::new(None)),
                ((true, true), RefCell::new(None)),
            ]),
            label,
        }
    }
}

fn parse_connections(input: &'_ str) -> HashMap<&'_ str, Vec<Path<'_>>> {
    let mut connections = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let key = parts[0];
        let paths = parts[1].split(" ").map(|label| Path::new(label)).collect();
        connections.insert(key, paths);
    }
    connections
}

fn count_connections_until_label(
    connections: &HashMap<&str, Vec<Path>>,
    from_label: &str,
    to_label: &str,
) -> usize {
    let next_paths = connections.get(from_label).unwrap();
    let mut result = 0;
    for path in next_paths {
        let count = if path.label == to_label {
            1
        } else if path.label != from_label && path.label != "out" {
            count_connections_until_label(connections, path.label, to_label)
        } else {
            0
        };
        result += count;
    }
    result
}

fn count_connections_until_label_going_through_dac_and_fft(
    connections: &HashMap<&str, Vec<Path>>,
    from_label: &str,
    to_label: &str,
    found_dac: bool,
    found_fft: bool,
) -> usize {
    let next_labels = connections.get(from_label).unwrap();
    let mut result = 0;
    for path in next_labels {
        match *path.count.get(&(found_dac, found_fft)).unwrap().borrow() {
            Some(count) => {
                result += count;
                continue;
            }
            _ => (),
        }
        let path_result = if path.label == to_label {
            if found_dac && found_fft { 1 } else { 0 }
        } else {
            count_connections_until_label_going_through_dac_and_fft(
                connections,
                path.label,
                to_label,
                found_dac || path.label == "dac",
                found_fft || path.label == "fft",
            )
        };
        *path
            .count
            .get(&(found_dac, found_fft))
            .unwrap()
            .borrow_mut() = Some(path_result);
        result += path_result;
    }
    result
}

pub fn solve_puzzle(input: &str) {
    let connections = parse_connections(input);
    let result = count_connections_until_label(&connections.clone(), "you", "out");
    println!("Part 1: {}", result);
    let result = count_connections_until_label_going_through_dac_and_fft(
        &connections,
        "svr",
        "out",
        false,
        false,
    );
    println!("Part 2: {}", result);
}
