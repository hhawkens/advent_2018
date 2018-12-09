use super::utils;
use std::collections::HashSet;

pub fn get_first_double_frequency() -> i32 {
    let mut current_result = 0;
    let mut results = HashSet::new();
    let frequencies = utils::frequency_string_to_array(super::data::INPUT).into_iter().cycle();

    for freq in frequencies {
        current_result += freq;
        if results.contains(&current_result) {
            return current_result;
        } else {
            results.insert(current_result);
        }
    }
    0
}
