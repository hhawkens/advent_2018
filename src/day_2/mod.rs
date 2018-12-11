mod data;
mod part_one;
mod part_two;
mod utils;

pub fn result_part_one() -> i32 {
    part_one::get_box_id_checksum()
}

pub fn result_part_two() -> String {
    part_two::get_common_chars_of_twin_boxes()
}
