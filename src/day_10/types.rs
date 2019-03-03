use std::ops::Add;

pub type Velocity = Point;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Point {
    /// To the right
    pub x: isize,
    /// Downwards
    pub y: isize,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Star {
    pub location: Point,
    pub velocity: Velocity,
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}


impl Star {
    /// Moves the location of this star one step forward, according to its velocity
    pub fn move_one_step(&mut self) {
        self.location.x += self.velocity.x;
        self.location.y += self.velocity.y;
    }
}
