extern crate alloc;
use alloc::sync::Arc;
use crate::{SyscallError, SyscallResult};
use axprocess::current_process;

use crate::syscall_fs::ctype::eventfd::{EventfdFile};

pub fn syscall_eventfd2(args: [usize; 6]) -> SyscallResult {
    let initval = args[0];
    let flags = args[1];

    let file = EventfdFile::new(initval as u64, flags as u32);
    let process = current_process();
    let mut fd_table = process.fd_manager.fd_table.lock();
    if let Ok(num) = process.alloc_fd(&mut fd_table) {
        fd_table[num] = Some(Arc::new(file));
        Ok(num as isize)
    } else {
        Err(SyscallError::EMFILE)
    }
}
