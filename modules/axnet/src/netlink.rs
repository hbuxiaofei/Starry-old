use num_enum::TryFromPrimitive;
extern crate alloc;
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::collections::VecDeque;
use core::cmp::min;
use axtask::yield_now;

use axsync::Mutex;
use axerrno::AxResult;

#[derive(TryFromPrimitive, PartialEq, Eq, Clone, Debug)]
#[repr(usize)]
#[allow(non_camel_case_types)]
pub enum NetlinkProto {
    /// Routing/device hook
    NETLINK_ROUTE = 0,
    /// Unused number
    NETLINK_UNUSED = 1,
    /// Reserved for user mode socket protocols
    NETLINK_USERSOCK = 2,
    /// Unused number, formerly ip_queu
    NETLINK_FIREWALL = 3,
    /// socket monitoring
    NETLINK_SOCK_DIAG = 4,
    /// netfilter/iptables ULOG
    NETLINK_NFLOG = 5,
    /// auditing
    NETLINK_AUDIT = 9,
    /// netfilter subsystem
    NETLINK_NETFILTER = 12,
    MAX_LINKS = 32,
}


#[derive(Debug)]
struct NetlinkFrame {
    pub data: Vec<u8>,
}

#[derive(Debug)]
struct NetlinkFrameQueue {
    pub buffer: Arc<Mutex<VecDeque<NetlinkFrame>>>,
}

#[allow(dead_code)]
impl NetlinkFrameQueue {
   fn new() -> Self {
        NetlinkFrameQueue {
            buffer: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    fn push_frame(&self, frame: NetlinkFrame) {
        self.buffer.lock().push_back(frame);
    }

    fn push_front(&self, frame: NetlinkFrame) {
        self.buffer.lock().push_front(frame);
    }

    fn pop_frame(&self) -> Option<NetlinkFrame> {
        self.buffer.lock().pop_front()
    }

    fn pop_back(&self) -> Option<NetlinkFrame> {
        self.buffer.lock().pop_back()
    }

    fn is_empty(&self) -> bool {
        self.buffer.lock().is_empty()
    }

    fn len(&self) -> usize {
        self.buffer.lock().len()
    }
}

pub struct NetlinkSocket {
    pub protocol: NetlinkProto,
    pub nl_groups: u32,
    pub nl_pid: u32,
    rx_buffer: NetlinkFrameQueue,
    tx_buffer: NetlinkFrameQueue,
}

impl NetlinkSocket {
    pub fn new(protocol: NetlinkProto) -> Self {
        NetlinkSocket {
            protocol,
            nl_groups: 64, // RTNLGRP_MAX ?
            nl_pid: 0,
            rx_buffer: NetlinkFrameQueue::new(),
            tx_buffer: NetlinkFrameQueue::new(),
        }
    }

    pub fn bind(&mut self, groups: u32) -> AxResult {
        self.nl_groups = groups;
        Ok(())
    }

    pub fn send(&self, buf: &[u8]) -> AxResult<usize> {
        let frame = NetlinkFrame {
            data: buf.to_vec()
        };

        self.rx_buffer.push_frame(frame);

        Ok(buf.len())
    }

    pub fn fill_tx(&self, buf: &[u8]) -> AxResult<usize> {
        let frame = NetlinkFrame {
            data: buf.to_vec()
        };

        self.tx_buffer.push_frame(frame);

        Ok(buf.len())
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> AxResult<usize> {
        loop {
            let len = {
                self.tx_buffer.len()
            };
            if len == 0 {
                yield_now();
                continue;
            }
            break;
        }

        let mut length = 0;
        if let Some(frame) = self.tx_buffer.pop_frame() {
            length = min(buf.len(), frame.data.len());
            if length < frame.data.len() {
                let left = &frame.data[length..frame.data.len()];
                let left_frame = NetlinkFrame {
                    data: left.to_vec()
                };
                self.tx_buffer.push_front(left_frame);
            }
            for i in 0..length {
                buf[i] = frame.data[i];
            }
        }

        Ok(length)
    }
}
