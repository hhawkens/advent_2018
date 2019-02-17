use std::collections::HashSet;

pub type TaskId = char;

/// Describes the dependencies from and to a task
#[derive(Debug, Eq, PartialEq)]
pub struct Deps {
    pub dependency_of: HashSet<TaskId>,
    pub depending_on: HashSet<TaskId>
}
