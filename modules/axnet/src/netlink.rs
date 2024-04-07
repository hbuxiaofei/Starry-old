extern crate alloc;
use alloc::sync::Arc;
use alloc::collections::VecDeque;
use core::cmp::min;

use axsync::Mutex;
use axerrno::AxResult;

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
    rx_buffer: Arc<Mutex<NetlinkBuffer<u8>>>,
    tx_buffer: Arc<Mutex<NetlinkBuffer<u8>>>,
    pub nl_groups: u32,
}

impl NetlinkSocket {
    pub fn new() -> Self {
        NetlinkSocket {
            rx_buffer: Arc::new(Mutex::new(NetlinkBuffer::new(1024))),
            tx_buffer: Arc::new(Mutex::new(NetlinkBuffer::new(1024))),
            nl_groups: 0,
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
        let mut tx = self.tx_buffer.lock();
        let length = min(buf.len(), tx.len());
        for i in 0..length {
            buf[i] = tx.pop().unwrap();
        }
       Ok(length)
    }
}
