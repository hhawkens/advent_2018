use super::utils::Polymer;
use super::utils::chain_react_polymer;
use rayon::prelude::ParallelIterator;

pub fn get_stripped_polymer_reacted_size<'a>(polymer: &str, unit_types: impl ParallelIterator<Item=&'a char>) -> usize {
    unit_types
        .map(|&unit_type| {
            let stripped_polymer = remove_unit_type_from_polymer(unit_type, polymer);
            let reacted_stripped_polymer = chain_react_polymer(&stripped_polymer);
            reacted_stripped_polymer.len()
        })
        .min()
        .unwrap_or(0)
}

fn remove_unit_type_from_polymer(unit_type: char, polymer: &str) -> Polymer {
    let unit_type_other_polarity = unit_type.to_uppercase().nth(0).unwrap();
    polymer.replace(unit_type, "").replace(unit_type_other_polarity, "")
}
