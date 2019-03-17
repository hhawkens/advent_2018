use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Debug)]
struct FuelCell {
    pub location: Point,
    pub power_level: Option<isize>,
}

#[derive(Debug)]
struct Grid {
    pub cells: HashMap<Point, FuelCell>,
}
