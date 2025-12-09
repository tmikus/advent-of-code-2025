use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn area(&self, other: &Point) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

impl<'a> Hash for &'a Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Use std::ptr::hash to hash the reference by its address.
        // `*self` is the &Position, and we pass it to the function.
        std::ptr::hash(*self, state);
    }
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            let components = line
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            Point::new(components[0], components[1])
        })
        .collect()
}

fn align_points_to_0_0(points: Vec<Point>) -> Vec<Point> {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    points
        .into_iter()
        .map(|p| Point::new(p.x - min_x, p.y - min_y))
        .collect()
}

fn solve_part_1(points: &Vec<Point>) {
    let mut largest_area = 0;
    for (i, from_point) in points.iter().enumerate() {
        for to_point in points.iter().skip(i + 1) {
            largest_area = largest_area.max(from_point.area(to_point));
        }
    }
    println!("Part 1: {}", largest_area);
}

#[derive(Hash, Eq, PartialEq)]
struct Rectangle {
    x_min: usize,
    y_min: usize,
    x_max: usize,
    y_max: usize,
}

impl Rectangle {
    fn new(from: &Point, to: &Point) -> Rectangle {
        let x_min = from.x.min(to.x);
        let x_max = from.x.max(to.x);
        let y_min = from.y.min(to.y);
        let y_max = from.y.max(to.y);
        Rectangle {
            x_min,
            y_min,
            x_max,
            y_max,
        }
    }

    pub fn overlaps(&self, other: &Rectangle) -> bool {
        let separated = self.x_max <= other.x_min
            || other.x_max <= self.x_min
            || self.y_max <= other.y_min
            || other.y_max <= self.y_min;
        !separated
    }
}

fn build_edges(points: &'_ Vec<Point>) -> HashSet<Rectangle> {
    let mut remaining_points = points.iter().collect::<Vec<_>>();
    let mut edges = HashSet::new();
    while let Some(from_point) = remaining_points.pop() {
        if let Some(to_point) = points.iter().find(|p| p.y == from_point.y) {
            edges.insert(Rectangle::new(from_point, to_point));
        }
        if let Some(to_point) = points.iter().find(|p| p.x == from_point.x) {
            edges.insert(Rectangle::new(from_point, to_point));
        }
    }
    edges
}

fn solve_part_2(points: &Vec<Point>) {
    let mut largest_area = 0;
    let edges = build_edges(points);
    for (i, from_point) in points.iter().enumerate() {
        for to_point in points.iter().skip(i + 1) {
            let rect = Rectangle::new(from_point, to_point);
            if edges.iter().any(|edge| edge.overlaps(&rect)) {
                continue;
            }
            let area = from_point.area(to_point);
            if area < largest_area {
                continue;
            }
            largest_area = area;
        }
    }
    println!("Part 2: {}", largest_area);
}

pub fn solve_puzzle(input: &str) {
    let points = parse_points(input);
    let points = align_points_to_0_0(points);
    solve_part_1(&points);
    solve_part_2(&points);
}
