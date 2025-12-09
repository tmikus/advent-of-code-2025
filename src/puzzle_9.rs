use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, PartialEq)]
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

struct Grid<'t> {
    height: usize,
    points: Vec<Vec<Option<&'t Point>>>,
    width: usize,
}

impl<'t> Grid<'t> {
    fn new(points: &'t Vec<Point>) -> Grid<'t> {
        let width = points.iter().map(|p| p.x).max().unwrap() + 1;
        let height = points.iter().map(|p| p.y).max().unwrap() + 1;
        let mut grid = Grid {
            height,
            points: vec![vec![None; width]; height],
            width,
        };
        for point in points.iter() {
            grid.points[point.y][point.x] = Some(point);
        }
        grid
    }

    fn find_next_points(&'t self, point: &'t Point) -> Vec<&'t Point> {
        let mut result = Vec::new();
        if let Some(p) = self.find_point_on_x_axis(point, -1) {
            result.push(p);
        }
        if let Some(p) = self.find_point_on_x_axis(point, 1) {
            result.push(p);
        }
        if let Some(p) = self.find_point_on_y_axis(point, -1) {
            result.push(p);
        }
        if let Some(p) = self.find_point_on_y_axis(point, 1) {
            result.push(p);
        }
        result
    }

    fn find_point_on_x_axis(&self, point: &Point, direction: i32) -> Option<&Point> {
        let mut x = point.x as i32;
        loop {
            x += direction;
            if x < 0 || x >= self.width as i32 {
                return None;
            }
            if let Some(p) = self.points[point.y][x as usize] {
                return Some(p);
            }
        }
    }

    fn find_point_on_y_axis(&self, point: &Point, direction: i32) -> Option<&Point> {
        let mut y = point.y as i32;
        loop {
            y += direction;
            if y < 0 || y >= self.height as i32 {
                return None;
            }
            if let Some(p) = self.points[y as usize][point.x] {}
        }
    }
}

struct Edge<'t> {
    from: &'t Point,
    to: &'t Point,
}

fn build_edges<'t>(grid: &'t Grid<'t>, start_point: &'t Point) -> Vec<Edge<'t>> {
    let mut edges = Vec::new();
    let mut visited_points = HashSet::<&Point>::new();
    let mut queue = vec![start_point];
    while let Some(point) = queue.pop() {
        visited_points.insert(point);
        for next_point in grid.find_next_points(point) {
            if visited_points.contains(&next_point) {
                continue;
            }
            edges.push(Edge {
                from: point,
                to: next_point,
            });
            queue.push(next_point);
        }
    }
    edges
}

fn solve_part_2(points: &Vec<Point>) {
    println!("Building grid...");
    let grid = Grid::new(points);
    println!("Building edges...");
    let edges = build_edges(&grid, &points[0]);
    println!("Part 2: {}", edges.len());
}

pub fn solve_puzzle(input: &str) {
    let points = parse_points(input);
    let points = align_points_to_0_0(points);
    solve_part_1(&points);
    solve_part_2(&points);
}
