use std::ops::Add;
use std::fmt::{Debug, Formatter, Error};
use num_traits::abs;
use ref_eq::ref_eq;
use std::collections::HashSet;

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

#[derive(Eq, PartialEq, Hash)]
pub struct Sky {
    pub stars: Vec<Star>
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

    /// Check if two stars are next to each other
    pub fn are_adjacent(s1: &Star, s2: &Star) -> bool {
        let diff_x = abs(s1.location.x - s2.location.x);
        let diff_y = abs(s1.location.y - s2.location.y);
        diff_x <= 1 && diff_y <= 1
    }
}

impl Debug for Sky {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&self.to_string())
    }
}

impl Sky {
    /// Custom debug string
    pub fn to_string(&self) -> String {
        let (left_extr, right_extr, up_extr, down_extr) = self.get_extremes();
        let vertical_capacity = (down_extr - up_extr + 1) as usize;
        let horizontal_capacity = (right_extr - left_extr + 1) as usize;

        (0..vertical_capacity)
            .map(|line_number| {
                let relevant_stars = self.stars.iter()
                    .filter(|&s| s.location.y == line_number as isize + up_extr)
                    .collect::<Vec<_>>();
                let mut line = String::with_capacity(horizontal_capacity + 1); // +1 for the line break

                for i in 0..line.capacity() as isize {
                    if relevant_stars.iter().find(|&&s| s.location.x == i + left_extr ).is_some() {
                        line.push_str("#");
                    } else if i == line.capacity() as isize - 1 {
                        line.push_str("\n");
                    } else {
                        line.push_str(".");
                    }
                }

                line
            })
            .collect::<String>()
    }

    /// Get the leftmost, rightmost, uppermost and lowermost point values
    pub fn get_extremes(&self) -> (isize, isize, isize, isize) {
        let mut left = std::isize::MAX;
        let mut right = std::isize::MIN;
        let mut up = std::isize::MAX;
        let mut down = std::isize::MIN;

        for star in &self.stars {
            let x = star.location.x;
            let y = star.location.y;

            if x < left {
                left = x;
            }
            if x > right {
                right = x;
            }
            if y < up {
                up = y;
            }
            if y > down {
                down = y;
            }
        }

        (left, right, up, down)
    }

    /// Advances star positions according to their speed
    pub fn move_stars_one_step(&mut self) {
        for star in &mut self.stars {
            star.move_one_step();
        }
    }

    /// Check if all stars have at least one direct neighbour
    pub fn are_stars_aligned(&self) -> bool {
        let (left_extr, right_extr, up_extr, down_extr) = self.get_extremes();
        let diff_x = abs(right_extr - left_extr) as usize;
        let diff_y = abs(down_extr - up_extr) as usize;
        let dist = self.stars.len() / 6; // heuristic

        if diff_x > dist && diff_y > dist {
            false
        } else {
            let mut stars_with_no_neighbours = self.stars.iter().collect::<HashSet<_>>();

            for star in &self.stars {
                if !stars_with_no_neighbours.contains(star) {
                    continue;
                }
                for inner_star in &self.stars {
                    if !ref_eq(star, inner_star) && Star::are_adjacent(star, inner_star) {
                        stars_with_no_neighbours.remove(star);
                        stars_with_no_neighbours.remove(inner_star);
                        break;
                    }
                }
            }

            stars_with_no_neighbours.len() == 0
        }
    }
}
