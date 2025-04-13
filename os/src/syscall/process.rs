//! Process management syscalls

use crate::config::PAGE_SIZE;
use crate::mm::{translated_byte_buffer, PageTable, PhysAddr, VirtAddr};
use crate::task::{change_program_brk, current_user_token, exit_current_and_run_next, push_map_area, suspend_current_and_run_next, unmap_area, TASK_MANAGER};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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
    let token = current_user_token();
    let size = core::mem::size_of::<TimeVal>();

    let mut vs = translated_byte_buffer(token, _ts as *const u8, size);

    let us = get_time_us();

    let time = TimeVal{
        sec:us/1_000_1000,
        usec:us%1_000_000
    };

    let time_bytes = unsafe {
        core::slice::from_raw_parts(&time as *const _ as *const u8, size)
    };
    let mut bytes_write = 0;

    for v in vs.iter_mut() {
        let len = v.len().min(time_bytes.len()-bytes_write);
        if len==0 {
            break;
        }
        v[..len].copy_from_slice(&time_bytes[bytes_write..bytes_write+len]);
        bytes_write += len;
    }

    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");

    let page_table = PageTable::from_token(current_user_token());
    let var = VirtAddr::from(_id);

    let pte = match page_table.translate(var.floor()){
        Some(pte) => pte,
        None => return -1,
    };


    match _trace_request {
        0=>{
            if !pte.is_user() || !pte.readable(){
                return -1;
            }

            let phys_addr:PhysAddr = pte.ppn().into();
            let addr = get_addr(&phys_addr,&var);
            let ptr = addr as *const u8 ;
            unsafe { *ptr as isize }


        }
        1=>{
            if !pte.is_user() || !pte.writable(){
                return -1;
            }


            let phys_addr:PhysAddr = pte.ppn().into();
            let addr = get_addr(&phys_addr,&var);
            let ptr = addr as *mut u8;
            unsafe {
                *ptr= _data as  u8;
            }
            0
        }
        2=>{
            TASK_MANAGER.get_call_num(_id)
        }
        _ => {
        -1
    }
    }
}

fn get_addr(phys_addr:&PhysAddr,virt_addr: &VirtAddr)->usize{
    phys_addr.0 | virt_addr.page_offset()
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _port &!0x7!=0 || _port &0x7==0{
        return -1;
    }
    if !VirtAddr::from(_start).aligned(){
        return -1;
    }

    let len = (_len+PAGE_SIZE-1) & ! (PAGE_SIZE-1);
    if !push_map_area(_start, _start+len, _port<<1) {
        return -1;
    }
    0
}


// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if !VirtAddr::from(_start).aligned() {
        return -1;
    }
    let len = (_len+PAGE_SIZE-1)&!(PAGE_SIZE-1);
    if !unmap_area(_start,_start+len) {
        return -1;
    }
    0
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
