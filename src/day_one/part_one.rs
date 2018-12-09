use super::utils;

pub fn get_final_frequency() -> i32 {
    utils::frequency_string_to_array(super::data::INPUT).into_iter().sum()
}
