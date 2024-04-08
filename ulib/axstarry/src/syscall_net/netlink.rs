use super::rtnetlink::*;
use axnet::NetlinkSocket;

pub fn netlink_ack(sk: &NetlinkSocket, nlh: &NlMsgHdr)
{
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

    if nlh.nlmsg_type == 18 || nlh.nlmsg_type == 20 {
	    let _ = sk.fill_tx(&buffer);
    }
}



