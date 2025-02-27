//! Types related to task management & Functions for completely changing TCB

use super::id::TaskUserRes;
use super::{kstack_alloc, KernelStack, ProcessControlBlock, TaskContext};
use crate::trap::TrapContext;
use crate::{mm::PhysPageNum, sync::UPSafeCell};
use alloc::sync::{Arc, Weak};
use core::cell::RefMut;
use alloc::collections::btree_map::BTreeMap;

/// Task control block structure
pub struct TaskControlBlock {
    /// immutable
    pub process: Weak<ProcessControlBlock>,
    /// Kernel stack corresponding to PID
    pub kstack: KernelStack,
    /// mutable
    inner: UPSafeCell<TaskControlBlockInner>,
}

impl TaskControlBlock {
    /// Get the mutable reference of the inner TCB
    pub fn inner_exclusive_access(&self) -> RefMut<'_, TaskControlBlockInner> {
        self.inner.exclusive_access()
    }
    /// Get the address of app's page table
    pub fn get_user_token(&self) -> usize {
        let process = self.process.upgrade().unwrap();
        let inner = process.inner_exclusive_access();
        inner.memory_set.token()
    }
}

/// TaskControlBlockInner
pub struct TaskControlBlockInner {
    /// res
    pub res: Option<TaskUserRes>,
    /// The physical page number of the frame where the trap context is placed
    pub trap_cx_ppn: PhysPageNum,
    /// Save task context
    pub task_cx: TaskContext,

    /// Maintain the execution status of the current process
    pub task_status: TaskStatus,
    /// It is set when active exit or execution error occurs
    pub exit_code: Option<i32>,
    /// task infos
    pub infos: TaskInfoBlock,
}

impl TaskControlBlockInner {
    /// get trap cx
    pub fn get_trap_cx(&self) -> &'static mut TrapContext {
        self.trap_cx_ppn.get_mut()
    }

    #[allow(unused)]
    fn get_status(&self) -> TaskStatus {
        self.task_status
    }
}

impl TaskControlBlock {
    /// Create a new task
    pub fn new(
        process: Arc<ProcessControlBlock>,
        ustack_base: usize,
        alloc_user_res: bool,
    ) -> Self {
        let res = TaskUserRes::new(Arc::clone(&process), ustack_base, alloc_user_res);
        let trap_cx_ppn = res.trap_cx_ppn();
        let kstack = kstack_alloc();
        let kstack_top = kstack.get_top();
        Self {
            process: Arc::downgrade(&process),
            kstack,
            inner: unsafe {
                UPSafeCell::new(TaskControlBlockInner {
                    res: Some(res),
                    trap_cx_ppn,
                    task_cx: TaskContext::goto_trap_return(kstack_top),
                    task_status: TaskStatus::Ready,
                    exit_code: None,
                    infos: TaskInfoBlock::new(),
                })
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
/// The execution status of the current process
pub enum TaskStatus {
    /// ready to run
    Ready,
    /// running
    Running,
    /// blocked
    Blocked,
}

#[derive(Debug, Clone, PartialEq)]
/// TaskInfoBlock
pub struct TaskInfoBlock {
    /// syscall times
    pub syscall_times: BTreeMap<usize, u32>,
    /// total running times
    pub running_times: RunningTimeInfo,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RunningTimeInfo {
    pub user_time_us: usize,
    pub kernel_time_us: usize,
    pub first_run_time_us: usize,
}

impl TaskInfoBlock {
    /// create a new TaskInfoBlock object
    pub fn new() -> Self {
        Self {
            syscall_times: BTreeMap::new(),
            running_times: Default::default(), // had derive Default
        }
    }
}

impl Default for TaskInfoBlock {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskControlBlockInner {
    /// Update sysycall times
    pub fn update_syscall_times(&mut self, syscall_id: usize) {
        *self.infos.syscall_times.entry(syscall_id).or_default() += 1;
    }

    /// Get current task info
    pub fn task_info(&self) -> (TaskStatus, TaskInfoBlock) {
        (self.task_status, self.infos.clone())
    }
}
