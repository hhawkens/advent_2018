/// Point in a grid coordinate system
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
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
    Down,
}

impl Point {
    pub fn get_surrounding_points(&self, offset: i32) -> Vec<Point> {
        if offset <= 0 {
            return Vec::new();
        }

        let double_offset = offset * 2;
        let mut surrounding_points = Vec::new();

        surrounding_points.push(Point {
            x: self.x + offset,
            y: self.y + offset,
        }); // Starting at bottom right

        for _ in 0..double_offset {
            // Moving left to the bottom left
            let last_point = surrounding_points.last().unwrap();
            surrounding_points.push(Point {
                x: last_point.x - 1,
                y: last_point.y,
            });
        }

        for _ in 0..double_offset {
            // Moving up to the upper left
            let last_point = surrounding_points.last().unwrap();
            surrounding_points.push(Point {
                x: last_point.x,
                y: last_point.y - 1,
            });
        }

        for _ in 0..double_offset {
            // Moving right to the upper right
            let last_point = surrounding_points.last().unwrap();
            surrounding_points.push(Point {
                x: last_point.x + 1,
                y: last_point.y,
            });
        }

        for _ in 0..double_offset - 1 {
            // Moving down to the bottom right (where we started)
            let last_point = surrounding_points.last().unwrap();
            surrounding_points.push(Point {
                x: last_point.x,
                y: last_point.y + 1,
            });
        }

        surrounding_points
    }
}
