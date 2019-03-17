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
                let cell = FuelCell { location, power_level };
                fuel_cells.insert(cell.location, cell);
            }
        }

        Grid { fuel_cells, serial_number }
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
