use std::collections::HashMap;

pub fn get_box_id_checksum() -> i32 {
    let box_ids = super::utils::get_box_ids();
    let mut two_counts = 0;
    let mut three_counts = 0;

    for id in box_ids {
        if has_identical_letters(&id, 2) {
            two_counts += 1;
        }
        if has_identical_letters(&id, 3) {
            three_counts += 1;
        }
    }
    two_counts * three_counts
}

fn has_identical_letters(word: &str, num_identical_letters: i32) -> bool {
    let mut letter_count_map = HashMap::new();

    for c in word.chars() {
        *letter_count_map.entry(c).or_insert(0) += 1;
    }
    letter_count_map.values().any(|count| *count == num_identical_letters)
}
