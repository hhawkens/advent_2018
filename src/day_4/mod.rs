mod data;
mod part_one;
mod utils;
pub mod types; // TODO private

pub fn result_part_one() -> Vec<types::GuardEvent> {
    utils::get_guard_events()
}
