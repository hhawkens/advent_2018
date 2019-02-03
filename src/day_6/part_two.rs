use super::types::*;
use std::collections::HashMap;
use std::cmp;

pub fn calculate_centre_point(points: &[Point]) -> Point {
    let extremes = get_all_direction_extremes(points);
    let x_centre = extremes[&AxisDirection::Left] + extremes[&AxisDirection::Right] / 2;
    let y_centre = extremes[&AxisDirection::Up] + extremes[&AxisDirection::Down] / 2;
    Point {x: x_centre, y: y_centre}
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
        let new_extreme = get_extreme(point, direction, &maybe_extreme);
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

fn get_extreme(point: &Point, direction: AxisDirection, comp_val: &Option<i32>) -> i32 {
    match direction {
        AxisDirection::Left => {
            match comp_val {
                None => return point.x,
                Some(x) => return cmp::min(*x, point.x),
            }
        },
        AxisDirection::Right => {
            match comp_val {
                None => return point.x,
                Some(x) => return cmp::max(*x, point.x),
            }
        },
        AxisDirection::Up => {
            match comp_val {
                None => return point.y,
                Some(y) => return cmp::min(*y, point.y),
            }
        },
        AxisDirection::Down => {
            match comp_val {
                None => return point.y,
                Some(y) => return cmp::max(*y, point.y),
            }
        },
    }
}
