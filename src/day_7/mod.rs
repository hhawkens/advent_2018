mod data;
mod utils;
mod types;
mod part_one;

pub fn result_part_one() -> String {
    let tasks= utils::get_tasks(data::STEPS);
    part_one::get_tasks_order(&tasks)
}
