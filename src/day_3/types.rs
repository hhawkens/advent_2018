#[derive(Debug)]
pub struct Rect {
    pub location: Point,
    pub size: Size,
}

#[derive(Debug)]
pub struct Point {
    /// To the right
    pub x: i32,
    /// Down
    pub y: i32,
}

#[derive(Debug)]
pub struct Size {
    /// Width
    pub w: i32,
    /// Height
    pub h: i32,
}
