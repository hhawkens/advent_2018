use super::types::Point;
use super::data::COORDINATES;
use num_traits::real::Real;
use std::collections::HashSet;

pub fn get_coordinates() -> HashSet<Point> {
    let mut coordinates = HashSet::new();

    COORDINATES
        .lines()
        .for_each(|line| {
            let mut points = line.split(", ").map(|p| {
                p.parse::<i32>().unwrap()
            });
            coordinates.insert(Point { x: points.next().unwrap(), y: points.next().unwrap() });
        });

    coordinates
}

pub fn manhattan_distance(point_1: &Point, point_2: &Point) -> i32 {
    (point_1.x - point_2.x).abs() + (point_1.y - point_2.y).abs()
}
