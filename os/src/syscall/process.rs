//! Process management syscalls
use riscv::{addr::Page, paging::PageTable};

use crate::{
    config::MAX_SYSCALL_NUM, 
    mm::{from_va_to_pa, VirtAddr, MapPermission}, 
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_times, 
        get_task_times, suspend_current_and_run_next, TaskStatus, insert_framed_area
    }, 
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    // `ts` is a mutable pointer(address) to a `TimeVal` type. 
    trace!("kernel: sys_get_time");
    let time = get_time_us();
    let ts = from_va_to_pa(current_user_token(), ts as usize) as *mut TimeVal;
    // from virtual address to physics address
    unsafe {
        *ts = TimeVal{
            sec: time / 1_000_000,
            usec: time % 1_000_000
        };
    }
    0  
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let ti = from_va_to_pa(current_user_token(), ti as usize) as *mut TaskInfo;
    unsafe {
        *ti = TaskInfo { 
            // or, (*ti).status = TaskStatus::Ready;
            status: TaskStatus::Running,
            syscall_times: get_syscall_times(),
            time: get_task_times()
        };   
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    //trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if (port & !0x7) != 0 || (port & 0x7) == 0 {
        return -1;
    }
    let end = start + len;
    let start_va = VirtAddr::from(start);
    let end_va = VirtAddr::from(end);

    // let mut start_vpn = start_va.floor();
    // let pt = PageTable::from_token(current_user_token());
    // for _ in 0..((len + Page::SIZE - 1) / Page::SIZE) {
    //     match pt.translate(start_vpn) {
    //         Some(pte) => {
    //             if pte.is_valid() {
    //                 return -1;
    //             }
    //         }
    //         None => {}
    //     }
    //     start_vpn.step();
    // }

    let mut permission = MapPermission::empty();
    if port & 0x1 != 0 {
        permission |= MapPermission::R;
    }
    if port & 0x2 != 0 {
        permission |= MapPermission::W;
    }
    if port & 0x4 != 0 {
        permission |= MapPermission::X;
    }
    permission |= MapPermission::U;
    insert_framed_area(start_va, end_va, permission);
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
