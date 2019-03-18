mod data;
mod types;

use self::types::*;

pub fn result_part_one() -> Point {
    let grid = Grid::new(300, 300, data::GRID_SERIAL);
    let largest_3_x_3_power_level = grid.get_largest_power_level_in_sub_grid(Size { x: 3, y: 3 });
    largest_3_x_3_power_level.0
}
