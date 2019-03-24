use super::types::*;
use super::utils::*;
use std::cmp;
use std::collections::HashMap;

pub fn get_area_with_total_distance_less_than(points: &[Point], dist: u64) -> Vec<Point> {
    let mut area = Vec::new();
    let centre_point = calculate_centre_point(points);

    if get_total_distance_to_points(&centre_point, points) >= dist {
        return area;
    } else {
        area.push(centre_point.clone());
    }

    for offset in 1.. {
        let initial_area_size = area.len();
        let surrounding_points = centre_point.get_surrounding_points(offset);
        for neighbour in surrounding_points {
            if get_total_distance_to_points(&neighbour, points) < dist {
                area.push(neighbour);
            }
        }

        if initial_area_size == area.len() {
            break;
        }
    }

    area
}

fn get_total_distance_to_points(point: &Point, points: &[Point]) -> u64 {
    points.iter().map(|p| manhattan_distance(point, p)).sum()
}

fn calculate_centre_point(points: &[Point]) -> Point {
    let extremes = get_all_direction_extremes(points);
    let x_centre = extremes[&AxisDirection::Left] + extremes[&AxisDirection::Right] / 2;
    let y_centre = extremes[&AxisDirection::Up] + extremes[&AxisDirection::Down] / 2;
    Point { x: x_centre, y: y_centre }
}

fn get_all_direction_extremes(points: &[Point]) -> HashMap<AxisDirection, i32> {
    let mut extremes = HashMap::new();
    extremes.insert(AxisDirection::Left, get_extreme_for_direction(points, AxisDirection::Left));
    extremes.insert(AxisDirection::Right, get_extreme_for_direction(points, AxisDirection::Right));
    extremes.insert(AxisDirection::Up, get_extreme_for_direction(points, AxisDirection::Up));
    extremes.insert(AxisDirection::Down, get_extreme_for_direction(points, AxisDirection::Down));

    extremes
}

fn get_extreme_for_direction(points: &[Point], direction: AxisDirection) -> i32 {
    let mut maybe_extreme: Option<i32> = None;

    for point in points {
        let new_extreme = get_extreme(point, direction, maybe_extreme);
        match maybe_extreme {
            None => maybe_extreme = Some(new_extreme),
            Some(extreme) => {
                if new_extreme != extreme {
                    maybe_extreme = Some(new_extreme);
                }
            }
        }
    }

    maybe_extreme.unwrap_or(0)
}

fn get_extreme(point: &Point, direction: AxisDirection, comp_val: Option<i32>) -> i32 {
    match direction {
        AxisDirection::Left => match comp_val {
            None => point.x,
            Some(x) => cmp::min(x, point.x),
        },
        AxisDirection::Right => match comp_val {
            None => point.x,
            Some(x) => cmp::max(x, point.x),
        },
        AxisDirection::Up => match comp_val {
            None => point.y,
            Some(y) => cmp::min(y, point.y),
        },
        AxisDirection::Down => match comp_val {
            None => point.y,
            Some(y) => cmp::max(y, point.y),
        },
    }
}
