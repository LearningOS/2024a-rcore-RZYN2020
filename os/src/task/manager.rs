//!Implementation of [`TaskManager`]

use super::TaskControlBlock;
use crate::config::BIG_STRIDE;
use crate::sync::UPSafeCell;
use alloc::collections::binary_heap::BinaryHeap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use lazy_static::*;

struct ArcTask(Arc<TaskControlBlock>);

impl PartialOrd for ArcTask {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArcTask {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        other
            .0
            .inner_exclusive_access()
            .stride
            .cmp(&self.0.inner_exclusive_access().stride)
    }
}

impl Eq for ArcTask {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialEq for ArcTask {
    fn eq(&self, other: &Self) -> bool {
        self.0.inner_exclusive_access().stride == other.0.inner_exclusive_access().stride
    }
}

///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_heap: BinaryHeap<ArcTask>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_heap: BinaryHeap::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_heap.push(ArcTask(task));
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.ready_heap.pop().map(|t| t.0)
    }

    /// ,,,
    pub fn update_stride(&mut self, task: Arc<TaskControlBlock>) {
        let mut modified_tasks = Vec::new();
        while let Some(t) = self.ready_heap.pop() {
            if Arc::as_ptr(&t.0) == Arc::as_ptr(&task) {
                // 修改优先级
                t.0.inner_exclusive_access().stride +=
                    BIG_STRIDE / t.0.inner_exclusive_access().priority;
            }
            modified_tasks.push(t);
        }
        for task in modified_tasks {
            self.ready_heap.push(task);
        }
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// ...
pub fn update_stride(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().update_stride(task);
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}
