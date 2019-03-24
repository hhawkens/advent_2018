mod data;
mod types;

use self::types::*;

pub fn result_part_one() -> Point {
    let grid = Grid::new(Size {x: 300, y: 300}, data::GRID_SERIAL);
    let (largest_power_location, _) = grid.find_largest_total_power(Size { x: 3, y: 3 });
    largest_power_location
}
