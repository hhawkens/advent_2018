use super::types::Point;
use super::data::COORDINATES;

pub fn get_coordinates() -> Vec<Point> {
    let mut coordinates = vec![];

    COORDINATES
        .lines()
        .for_each(|line| {
            let mut points = line.split(", ").map(|p| {
                p.parse::<i32>().unwrap()
            });
            coordinates.push(Point { x: points.next().unwrap(), y: points.next().unwrap() });
        });

    coordinates
}
