mod data;
mod part_one;

pub fn result_part_one() -> usize {
    let reacted_polymer = part_one::react_polymer_parallel(data::POLYMER);
    reacted_polymer.chars().count()
}
