use std::collections::HashMap;

pub fn get_tasks(tasks_text: &str) -> HashMap<char, Vec<char>> {
    let mut tasks = HashMap::new();

    tasks_text
        .lines()
        .for_each(|t| {
            let dependency_task = t.chars().nth(5).unwrap();
            let dependent_task = t.chars().nth(36).unwrap();
            tasks.entry(dependency_task).or_insert(vec![]).push(dependent_task);
        });

    tasks
}
