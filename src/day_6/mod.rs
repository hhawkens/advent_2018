mod data;
mod part_one;
mod types;
mod utils;

pub fn result_part_one() -> i32 {
    let coordinates = utils::get_coordinates(data::COORDINATES);
    let limited_points = utils::get_limited_area_points(&coordinates);
    let point_area_sizes = part_one::get_area_sizes_of_points(&coordinates, &limited_points);

    point_area_sizes
        .iter()
        .map(|(_, &size)| size)
        .max()
        .unwrap_or(-1)
}
