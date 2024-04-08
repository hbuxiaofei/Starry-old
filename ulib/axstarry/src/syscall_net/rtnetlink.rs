extern crate alloc;
use alloc::{string::String, string::ToString, collections::BTreeMap, vec::Vec};

use axerrno::{AxError, AxResult};


pub struct SkBuff {
    payload: Vec<u8>,
    len: usize,
}

impl SkBuff {
    pub fn new() -> SkBuff {
        SkBuff {
            payload: Vec::new(),
            len: 0,
        }
    }

    pub fn push_data(&mut self, data: &[u8]) {
        self.payload.extend_from_slice(data);
        self.len += data.len();
    }

    pub fn get_data(&self) -> &[u8] {
        self.payload.as_slice()
    }

    pub fn length(&self) -> usize {
        self.len
    }
}


impl SkBuff {
    pub fn skb_put(&mut self, data: &[u8]) {
        self.push_data(data);
    }

    pub fn skb_pull(&mut self, len: usize) -> Option<Vec<u8>> {
        if self.len < len {
            None
        } else {
            let mut removed = Vec::with_capacity(len);
            removed.extend(self.payload.drain(..len));
            self.len -= len;
            Some(removed)
        }
    }

    pub fn skb_push(&mut self, data: &[u8]) {
        let mut new_payload = Vec::with_capacity(data.len() + self.payload.len());
        new_payload.extend_from_slice(data);
        new_payload.append(&mut self.payload);
        self.payload = new_payload;
        self.len += data.len();
    }

    pub fn skb_trim(&mut self, len: usize) {
        if self.len > len {
            self.payload.truncate(len);
            self.len = len;
        }
    }
}


#[derive(PartialEq, Eq, Clone, Debug)]
#[repr(C)]
pub struct NlMsgHdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}

type FnPtr = fn(&mut SkBuff, &mut NlMsgHdr);

pub fn rtnl_getlink(skb: &mut SkBuff, nlh: &mut NlMsgHdr) -> AxResult {
    Ok(())
}
