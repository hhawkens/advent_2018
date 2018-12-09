use super::utils;
use stopwatch::{Stopwatch};

pub fn get_first_double_frequency() -> i32 {
    let mut current_result = 0;
    let mut results = vec![0];
    let frequencies = utils::frequency_string_to_array(super::data::INPUT).into_iter().cycle();

    let sw = Stopwatch::start_new();
    for freq in frequencies {
        current_result += freq;
        if results.contains(&current_result) {
            println!("{}", sw.elapsed_ms());
            return current_result;
        } else {
            results.push(current_result);
        }
    }
    0
}
