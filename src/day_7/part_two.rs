use crate::day_7::types::*;
use crate::day_7::utils;
use std::collections::{HashMap, HashSet};

pub fn get_tasks_completion_time(
    tasks: &HashMap<TaskId, Deps>,
    task_base_duration: usize,
    worker_count: usize,
) -> usize {
    let duration_per_task = duration_per_task(task_base_duration);
    let mut worker_pool = WorkerPool::new(worker_count);
    let mut ready_to_execute_tasks = utils::get_executable_tasks::<HashSet<TaskId>>(&tasks);
    let mut finished_tasks = HashSet::with_capacity(tasks.len());
    let mut current_second = 0_usize;

    while finished_tasks.len() < tasks.len() {
        for ready_task in &ready_to_execute_tasks {
            worker_pool.try_assign_task(*ready_task, duration_per_task[ready_task]);
        }
        for in_progress_task in worker_pool.in_progress_tasks() {
            ready_to_execute_tasks.remove(&in_progress_task);
        }

        let tasks_finished_this_tick = worker_pool.tick();
        for just_finished_task_id in tasks_finished_this_tick {
            finished_tasks.insert(just_finished_task_id);
            // Find new ready tasks
            for new_ready_task_id in &tasks[&just_finished_task_id].dependency_of {
                if utils::hashset_contains_all(
                    &finished_tasks,
                    &tasks[&new_ready_task_id].depending_on,
                ) {
                    ready_to_execute_tasks.insert(*new_ready_task_id);
                }
            }
        }

        current_second += 1;
    }

    current_second
}

pub fn duration_per_task(base_duration: TaskDuration) -> HashMap<TaskId, TaskDuration> {
    let mut task_durations = HashMap::with_capacity(26);
    let mut duration = base_duration;

    for task in b'A'..=b'Z' {
        duration += 1;
        task_durations.insert(task as TaskId, duration);
    }

    task_durations
}
