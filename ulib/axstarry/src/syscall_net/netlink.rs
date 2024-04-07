use axprocess::current_process;
use super::socket::{Socket, SocketInner};

pub fn netlink_unicast()
{
    let process = current_process();
    let fd_table = process.fd_manager.fd_table.lock();

    // # define NLMSG_OK(nlh,len) ((len) >= (int)sizeof(struct nlmsghdr) && \
	//		   (nlh)->nlmsg_len >= sizeof(struct nlmsghdr) && \
	//		   (nlh)->nlmsg_len <= (len))

    let mut buffer: [u8; 20] = [0; 20];
    buffer[0] = 16;  // nlmsg_len: sizeof(struct nlmsghdr)
    buffer[4] = 16;  // nlmsg_type: RTM_NEWLINK
    buffer[6] = 65;  // 'A'
    buffer[7] = 66;  // 'B'
    buffer[8] = 67;  // 'C'
    buffer[9] = 68;  // 'D'
    buffer[10] = 69; // 'E'

    for i in 0..fd_table.len() {
        if let Some(file) = fd_table[i].as_ref() {
             if let Some(s_file) = file.as_any().downcast_ref::<Socket>() {
                let inner = s_file.inner.lock();
                if let SocketInner::Netlink(s) = &*inner {
                    let _ = s.fill_tx(&buffer);
                }
             }
        }
    }

}



