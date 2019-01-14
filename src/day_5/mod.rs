mod data;
mod part_two;
mod utils;

use rayon::prelude::*;

pub fn result_part_one() -> usize {
    let reacted_polymer = utils::chain_react_polymer(data::POLYMER);
    reacted_polymer.chars().count()
}

pub fn result_part_two() -> usize {
    part_two::get_stripped_polymer_reacted_size(data::POLYMER, data::ALL_UNIT_TYPES.par_iter())
}
