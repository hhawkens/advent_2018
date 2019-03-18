use super::*;
use std::collections::HashMap;

impl Grid {
    /// Creates new grid with given size
    pub fn new(horizontal_size: usize, vertical_size: usize, serial_number: usize) -> Grid {
        let mut fuel_cells = HashMap::with_capacity(horizontal_size * vertical_size);

        for x in 1..=horizontal_size {
            for y in 1..=vertical_size {
                let location = Point { x, y };
                let power_level = get_cell_power_level(location, serial_number);
                let cell = FuelCell {
                    location,
                    power_level,
                };
                fuel_cells.insert(cell.location, cell);
            }
        }

        Grid {
            fuel_cells,
            serial_number,
            size: Size {
                x: horizontal_size,
                y: vertical_size,
            },
        }
    }

    pub fn get_largest_power_level_in_sub_grid(&self, sub_grid_size: Size) -> (Point, isize) {
        let mut largest_power_level: Option<(Point, isize)> = None;

        for x in 1..=self.size.x - sub_grid_size.x {
            for y in 1..=self.size.y - sub_grid_size.y {
                let upper_left_corner = Point { x, y };
                let power_level = self.get_sub_grid_power_level(upper_left_corner, sub_grid_size);
                match largest_power_level {
                    None => largest_power_level = Some((upper_left_corner, power_level)),
                    Some((_, curr_largest)) => {
                        if power_level > curr_largest {
                            largest_power_level = Some((upper_left_corner, power_level))
                        }
                    }
                }
            }
        }

        match largest_power_level {
            None => (Point { x: 0, y: 0 }, 0),
            Some(x) => x,
        }
    }

    /// Calculate the power level of given x*y sub grid
    pub fn get_sub_grid_power_level(&self, upper_left_corner: Point, sub_grid_size: Size) -> isize {
        if upper_left_corner.x + sub_grid_size.x > self.size.x + 1
            || upper_left_corner.y + sub_grid_size.y > self.size.y + 1
        {
            panic!("Too much");
        }

        let mut accumulated_power_level = 0;
        for x in upper_left_corner.x..upper_left_corner.x + sub_grid_size.x {
            for y in upper_left_corner.y..upper_left_corner.y + sub_grid_size.y {
                accumulated_power_level += self.fuel_cells[&Point { x, y }].power_level;
            }
        }
        accumulated_power_level
    }
}

fn get_cell_power_level(location: Point, serial_number: usize) -> isize {
    let rack_id = location.x + 10;
    let mut power_level: isize = (rack_id * location.y) as isize;
    power_level += serial_number as isize;
    power_level *= rack_id as isize;
    power_level = power_level / 100 % 10;
    power_level -= 5;

    power_level
}
