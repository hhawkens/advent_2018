use num_traits::{Num};

pub fn react_polymer(polymer: &str) -> String {
    let mut reacted_polymer = polymer.to_string();
    let mut indices_to_remove = Vec::new();
    let mut all_reactions_finished = false;

    while !all_reactions_finished {
        let last_index = reacted_polymer.chars().count() - 1;
        let mut skip_one_turn = false;
        for char_index in 0..last_index {
            if skip_one_turn {
                skip_one_turn = false;
                continue;
            }

            let first = reacted_polymer.chars().nth(char_index).unwrap();
            let second = reacted_polymer.chars().nth(char_index + 1).unwrap();
            if do_polymer_parts_react(&first, &second) {
                indices_to_remove.push(char_index);
                indices_to_remove.push(char_index + 1);
                skip_one_turn = true;
            }
        }

        if indices_to_remove.is_empty() {
            all_reactions_finished = true;
            break;
        }

        indices_to_remove.reverse();
        for &index_to_remove in &indices_to_remove {
            reacted_polymer.remove(index_to_remove);
        }
        indices_to_remove.clear();
    }

    reacted_polymer
}

fn do_polymer_parts_react(first: &char, second: &char) -> bool {
    let are_same = first == second;
    let are_equal = first.to_lowercase().to_string() == second.to_lowercase().to_string();

    are_equal && !are_same
}
