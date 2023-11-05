//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    mm::{get_kernel_ptr, VirtAddr},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_start_time,
        get_task_status, mmap, munmap, suspend_current_and_run_next, TaskStatus, get_syscall_num,
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
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let kernel_time = get_kernel_ptr(current_user_token(), _ts);
    unsafe {
        *kernel_time = TimeVal {
            sec: us / 1000000,
            usec: us % 1000000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info ");
    let tk_info = get_kernel_ptr(current_user_token(), _ti);
    unsafe {
        (*tk_info).status = get_task_status();
        (*tk_info).time = (get_time_us() - get_start_time())/1000;
        (*tk_info).syscall_times = get_syscall_num();
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let start_vaddr: VirtAddr = _start.into();
    if !start_vaddr.aligned() {
        return -1;
    }
    if _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }
    if _len == 0 {
        return 0;
    }
    mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    let start_vaddr: VirtAddr = _start.into();
    if !start_vaddr.aligned() {
        return -1;
    }
    if _len == 0 {
        return 0;
    }
    munmap(_start, _len)
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
