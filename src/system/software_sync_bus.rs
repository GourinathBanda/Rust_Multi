//! # Software synchronization bus definition
//!
use crate::system::types::{SemaphoreId, TaskId};
use crate::types::BooleanVector;
use crate::KernelError;
use crate::kernel::task_management::{get_curr_tid, release, schedule};
use cortex_m::interrupt;
use core::cell::RefCell;

/// Semaphores form the core of synchronization and communication in the Kernel.
pub struct SemaphoreControlBlock {
    /// It is a boolean vector which represents the tasks notified by the semaphore.
    pub flags: RefCell<BooleanVector>,
    /// It is a boolean vector that corresponds to the tasks that are to be released by the semaphore on being signaled.
    pub tasks: BooleanVector,
}

impl SemaphoreControlBlock {
    /// Creates and returns a new semaphore instance with tasks field set to `tasks_mask`.
    pub const fn new(tasks: BooleanVector) -> Self {
        Self { flags: RefCell::new(0), tasks }
    }

    /// This method, when called, appends the `tasks_mask` to the flags field. Next, the tasks in the tasks field are released.
    pub fn signal_and_release(&self, tasks_mask: BooleanVector) {
        interrupt::free(|_| {
            let flags: &mut BooleanVector = &mut self.flags.borrow_mut();
            *flags |= tasks_mask;
            release(self.tasks);
            schedule();
        })
    }

    /// This method, when called, appends the `tasks_mask` to the `flags` field. Next, the `tasks` in the `tasks` field are released.
    pub fn test_and_reset(&self) -> Result<bool, KernelError> {
        interrupt::free(|_| {
            let curr_tid = get_curr_tid() as u32;
        let curr_tid_mask = 1 << curr_tid;
        let flags: &mut BooleanVector = &mut self.flags.borrow_mut();
        if *flags & curr_tid_mask == curr_tid_mask {
            *flags &= !curr_tid_mask;
            return Ok(true);
        } else {
            return Ok(false);
        }
    })
}
}

unsafe impl Sync for SemaphoreControlBlock {}