pub fn frequency_string_to_array(freq: &str) -> Vec<i32> {
    let frequency = String::from(freq);
    let frequency_texts: Vec<&str> = frequency.split('\n').collect();
    let frequency_nums: Vec<i32> = frequency_texts
        .into_iter()
        .map(|f| { f.parse().unwrap() })
        .collect();

    frequency_nums
}
