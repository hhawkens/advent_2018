mod types;
mod data;
mod utils;

pub fn result_part_one() -> usize {
    let license_tree = utils::get_license_tree(data::LICENSE);
    license_tree.meta_data_sum()
}
