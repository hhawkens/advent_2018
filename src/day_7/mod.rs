mod data;
mod part_one;
mod part_two;
mod types;
mod utils;

pub fn result_part_one() -> String {
    let tasks = utils::get_tasks(data::STEPS);
    part_one::get_tasks_order(&tasks)
}

pub fn result_part_two() -> usize {
    0
}
