mod data;
mod part_one;
pub mod types;
mod utils; // TODO private

pub fn result_part_one() -> i32 {
    part_one::get_guard_with_most_sleep_time()
}
