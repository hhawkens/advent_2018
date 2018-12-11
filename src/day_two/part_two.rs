pub fn get_common_chars_of_twin_boxes() -> String {
    let twins = get_twin_boxes();
    get_common_chars(&twins.0, &twins.1)
}

fn get_twin_boxes() -> (String, String) {
    let box_ids = super::utils::get_box_ids();
    let mut result = (String::new(), String::new());

    for i in 0..box_ids.len() - 2 {
        let first = &box_ids[i];
        for j in i + 1..box_ids.len() - 1 {
            let second = &box_ids[j];
            if are_equal_bar_one(first, second) {
                result = (first.clone(), second.clone())
            }
        }
    }

    result
}

fn are_equal_bar_one(first: &String, second: &String) -> bool {
    let mut counter = 0;
    first.chars().zip(second.chars()).for_each(|char_pair| {
        if char_pair.0 != char_pair.1 {
            counter += 1;
        }
    });

    counter == 1
}

fn get_common_chars(first: &String, second: &String) -> String {
    first
        .chars()
        .zip(second.chars())
        .filter(|char_pair| char_pair.0 == char_pair.1)
        .map(|char_pair| char_pair.0)
        .collect()
}
