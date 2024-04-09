use super::rtnetlink::*;
use axnet::NetlinkSocket;
use axlog::error;
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

    let mut done_msg: [u8; 20] = [0; 20];
    done_msg[0] = 16;  // nlmsg_len: sizeof(struct nlmsghdr)
    done_msg[4] = 0x3; // nlmsg_type: NLMSG_DONE
    done_msg[6] = 65;  // 'A'
    done_msg[7] = 66;  // 'B'
    done_msg[8] = 67;  // 'C'
    done_msg[9] = 68;  // 'D'
    done_msg[10] = 69; // 'E'

   if let Ok(msg_type) = RtmType::try_from(nlh.nlmsg_type) {
        if msg_type == RtmType::RTM_GETLINK || msg_type == RtmType::RTM_GETADDR {
            let mut skb = SkBuff::new();
            let _ = rtnl_getlink( &mut skb, nlh);
            error!(">>> recv nlmsg_len:{} skb: {:?}", nlh.nlmsg_len, skb.get_data());
			let _ = sk.fill_tx(skb.get_data());
            // let _ = sk.fill_tx(&done_msg);
        }
    };
}



