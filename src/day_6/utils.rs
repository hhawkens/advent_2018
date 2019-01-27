use super::types::{Point, AxisDirection};
use std::collections::HashSet;
use num_traits::real::Real;

const MAX_INT: i32 = i32::max_value();
const MIN_INT: i32 = i32::min_value();

pub fn get_coordinates(coord_text: &str) -> Vec<Point> {
    coord_text
        .lines()
        .map(|line| {
            let mut points = line.split(", ").map(|p| p.parse::<i32>().unwrap());
            Point {
                x: points.next().unwrap(),
                y: points.next().unwrap(),
            }
        })
        .collect()
}

pub fn manhattan_distance(point_1: &Point, point_2: &Point) -> u64 {
    ((point_1.x as i64 - point_2.x as i64).abs() + (point_1.y as i64 - point_2.y as i64).abs()) as u64
}

pub fn get_limited_area_points(points: &Vec<Point>) -> HashSet<&Point> {
    points
        .into_iter()
        .filter(|&p| { point_area_is_limited_in_all_directions_by(p, points) })
        .collect()
}

pub fn point_area_is_limited_in_all_directions_by(test_point: &Point, points: &Vec<Point>) -> bool {
    let mut is_limited_left = false;
    let mut is_limited_right = false;
    let mut is_limited_up = false;
    let mut is_limited_down = false;

    for point in points {
        let is_limited_fn = |dir: AxisDirection| {
            is_point_area_limited_by(test_point, &point, dir)
        };

        if !is_limited_left {
            is_limited_left = is_limited_fn(AxisDirection::Left);
        }
        if !is_limited_right {
            is_limited_right = is_limited_fn(AxisDirection::Right);
        }
        if !is_limited_up {
            is_limited_up = is_limited_fn(AxisDirection::Up);
        }
        if !is_limited_down {
            is_limited_down = is_limited_fn(AxisDirection::Down);
        }

        if is_limited_left && is_limited_right && is_limited_up && is_limited_down {
            return true;
        }
    }

    false
}

fn is_point_area_limited_by(tested_point: &Point, limiting_point: &Point, direction: AxisDirection) -> bool {
    if tested_point == limiting_point {
        return false;
    }

    match direction {
        AxisDirection::Left => {
            let extreme_left_point = Point {x: MIN_INT, y: tested_point.y};
            if point_is_further_than(tested_point, limiting_point, extreme_left_point) {
                return true;
            }
        },
        AxisDirection::Right => {
            let extreme_right_point = Point {x: MAX_INT, y: tested_point.y};
            if point_is_further_than(tested_point, limiting_point, extreme_right_point) {
                return true;
            }
        },
        AxisDirection::Up => {
            let extreme_up_point = Point {x: tested_point.x, y: MIN_INT};
            if point_is_further_than(tested_point, limiting_point, extreme_up_point) {
                return true;
            }
        },
        AxisDirection::Down => {
            let extreme_down_point = Point {x: tested_point.x, y: MAX_INT};
            if point_is_further_than(tested_point, limiting_point, extreme_down_point) {
                return true;
            }
        }
    }

    false
}

fn point_is_further_than(from_test: &Point, from_cmp: &Point, to: Point) -> bool {
    let distance_test_point = manhattan_distance(from_test, &to);
    let distance_cmp_point = manhattan_distance(from_cmp, &to);

    distance_test_point >= distance_cmp_point
}
