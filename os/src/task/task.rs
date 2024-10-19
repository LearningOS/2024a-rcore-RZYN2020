//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// tasks statics
    pub task_stat: TaskStat,
    /// The task context
    pub task_cx: TaskContext,
}

#[derive(Clone, Copy)]
pub struct TaskStat {
    pub start_time: usize,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
}

impl TaskStat {
    pub fn new() -> Self {
        TaskStat {
            start_time: 0,
            syscall_times: [0; MAX_SYSCALL_NUM],
        }
    }
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
