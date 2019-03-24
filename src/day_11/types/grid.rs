use super::*;
use std::cmp::min;
use rayon::prelude::*;

impl Grid {
    /// Creates new grid with given size
    pub fn new(size: Size, serial_number: usize) -> Grid {
        let mut fuel_cells = Vec::with_capacity(size.x * size.y);

        for y in 1..=size.y {
            for x in 1..=size.x {
                let location = Point { x, y };
                let power_level = Self::calculate_cell_power_level(location, serial_number);
                fuel_cells.push(power_level);
            }
        }

        Grid { size, serial_number, fuel_cells, }
    }

    pub fn find_largest_total_power(&self, sub_grid_size: Size) -> (Point, PowerLevel) {
        let power_summed_area_table = self.create_summed_area_table();
        self.lookup_largest_total_power(sub_grid_size, &power_summed_area_table)
    }

    pub fn find_largest_total_power_of_any_sub_grid_size(&self) -> (Point, PowerLevel, usize) {
        let power_summed_area_table = self.create_summed_area_table();
        let max_sub_grid_size = min(self.size.x, self.size.y);

        (1..max_sub_grid_size + 1)
            .into_par_iter()
            .map(|size| {
                let sub_grid_size = Size {x: size, y: size};
                let (largest_power_point, largest_power) =
                    self.lookup_largest_total_power(sub_grid_size, &power_summed_area_table);
                (largest_power_point, largest_power, size)
            })
            .reduce(|| (Point {x: 0, y: 0}, std::isize::MIN, 0), |acc, curr| {
                let (_, max_power, _) = acc;
                let (_, curr_power, _) = curr;
                if curr_power > max_power {
                    curr
                } else {
                    acc
                }
            })
    }

    fn lookup_largest_total_power(&self, sub_grid_size: Size, power_table: &Vec<PowerLevel>) -> (Point, PowerLevel) {
        let (mut max_point, mut max_power_level) = (Point{x: 0, y: 0}, std::isize::MIN as isize);

        for y in 1..=self.size.y - sub_grid_size.y + 1 {
            for x in 1..=self.size.x - sub_grid_size.x + 1 {
                let table_x = x + sub_grid_size.x - 1;
                let table_y = y + sub_grid_size.y - 1;
                let table_index = self.get_array_2d_flat_index(table_x, table_y);
                let mut power_level = power_table[table_index];

                if x > 1 && y > 1 {
                    power_level -= power_table[self.get_array_2d_flat_index(table_x - sub_grid_size.x, table_y)];
                    power_level -= power_table[self.get_array_2d_flat_index(table_x, table_y - sub_grid_size.y)];
                    power_level += power_table[self.get_array_2d_flat_index(table_x - sub_grid_size.x, table_y - sub_grid_size.y)];
                } else if x > 1 {
                    power_level -= power_table[self.get_array_2d_flat_index(table_x - sub_grid_size.x, table_y)];
                } else if y > 1 {
                    power_level -= power_table[self.get_array_2d_flat_index(table_x, table_y - sub_grid_size.y)];
                }

                if power_level > max_power_level {
                    max_point = Point {x, y};
                    max_power_level = power_level;
                }
            }
        }

        (max_point, max_power_level)
    }

    fn create_summed_area_table(&self) -> Vec<PowerLevel> {
        let mut area_table = Vec::with_capacity(self.size.x * self.size.y);

        for y in 1..=self.size.y {
            for x in 1..=self.size.x {
                let power_level = self.get_power_level_at_point(Point{x, y});

                if x > 1 && y > 1 {
                    let area_above_value = area_table[self.get_array_2d_flat_index(x, y - 1)];
                    let area_left_value = area_table[self.get_array_2d_flat_index(x - 1, y)];
                    let area_left_above_value = area_table[self.get_array_2d_flat_index(x - 1, y - 1)];
                    area_table.push(power_level + area_above_value + area_left_value - area_left_above_value);
                } else if x > 1 {
                    let area_left_value = area_table[self.get_array_2d_flat_index(x - 1, y)];
                    area_table.push(power_level + area_left_value);
                } else if y > 1 {
                    let area_above_value = area_table[self.get_array_2d_flat_index(x, y - 1)];
                    area_table.push(power_level + area_above_value);
                } else {
                    area_table.push(power_level)
                }
            }
        }

        area_table
    }

    fn get_power_level_at_point(&self, point: Point) -> PowerLevel {
        let index = self.get_array_2d_flat_index(point.x, point.y);
        self.fuel_cells[index]
    }

    fn get_array_2d_flat_index(&self, x: usize, y: usize) -> usize {
        self.size.x * (y - 1) + (x - 1)
    }

    fn calculate_cell_power_level(location: Point, serial_number: usize) -> PowerLevel {
        let rack_id = location.x + 10;
        let mut power_level: isize = (rack_id * location.y) as isize;
        power_level += serial_number as isize;
        power_level *= rack_id as isize;
        power_level = power_level / 100 % 10;
        power_level -= 5;

        power_level
    }
}
