mod grid;

pub type Size = Point;
pub type PowerLevel = isize;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)] // TODO what can be removed?
pub struct Point {
    /// To the right
    pub x: usize,
    /// Downwards
    pub y: usize,
}

#[derive(Debug)]
pub struct Grid {
    size: Size,
    serial_number: usize,
    fuel_cells: Vec<PowerLevel>, // Flat representation of a logically 2D array
}
