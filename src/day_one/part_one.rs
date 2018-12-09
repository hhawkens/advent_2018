use super::utils;

pub fn get_result() -> i32 {
    utils::frequency_string_to_array(super::data::INPUT).into_iter().sum()
}
