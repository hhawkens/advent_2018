use self::types::MetaDataEntry;

mod types;
mod data;
mod utils;

pub fn result_part_one() -> MetaDataEntry {
    let license_tree = utils::get_license_tree(data::LICENSE);
    license_tree.meta_data_sum()
}

pub fn result_part_two() -> MetaDataEntry {
    let license_tree = utils::get_license_tree(data::LICENSE);
    license_tree.root_node_value()
}
