/// Point in a grid coordinate system
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Point {
    /// To the right
    pub x: i32,
    /// Downwards
    pub y: i32,
}

#[derive(Copy, Clone)]
pub enum AxisDirection {
    Left,
    Right,
    Up,
    Down
}
