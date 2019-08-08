/// Point in a grid coordinate system
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Point {
    /// To the right
    pub x: i32,
    /// Downwards
    pub y: i32,
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
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

        surrounding_points.push(Point { x: self.x + offset, y: self.y + offset }); // Starting at bottom right

        for _ in 0..double_offset {
            let last_point = surrounding_points.last().unwrap();
            let bottom_left = Point { x: last_point.x - 1, y: last_point.y };
            surrounding_points.push(bottom_left);
        }

        for _ in 0..double_offset {
            let last_point = surrounding_points.last().unwrap();
            let upper_left = Point { x: last_point.x, y: last_point.y - 1 };
            surrounding_points.push(upper_left);
        }

        for _ in 0..double_offset {
            let last_point = surrounding_points.last().unwrap();
            let upper_right = Point { x: last_point.x + 1, y: last_point.y };
            surrounding_points.push(upper_right);
        }

        for _ in 0..double_offset - 1 {
            let last_point = surrounding_points.last().unwrap();
            let bottom_right = Point { x: last_point.x, y: last_point.y + 1 };
            surrounding_points.push(bottom_right);
        }

        surrounding_points
    }
}
