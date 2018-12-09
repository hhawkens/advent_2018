mod part_one;
mod part_two;
mod data;
mod utils;

pub fn result_part_one() -> i32 {
    part_one::get_final_frequency()
}

pub fn result_part_two() -> i32 {
    part_two::get_first_double_frequency()
}
