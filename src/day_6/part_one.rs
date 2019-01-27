use super::types::*;
use super::utils::*;
use std::collections::HashMap;

pub fn get_area_sizes_of_points<'a, 'b>(
    all_points: &'a [Point],
    limited_area_points: &'b [&'b Point],
) -> HashMap<&'b Point, i32> {
    limited_area_points
        .iter()
        .map(|&p| {
            let area_size = get_area_size_of_point(p, all_points);
            (p, area_size)
        })
        .collect()
}
