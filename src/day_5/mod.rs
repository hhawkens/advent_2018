mod data;
mod utils;

pub fn result_part_one() -> usize {
    let reacted_polymer = utils::chain_react_polymer(data::POLYMER);
    reacted_polymer.chars().count()
}
