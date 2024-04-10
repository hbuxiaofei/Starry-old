use alloc::vec::Vec;
use super::rtnetlink::*;
use axnet::NetlinkSocket;
use num_enum::TryFromPrimitive;
use super::rtnetlink::*;

#[derive(TryFromPrimitive, PartialEq, Eq, Debug)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum NlMessageFlags {
    /// It is request message.
    NLM_F_REQUEST = 1,
    /// Multipart message, terminated by NLMSG_DONE.
    NLM_F_MULTI = 2,
    /// Reply with ack, with zero or error code.
    NLM_F_ACK = 4,
    /// Echo this request.
    NLM_F_ECHO = 8,
    /// Dump was inconsistent due to sequence change.
    NLM_F_DUMP_INTR = 16,
}

#[derive(TryFromPrimitive, PartialEq, Eq, Debug)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum NlMessageGet {
    NLM_F_ROOT = 0x100,
    NLM_F_MATCH = 0x200,
    NLM_F_ATOMIC = 0x400,
    NLM_F_DUMP = 0x100 | 0x200,
}

#[derive(TryFromPrimitive, PartialEq, Eq, Debug)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum NlMessageNew {
    NLM_F_REPLACE = 0x100,
    NLM_F_EXCL = 0x200,
    NLM_F_CREATE = 0x400,
    NLM_F_APPEND = 0x800,
}

pub fn netlink_ack(sk: &NetlinkSocket, nlh: &mut NlMsgHdr)
{
    // # define NLMSG_OK(nlh,len) ((len) >= (int)sizeof(struct nlmsghdr) && \
	//		   (nlh)->nlmsg_len >= sizeof(struct nlmsghdr) && \
	//		   (nlh)->nlmsg_len <= (len))

    if let Ok(msg_type) = RtmType::try_from(nlh.nlmsg_type) {
        if msg_type == RtmType::RTM_GETLINK {
            let mut skb = SkBuff::new();
            let _ = rtnl_getlink( &mut skb, nlh);
            // let _ = sk.fill_tx(skb.get_data());

            let mut skb_done = SkBuff::new();
            nlmsg_put(&mut skb_done, 0, nlh.nlmsg_seq + 1, 0x3, 0, 0); // NLMSG_DONE 0x3
            // let _ = sk.fill_tx(skb_done.get_data());

            let tx_buf1 = skb.get_data();
            let tx_buf2 = skb_done.get_data();

            let mut combined_vec = Vec::new();
            combined_vec.extend_from_slice(tx_buf1);
            combined_vec.extend_from_slice(tx_buf2);

            let combined_slice: &[u8] = &combined_vec;
            let _ = sk.fill_tx( combined_slice);
        } else if msg_type == RtmType::RTM_GETADDR {
            let mut skb = SkBuff::new();
            let _ = rtnl_dump_ifinfo( &mut skb, nlh);
            // let _ = sk.fill_tx(skb.get_data());

            let mut skb_done = SkBuff::new();
            nlmsg_put(&mut skb_done, 0, nlh.nlmsg_seq + 1, 0x3, 4, 0); // NLMSG_DONE 0x3
            // let _ = sk.fill_tx(skb_done.get_data());

            let tx_buf1 = skb.get_data();
            let tx_buf2 = skb_done.get_data();

            let mut combined_vec = Vec::new();
            combined_vec.extend_from_slice(tx_buf1);
            combined_vec.extend_from_slice(tx_buf2);

            let combined_slice: &[u8] = &combined_vec;
            let _ = sk.fill_tx( combined_slice);
       }
    };
}



