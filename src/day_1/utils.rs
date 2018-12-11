pub fn frequency_string_to_array(freq: &str) -> Vec<i32> {
    let frequency_full_text = String::from(freq);
    let frequencies: Vec<i32> = frequency_full_text
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    frequencies
}
