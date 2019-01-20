use super::types::{Point, PointAxis};
use itertools::Itertools;
use num_traits::real::Real;
use std::collections::HashSet;

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

pub fn manhattan_distance(point_1: &Point, point_2: &Point) -> i32 {
    (point_1.x - point_2.x).abs() + (point_1.y - point_2.y).abs()
}

pub fn get_extreme_points<'a>(points: impl IntoIterator<Item = &'a Point>) -> HashSet<&'a Point> {
    let mut left_extremes = (i32::max_value(), vec![]);
    let mut right_extremes = (i32::min_value(), vec![]);
    let mut up_extremes = (i32::max_value(), vec![]);
    let mut down_extremes = (i32::min_value(), vec![]);
    let smaller_than = |i1: i32, i2: i32| i1 < i2;
    let bigger_than = |i1: i32, i2: i32| i1 > i2;

    for point in points {
        add_point_if_extreme(point, &mut left_extremes, PointAxis::X, smaller_than);
        add_point_if_extreme(point, &mut right_extremes, PointAxis::X, bigger_than);
        add_point_if_extreme(point, &mut up_extremes, PointAxis::Y, smaller_than);
        add_point_if_extreme(point, &mut down_extremes, PointAxis::Y, bigger_than);
    }

    left_extremes
        .1
        .iter()
        .chain(&right_extremes.1)
        .chain(&up_extremes.1)
        .chain(&down_extremes.1)
        .dedup()
        .cloned()
        .collect()
}

fn add_point_if_extreme<'a>(
    point: &'a Point,
    (curr_extreme, extreme_points): &mut (i32, Vec<&'a Point>),
    axis: PointAxis,
    compare_fn: impl Fn(i32, i32) -> bool,
) {
    let point_extreme = match axis {
        PointAxis::X => point.x,
        PointAxis::Y => point.y,
    };

    if point_extreme == *curr_extreme {
        extreme_points.push(point);
    } else if compare_fn(point_extreme, *curr_extreme) {
        *curr_extreme = point_extreme;
        extreme_points.clear();
        extreme_points.push(point);
    }
}
