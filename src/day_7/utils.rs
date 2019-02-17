use std::collections::{HashMap, HashSet};
use super::types::*;

pub fn get_tasks(tasks_text: &str) -> HashMap<TaskId, Deps> {
    let mut all_tasks = HashMap::new();

    tasks_text
        .lines()
        .for_each(|t| {
            let task_id = t.chars().nth(5).unwrap();
            let dependent_task_id = t.chars().nth(36).unwrap();

            let mut task = all_tasks
                .entry(task_id)
                .or_insert(Deps {dependency_of: HashSet::new(), depending_on: HashSet::new()});
            task.dependency_of.insert(dependent_task_id);

            let mut dependent_task = all_tasks
                .entry(dependent_task_id)
                .or_insert(Deps {dependency_of: HashSet::new(), depending_on: HashSet::new()});
            dependent_task.depending_on.insert(task_id);
        });

    all_tasks
}
