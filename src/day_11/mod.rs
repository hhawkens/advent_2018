mod data;
mod types;

use self::types::*;

pub fn result_part_one() -> Point {
    let grid = Grid::new(Size {x: 300, y: 300}, data::GRID_SERIAL);
    let (largest_power_location, _) = grid.find_largest_total_power(Size { x: 3, y: 3 });
    largest_power_location
}

pub fn result_part_two() -> (Point, usize) {
    let grid = Grid::new(Size {x: 300, y: 300}, data::GRID_SERIAL);
    let (location, _, sub_grid_size) = grid.find_largest_total_power_of_any_sub_grid_size();
    (location, sub_grid_size)
}
