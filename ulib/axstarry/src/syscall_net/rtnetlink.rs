extern crate alloc;
use alloc::{vec, vec::Vec};

use axerrno::{AxError, AxResult};
use num_enum::TryFromPrimitive;

macro_rules! netlink_align {
    ($len:expr) => {
        (($len + 3) & !3)
    };
}

#[derive(TryFromPrimitive, PartialEq, Eq, Clone, Debug)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum RtmType {
    RTM_NEWLINK	= 16, // RTM_BASE
    RTM_DELLINK = 17,
    RTM_GETLINK = 18,
    RTM_SETLINK = 19,
    RTM_NEWADDR = 20,
    RTM_DELADDR = 21,
    RTM_GETADDR = 22,
}

#[derive(TryFromPrimitive, PartialEq, Eq, Clone, Debug)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum IflaSpec {
    IFLA_UNSPEC = 0,
    IFLA_ADDRESS = 1,
    IFLA_BROADCAST = 2,
    IFLA_IFNAME = 3,
    IFLA_MTU = 4,
    IFLA_LINK = 5,
    IFLA_QDISC = 6,
    IFLA_STATS = 7,
    IFLA_COST = 8,
}

#[derive(TryFromPrimitive, PartialEq, Eq, Clone, Debug)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum IfaSpec {
    IFA_UNSPEC = 0,
    IFA_ADDRESS = 1,
    IFA_LOCAL = 2,
    IFA_LABEL = 3,
    IFA_BROADCAST = 4,
    IFA_ANYCAST = 5,
    IFA_CACHEINFO = 6,
    IFA_MULTICAST = 7,
}

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


