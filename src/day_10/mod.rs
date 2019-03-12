use crate::day_10::types::*;

mod data;
mod types;
mod utils;

pub fn result() -> (String, usize) {
    let stars = utils::get_stars(data::STARS);
    let mut sky = Sky { stars };
    let mut waited_seconds = 0usize;

    while !sky.are_stars_aligned() {
        waited_seconds += 1;
        sky.move_stars_one_step();
    }

    (sky.to_string(), waited_seconds)
}
