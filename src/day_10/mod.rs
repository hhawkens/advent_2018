use crate::day_10::types::*;

mod data;
mod types;
mod utils;

pub fn result_part_one() -> String {
    let stars = utils::get_stars(data::STARS);
    let mut sky = Sky { stars };

    while !sky.are_stars_aligned() {
        sky.move_stars_one_step();
    }

    sky.to_string()
}
