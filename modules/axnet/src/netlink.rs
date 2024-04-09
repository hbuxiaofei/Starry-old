use num_enum::TryFromPrimitive;
extern crate alloc;
use alloc::sync::Arc;
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


struct NetlinkBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> NetlinkBuffer<T> {
    fn new(capacity: usize) -> Self {
        NetlinkBuffer {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    fn push(&mut self, element: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(element);
    }

    fn pop(&mut self) -> Option<T> {
        self.buffer.pop_front()
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

pub struct NetlinkSocket {
    pub protocol: NetlinkProto,
    pub nl_groups: u32,
    pub nl_pid: u32,
    rx_buffer: Arc<Mutex<NetlinkBuffer<u8>>>,
    tx_buffer: Arc<Mutex<NetlinkBuffer<u8>>>,
}

impl NetlinkSocket {
    pub fn new(protocol: NetlinkProto) -> Self {
        NetlinkSocket {
            protocol,
            nl_groups: 64, // RTNLGRP_MAX ?
            nl_pid: 0,
            rx_buffer: Arc::new(Mutex::new(NetlinkBuffer::new(1024))),
            tx_buffer: Arc::new(Mutex::new(NetlinkBuffer::new(1024))),
        }
    }

    pub fn bind(&mut self, groups: u32) -> AxResult {
        self.nl_groups = groups;
        Ok(())
    }

    pub fn send(&self, buf: &[u8]) -> AxResult<usize> {
        let mut rx = self.rx_buffer.lock();
        for byte in buf.iter() {
            rx.push(*byte);
        }
        Ok(buf.len())
    }

    pub fn fill_tx(&self, buf: &[u8]) -> AxResult<usize> {
        let mut tx = self.tx_buffer.lock();
        let length = min(buf.len(), tx.capacity() - tx.len());
        for i in 0..length {
            tx.push(buf[i]);
        }
       Ok(length)
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> AxResult<usize> {
       //  loop {
       //      let len = {
       //          let tx = self.tx_buffer.lock();
       //          tx.len()
       //      };
       //      if len == 0 {
       //          yield_now();
       //          continue;
       //      }
       //      break;
       //  }

        let mut tx = self.tx_buffer.lock();
        let length = min(buf.len(), tx.len());
        for i in 0..length {
            buf[i] = tx.pop().unwrap();
        }
       Ok(length)
    }
}
