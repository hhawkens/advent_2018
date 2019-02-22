use self::WorkerState::*;
use std::collections::HashSet;

pub type TaskId = char;
pub type TaskDuration = usize;
pub type FinishedTasks = Vec<TaskId>;

/// Describes the dependencies from and to a task
#[derive(Debug, Eq, PartialEq)]
pub struct Deps {
    pub dependency_of: HashSet<TaskId>,
    pub depending_on: HashSet<TaskId>,
}

#[derive(Debug)]
pub struct WorkerPool {
    workers: Vec<Worker>,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Worker {
    id: usize,
    pub state: WorkerState,
    pub worked_for_seconds: TaskDuration,
}

#[derive(Debug, Eq, PartialEq)]
pub enum WorkerState {
    Free,
    Busy(TaskId, TaskDuration),
}

/// --- WORKER --- ///
impl Worker {
    /// Ends the worker's currently assigned task
    pub fn set_free(&mut self) {
        self.state = Free;
        self.worked_for_seconds = 0;
    }
}

/// --- WORKER POOL --- ///
impl WorkerPool {
    /// Creates new instance with n workers
    pub fn new(num_workers: usize) -> WorkerPool {
        let mut pool = WorkerPool {
            workers: Vec::with_capacity(num_workers),
        };

        for id in 0..num_workers {
            pool.workers.push(Worker {
                id,
                state: Free,
                worked_for_seconds: 0,
            });
        }

        pool
    }

    /// Tries to assign a task to any free worker, returns bool indicating if assignment was successful
    pub fn try_assign_task(&mut self, task_id: TaskId, task_duration: TaskDuration) -> bool {
        let free_worker = self.workers.iter_mut().filter(|w| w.state == Free).nth(0);
        match free_worker {
            Some(worker) => {
                worker.state = Busy(task_id, task_duration);
                worker.worked_for_seconds = 0;
                true
            }
            None => false,
        }
    }

    /// Progresses active workers one second forward
    pub fn tick(&mut self) -> FinishedTasks {
        let mut finished_tasks = vec![];

        for worker in &mut self.workers {
            match worker.state {
                Busy(task_id, duration) => {
                    worker.worked_for_seconds += 1;
                    if worker.worked_for_seconds >= duration {
                        worker.set_free();
                        finished_tasks.push(task_id);
                    }
                }
                Free => {}
            }
        }

        finished_tasks
    }

    /// Gets all tasks currently being worked on
    pub fn in_progress_tasks(&self) -> Vec<TaskId> {
        self.workers
            .iter()
            .filter(|w| w.state != Free)
            .map(|w| match w.state {
                Busy(task_id, _) => task_id,
                _ => panic!(),
            })
            .collect()
    }
}
