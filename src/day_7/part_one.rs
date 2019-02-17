use crate::day_7::types::*;
use crate::day_7::utils;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};

pub fn get_tasks_order(tasks: &HashMap<TaskId, Deps>) -> String {
    let num_tasks = tasks.len();
    let mut executable_tasks = utils::get_executable_tasks::<BTreeSet<TaskId>>(&tasks);
    let mut done_tasks = HashMap::with_capacity(num_tasks);
    let mut counter = 0usize;

    while !executable_tasks.is_empty() {
        // Execute
        counter += 1;
        let &next_task_id_to_execute = executable_tasks.iter().nth(0).unwrap();
        executable_tasks.remove(&next_task_id_to_execute);
        done_tasks.insert(next_task_id_to_execute, counter);
        // Add new executable tasks
        for dependent_task_id in &tasks[&next_task_id_to_execute].dependency_of {
            let dependent_task = &tasks[&dependent_task_id];
            if utils::contains_all(&done_tasks, &dependent_task.depending_on) {
                executable_tasks.insert(*dependent_task_id);
            }
        }
    }

    done_tasks
        .iter()
        .sorted_by(|(_, i1), (_, i2)| i1.cmp(i2))
        .map(|(&id, _)| id)
        .collect()
}
