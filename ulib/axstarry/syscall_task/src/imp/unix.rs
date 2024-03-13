extern crate alloc;
use alloc::sync::Arc;

use axprocess::current_process;
use syscall_utils::{SyscallError, SyscallResult};
use syscall_net::UnixSocket;

/// 创建一对套接字
///
/// 仅仅实现一个简单的 unix socket
/// 因此暂时忽略 domain socktype protocol 参数
///
pub fn syscall_socketpair(_domain: u32, _socktype: u32, _protocol: u32, fd: *mut u32) -> SyscallResult {
    let unix_socket1 = UnixSocket::new();
    let unix_socket2 = UnixSocket::new_with_pair(&unix_socket1);

    let curr = current_process();
    let mut fd_table = curr.fd_manager.fd_table.lock();

    let Ok(fd1) = curr.alloc_fd(&mut fd_table) else {
        return Err(SyscallError::EMFILE);
    };
    fd_table[fd1] = Some(Arc::new(unix_socket1));

    let Ok(fd2) = curr.alloc_fd(&mut fd_table) else {
        return Err(SyscallError::EMFILE);
    };
    fd_table[fd2] = Some(Arc::new(unix_socket2));

    unsafe {
        core::ptr::write(fd, fd1 as u32);
        core::ptr::write(fd.offset(1), fd2 as u32);
    }

    Ok(0)
}
