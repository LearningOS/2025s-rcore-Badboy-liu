//! Types related to task management

use alloc::boxed::ThinBox;
use crate::syscall::SYSTEM_CALL_MAX_NUM;
use super::TaskContext;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// task info
    pub task_info:TaskInfo,
}
impl TaskControlBlock {
    /// call inc
    pub fn system_call_num_inc(&mut self, system_id: usize) {
        self.task_info.system_call_num_inc(system_id)
    }
}
#[derive(Clone, Copy)]
pub struct TaskInfo{
    /// call num
    system_call_nums:[usize;SYSTEM_CALL_MAX_NUM],
}




impl TaskInfo {
    pub fn new()-> Self {
        let five = ThinBox::new([0;500]);
        TaskInfo{system_call_nums:[0;SYSTEM_CALL_MAX_NUM]}
    }
    pub fn system_call_num_inc(&mut self, system_id:usize){
        // let index = index(system_id);
        self.system_call_nums[system_id] +=1;
        // println!("system_id:{} system_call_num_inc [system_id]+=1 {}", system_id,self.system_call_nums[index]);
    }
    pub fn get_system_call_num(&self,system_id:usize)->usize{
        // let index = index(system_id);
        // println!("system_id:{} get_system_call_num {}",system_id, self.system_call_nums[index]);
        self.system_call_nums[system_id]
    }
}

// pub fn index(system_call_id:usize)->usize{
//     let i ;
//     match system_call_id {
//         64=> i= 0,
//         93=> i= 1,
//         124=> i= 2,
//         169=> i= 3,
//         410=> i= 4,
//         _ => {
//             i = 0;
//     }
//
//     }
//     i
// }

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
