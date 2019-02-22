use super::types::*;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FromIterator;

pub fn get_tasks(tasks_text: &str) -> HashMap<TaskId, Deps> {
    let mut all_tasks = HashMap::new();

    tasks_text.lines().for_each(|t| {
        let task_id = t.chars().nth(5).unwrap();
        let dependent_task_id = t.chars().nth(36).unwrap();

        let task = all_tasks.entry(task_id).or_insert(Deps {
            dependency_of: HashSet::new(),
            depending_on: HashSet::new(),
        });
        task.dependency_of.insert(dependent_task_id);

        let dependent_task = all_tasks.entry(dependent_task_id).or_insert(Deps {
            dependency_of: HashSet::new(),
            depending_on: HashSet::new(),
        });
        dependent_task.depending_on.insert(task_id);
    });

    all_tasks
}

pub fn get_executable_tasks<T: FromIterator<TaskId>>(tasks: &HashMap<TaskId, Deps>) -> T {
    tasks
        .iter()
        .filter(|(_, deps)| deps.depending_on.is_empty())
        .map(|(&id, _)| id)
        .collect()
}

pub fn hashmap_contains_all<'a, K: 'a + Eq + Hash, V>(
    map: &HashMap<K, V>,
    keys: impl IntoIterator<Item = &'a K>,
) -> bool {
    for key in keys {
        if !map.contains_key(key) {
            return false;
        }
    }
    true
}

pub fn hashset_contains_all<'a, K: 'a + Eq + Hash>(
    set: &HashSet<K>,
    keys: impl IntoIterator<Item = &'a K>,
) -> bool {
    for key in keys {
        if !set.contains(key) {
            return false;
        }
    }
    true
}
