use super::types::{AxisDirection, Point};

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

pub fn get_area_size_of_point(point: &Point, all_points: &[Point]) -> i32 {
    let mut area_point_count = 1; // 1 because the point itself belongs to its area

    for i in 1.. {
        let mut area_point_found = false;
        let surrounding_points = point.get_surrounding_points(i);
        for neighbour in surrounding_points {
            if is_point_in_area_of(point, &neighbour, all_points) {
                area_point_count += 1;
                area_point_found = true;
            }
        }

        if !area_point_found {
            break;
        }
    }

    area_point_count
}

pub fn get_limited_area_points(points: &[Point]) -> Vec<&Point> {
    points
        .iter()
        .filter(|&p| point_area_is_limited_in_all_directions_by(p, points))
        .collect()
}

fn is_point_in_area_of(area_point: &Point, point: &Point, all_area_points: &[Point]) -> bool {
    if area_point == point {
        return false;
    }

    let dist_area_point = manhattan_distance(area_point, point);

    for cmp_area_point in all_area_points {
        if cmp_area_point == point {
            return false;
        }
        if area_point == cmp_area_point {
            continue;
        }

        let dist_cmp_area_point = manhattan_distance(cmp_area_point, point);
        if dist_cmp_area_point <= dist_area_point {
            return false;
        }
    }

    true
}

fn point_area_is_limited_in_all_directions_by(test_point: &Point, points: &[Point]) -> bool {
    let mut is_limited_left = false;
    let mut is_limited_right = false;
    let mut is_limited_up = false;
    let mut is_limited_down = false;

    for point in points {
        let is_limited_fn = |dir: AxisDirection| is_point_area_limited_by(test_point, &point, dir);

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
            let extreme_left_point = Point {
                x: MIN_INT,
                y: tested_point.y,
            };
            if point_is_further_than(tested_point, limiting_point, extreme_left_point) {
                return true;
            }
        }
        AxisDirection::Right => {
            let extreme_right_point = Point {
                x: MAX_INT,
                y: tested_point.y,
            };
            if point_is_further_than(tested_point, limiting_point, extreme_right_point) {
                return true;
            }
        }
        AxisDirection::Up => {
            let extreme_up_point = Point {
                x: tested_point.x,
                y: MIN_INT,
            };
            if point_is_further_than(tested_point, limiting_point, extreme_up_point) {
                return true;
            }
        }
        AxisDirection::Down => {
            let extreme_down_point = Point {
                x: tested_point.x,
                y: MAX_INT,
            };
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

pub fn manhattan_distance(point_1: &Point, point_2: &Point) -> u64 {
    ((i64::from(point_1.x) - i64::from(point_2.x)).abs()
        + (i64::from(point_1.y) - i64::from(point_2.y)).abs()) as u64
}
