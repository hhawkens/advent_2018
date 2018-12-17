mod data;
mod part_one;
mod part_two;
mod types;
mod utils;

pub fn result_part_one() -> i32 {
    part_one::multiple_claims_count()
}

pub fn result_part_two() -> i32 {
    part_two::find_claim_id_without_overlap()
}
