mod data;
mod utils;

pub fn result_part_one() -> usize {
    let reacted_polymer = utils::react_polymer_parallel(data::POLYMER);
    reacted_polymer.chars().count()
}
