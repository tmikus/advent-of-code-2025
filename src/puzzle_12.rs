use std::fmt::Debug;

const PRESENT_SIZE: usize = 3;

type PresentData = [[bool; PRESENT_SIZE]; PRESENT_SIZE]; // Each shape is 3x3

struct PresentShape {
    required_spaces: usize,
    variants: Vec<PresentData>,
}

impl PresentShape {
    fn flip_horizontally(data: &PresentData) -> PresentData {
        data.iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn rotate_left(data: &PresentData) -> PresentData {
        let mut rotated_data = [[false; PRESENT_SIZE]; PRESENT_SIZE];
        for y in 0..PRESENT_SIZE {
            for x in 0..PRESENT_SIZE {
                rotated_data[PRESENT_SIZE - x - 1][y] = data[y][x];
            }
        }
        rotated_data
    }

    fn parse(input: &[&str]) -> PresentShape {
        let mut last_variant = Self::parse_base_variant(input);
        let mut required_spaces = 0;
        for row in last_variant.iter() {
            required_spaces += row.iter().filter(|&&b| b).count();
        }
        let mut variants = vec![last_variant.clone()];
        for _ in 0..=3 {
            last_variant = Self::rotate_left(&last_variant);
            if variants.iter().any(|v| *v == last_variant) {
                continue;
            }
            variants.push(last_variant.clone());
            let flipped_variant = Self::flip_horizontally(&last_variant);
            if variants.iter().any(|v| *v == flipped_variant) {
                continue;
            }
            variants.push(flipped_variant);
        }
        PresentShape {
            required_spaces,
            variants,
        }
    }

    fn parse_base_variant(input: &[&str]) -> PresentData {
        let mut base_variant: PresentData = [[false; PRESENT_SIZE]; PRESENT_SIZE];
        let data = input
            .iter()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for y in 0..PRESENT_SIZE {
            for x in 0..PRESENT_SIZE {
                base_variant[y][x] = data[y][x];
            }
        }
        base_variant
    }
}

#[derive(Debug)]
struct TreeArea {
    height: usize,
    width: usize,
    presents: Vec<usize>,
}

impl TreeArea {
    fn parse(input: &&str) -> Self {
        let parts = input.split(": ").collect::<Vec<&str>>();
        let dimensions = parts[0]
            .split("x")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let presents = parts[1]
            .split(" ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Self {
            width: dimensions[0],
            height: dimensions[1],
            presents,
        }
    }
}

fn parse_present_shapes(lines: &[&str]) -> Vec<PresentShape> {
    lines
        .split(|line| line.trim() == "")
        .map(|lines| PresentShape::parse(&lines[1..]))
        .collect()
}

fn parse_tree_areas(lines: &[&str]) -> Vec<TreeArea> {
    lines.iter().map(TreeArea::parse).collect()
}

fn parse_input(input: &str) -> (Vec<PresentShape>, Vec<TreeArea>) {
    let lines = input.lines().collect::<Vec<&str>>();
    let last_empty_line_index = lines.iter().rposition(|l| l.trim() == "").unwrap();
    let present_shapes = parse_present_shapes(&lines[..last_empty_line_index]);
    let tree_areas = parse_tree_areas(&lines[last_empty_line_index + 1..]);
    (present_shapes, tree_areas)
}

fn place_remaining_presents(
    area: &Vec<Vec<bool>>,
    area_width: usize,
    area_height: usize,
    present_shapes: &[PresentShape],
    remaining_presents: &[usize],
) -> bool {
    if remaining_presents.iter().all(|&p| p == 0) {
        return true;
    }
    for (present_index, &remaining_present_count) in remaining_presents.iter().enumerate() {
        if remaining_present_count == 0 {
            continue;
        }
        let updated_remaining_presents = remaining_presents
            .iter()
            .enumerate()
            .map(|(i, &count)| if i == present_index { count - 1 } else { count })
            .collect::<Vec<_>>();
        if try_placing_present(
            area,
            area_width,
            area_height,
            present_shapes,
            present_shapes.get(present_index).unwrap(),
            &updated_remaining_presents,
        ) {
            return true;
        }
    }
    false
}

fn does_present_fit(area: &Vec<Vec<bool>>, present_data: &PresentData, x: usize, y: usize) -> bool {
    for y_offset in 0..PRESENT_SIZE {
        for x_offset in 0..PRESENT_SIZE {
            if present_data[y_offset][x_offset] == false {
                continue;
            }
            if area[y + y_offset][x + x_offset] {
                return false;
            }
        }
    }
    true
}

fn place_variant(
    area: &Vec<Vec<bool>>,
    present_data: &PresentData,
    x: usize,
    y: usize,
) -> Vec<Vec<bool>> {
    let mut area = area.clone();
    for y_offset in 0..PRESENT_SIZE {
        for x_offset in 0..PRESENT_SIZE {
            area[y + y_offset][x + x_offset] |= present_data[y_offset][x_offset];
        }
    }
    area
}

fn try_placing_present(
    area: &Vec<Vec<bool>>,
    area_width: usize,
    area_height: usize,
    present_shapes: &[PresentShape],
    present_shape: &PresentShape,
    remaining_presents: &[usize],
) -> bool {
    for variant in &present_shape.variants {
        for y in 0..=area_height - PRESENT_SIZE {
            for x in 0..=area_width - PRESENT_SIZE {
                if !does_present_fit(area, variant, x, y) {
                    continue;
                }
                let updated_area = place_variant(area, variant, x, y);
                if place_remaining_presents(
                    &updated_area,
                    area_width,
                    area_height,
                    present_shapes,
                    remaining_presents,
                ) {
                    return true;
                }
            }
        }
    }
    false
}

pub fn solve_puzzle(input: &str) {
    let (present_shapes, tree_areas) = parse_input(input);
    let mut count = 0;
    for (tree_index, tree_area) in tree_areas.iter().enumerate() {
        let required_spaces = tree_area
            .presents
            .iter()
            .enumerate()
            .map(|(i, &count)| count * present_shapes[i].required_spaces)
            .sum::<usize>();
        if required_spaces > tree_area.width * tree_area.height {
            continue;
        }
        if place_remaining_presents(
            &vec![vec![false; tree_area.width]; tree_area.height],
            tree_area.width,
            tree_area.height,
            &present_shapes,
            &tree_area.presents,
        ) {
            count += 1;
        }
        println!("Checked tree area {}/{}", tree_index + 1, tree_areas.len())
    }
    println!("Part 1: {}", count);
}
