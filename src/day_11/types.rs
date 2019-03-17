mod grid;

use std::collections::HashMap;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    /// To the right
    pub x: usize,
    /// Downwards
    pub y: usize,
}

#[derive(Debug)]
pub struct FuelCell {
    pub location: Point,
    pub power_level: isize,
}

#[derive(Debug)]
pub struct Grid {
    pub fuel_cells: HashMap<Point, FuelCell>,
    serial_number: usize,
}
