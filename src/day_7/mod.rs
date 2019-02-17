mod data;
mod utils;
mod types;
mod part_one;

pub fn result_part_one() -> usize {
    data::STEPS.lines().count()
}