#[derive(PartialEq, Eq, Copy, Clone, Debug, Default)]
#[repr(align(4))]
#[repr(C)]
pub struct NlMsgHdr {
    pub nlmsg_len: u32,
    pub nlmsg_type: u16,
    pub nlmsg_flags: u16,
    pub nlmsg_seq: u32,
    pub nlmsg_pid: u32,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct IfInfoMsg {
    pub ifi_family: u8,
    pub __ifi_pad: u8,
    pub ifi_type: u16,
    pub ifi_index: i32,
    pub ifi_flags: u32,
    pub ifi_change: u32,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct IfAddrMsg {
    pub ifa_family: u8,
    /// The prefix length
    pub ifa_prefixle: u8,
    /// Flags
    pub ifa_flags: u8,
    /// Address scope
    pub ifa_scope: u8,
    /// Link index
    pub ifa_index: u32,
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
#[repr(C)]
pub struct RtAddr {
    pub rta_len: u16,
    pub rta_type: u16,
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
#[repr(C)]
struct RtnlLinkStats {
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub rx_bytes: u32,
    pub tx_bytes: u32,
    pub rx_errors: u32,
    pub tx_errors: u32,
    pub rx_dropped: u32,
    pub tx_dropped: u32,
    pub multicast: u32,
    pub collisions: u32,

    pub rx_length_errors: u32,
    pub rx_over_errors: u32,
    pub rx_crc_errors: u32,
    pub rx_frame_errors: u32,
    pub rx_fifo_errors: u32,
    pub rx_missed_errors: u32,

    pub tx_aborted_errors: u32,
    pub tx_carrier_errors: u32,
    pub tx_fifo_errors: u32,
    pub tx_heartbeat_errors: u32,
    pub tx_window_errors: u32,

    pub rx_compressed: u32,
    pub tx_compressed: u32,

    pub rx_nohandler: u32,
}

fn nlmsg_end(skb: &mut SkBuff, nlh: &mut NlMsgHdr) -> usize {
    let len = skb.length();
    let align = netlink_align!(len) - len;
    if align > 0 {
        let v: Vec<u8> = vec![0; align];
        skb.push_data(&v);
   }

    nlh.nlmsg_len = skb.length() as u32;

    let ptr: *const u8 = skb.get_data().as_ptr();
    let nlh_ptr: *mut NlMsgHdr = ptr as *mut NlMsgHdr;
    unsafe {
        let nlh_new: &mut NlMsgHdr = &mut *nlh_ptr;
        nlh_new.nlmsg_len = nlh.nlmsg_len;
    }

    skb.length()
}

fn nla_put_u8(skb: &mut SkBuff, attrtype: u16, buf: &[u8]) {
    let mut rtattr = RtAddr {
        ..Default::default()
    };
    rtattr.rta_type = attrtype;
    rtattr.rta_len = core::mem::size_of::<RtAddr>() as u16 + buf.len() as u16;

    let ptr = &rtattr as *const RtAddr as *const u8;
    let rtattr_slice = unsafe {
        core::slice::from_raw_parts(ptr, core::mem::size_of::<RtAddr>())
    };

    skb.skb_put(rtattr_slice);
    skb.skb_put(buf);
}

fn nla_put_u32(skb: &mut SkBuff, attrtype: u16, value: u32) {
    let bytes: [u8; 4] = value.to_ne_bytes();
    nla_put_u8(skb, attrtype, &bytes);
}

fn nla_put_string(skb: &mut SkBuff, attrtype: u16, s: &str) {
    let bytes = s.as_bytes();
    nla_put_u8(skb, attrtype, &bytes);
}

pub fn nlmsg_put(skb: &mut SkBuff, portid: u32, seq: u32, ty: u16, len: u32, flags: u16) {
    let nlh = NlMsgHdr {
        nlmsg_type: ty,
        nlmsg_len: len + core::mem::size_of::<NlMsgHdr>() as u32,
        nlmsg_flags: flags,
        nlmsg_pid: portid,
        nlmsg_seq: seq,
    };

    let ptr = &nlh as *const NlMsgHdr as *const u8;
    let nlh_buf = unsafe {
        core::slice::from_raw_parts(ptr, core::mem::size_of::<NlMsgHdr>())
    };
    skb.skb_put(nlh_buf);

    let v: Vec<u8> = vec![0; netlink_align!(len) as usize];
    skb.skb_put(&v[..]);
}


pub fn rtnl_getlink(skb: &mut SkBuff, nlh: &mut NlMsgHdr) -> AxResult {
    nlh.nlmsg_type = RtmType::RTM_NEWLINK as u16;
    let ptr = nlh as *const NlMsgHdr as *const u8;
    let nlh_buf = unsafe {
        core::slice::from_raw_parts(ptr, core::mem::size_of::<NlMsgHdr>())
    };
    skb.skb_put(nlh_buf);

   let ifinfomsg = IfInfoMsg {
        ..Default::default()
    };
    let ptr = &ifinfomsg as *const IfInfoMsg as *const u8;
    let ifinfomsg_buf = unsafe {
        core::slice::from_raw_parts(ptr, core::mem::size_of::<IfInfoMsg>())
    };
    skb.skb_put(ifinfomsg_buf);

    nla_put_string(skb, IflaSpec::IFLA_IFNAME as u16, "eth0");

    nla_put_u32(skb, IflaSpec::IFLA_MTU as u16, 1500);

    let mut link_state = RtnlLinkStats {
        ..Default::default()
    };
    link_state.rx_packets = 75937;
    link_state.rx_bytes = 29396057;
    link_state.tx_packets = 506109;
    link_state.tx_bytes = 174857788;
    let ptr = &link_state as *const RtnlLinkStats as *const u8;
    let link_state_buf = unsafe {
        core::slice::from_raw_parts(ptr, core::mem::size_of::<RtnlLinkStats>())
    };
    nla_put_u8(skb, IflaSpec::IFLA_STATS as u16, &link_state_buf);

    let mac: [u8; 6] = [0x00, 0x0c, 0x29, 0xe9, 0xf2, 0x2e];
    nla_put_u8(skb, IflaSpec::IFLA_ADDRESS as u16, &mac);

    nlmsg_end(skb, nlh);

    Ok(())
}

pub fn rtnl_dump_ifinfo(skb: &mut SkBuff, nlh: &mut NlMsgHdr) -> AxResult {
    nlh.nlmsg_type = RtmType::RTM_NEWADDR as u16;
    let ptr = nlh as *const NlMsgHdr as *const u8;
    let nlh_buf = unsafe {
        core::slice::from_raw_parts(ptr, core::mem::size_of::<NlMsgHdr>())
    };
    skb.skb_put(nlh_buf);

    let ifaddrmsg = IfAddrMsg {
        ..Default::default()
    };
    let ptr = &ifaddrmsg as *const IfAddrMsg as *const u8;
    let ifaddrmsg_buf = unsafe {
        core::slice::from_raw_parts(ptr, core::mem::size_of::<IfAddrMsg>())
    };
    skb.skb_put(ifaddrmsg_buf);

    nla_put_string(skb, IfaSpec::IFA_LABEL as u16, "eth0");
    nla_put_u32(skb, IfaSpec::IFA_ADDRESS as u16, 0);

    nlmsg_end(skb, nlh);

    Ok(())
}
