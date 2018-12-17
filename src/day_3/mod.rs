mod data;
mod part_one;
pub mod part_two; // TODO private
pub mod types; // TODO private
mod utils;

pub fn result_part_one() -> i32 {
    part_one::multiple_claims_count()
}
