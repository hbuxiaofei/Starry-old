extern crate alloc;
use alloc::sync::Arc;
use core::mem::size_of;
use axerrno::{AxError, AxResult};
use axfs::api::{FileIO, FileIOType, SeekFrom};
use axsync::Mutex;
use axtask::yield_now;
use axlog::error;

pub struct EventfdFile {
    pub inner: Arc<Mutex<EventfdFileInner>>,
}

struct EventfdCtx {
    count: u64,
    flags: u32,
}

impl EventfdCtx {
    pub fn new(count: u64, flags: u32) -> Self {
        Self {
            count,
            flags,
        }
    }
}

pub struct EventfdFileInner {
    ctx: EventfdCtx,
}

impl EventfdFile {
    pub fn new(count: u64, flags: u32) -> Self {
        Self {
            inner: Arc::new(Mutex::new(EventfdFileInner {
                ctx: EventfdCtx::new(count, flags),
            })),
        }
    }
}

impl FileIO for EventfdFile {
    fn read(&self, buf: &mut [u8]) -> AxResult<usize> {
        error!(">>> eventfd read start");
        let mut cnt = 0;
        loop {
            let mut inner = self.inner.lock();
            if inner.ctx.count > 0 {
                cnt = inner.ctx.count; // EFD_SEMAPHORE ?
                inner.ctx.count -= cnt;
                break
            }
            drop(inner);
            yield_now();
        }

        let bytes = cnt.to_ne_bytes();
        buf.copy_from_slice(&bytes);

        error!(">>> eventfd read over");

        Ok(size_of::<u64>())
    }
    fn write(&self, buf: &[u8]) -> AxResult<usize> {
        error!(">>> eventfd write start");

        let mut inner = self.inner.lock();
        let ucnt: u64 = u64::from_ne_bytes(buf.try_into().unwrap());
        inner.ctx.count += ucnt;

        error!(">>> eventfd write over");

        Ok(size_of::<u64>())
    }
    fn flush(&self) -> AxResult {
        Err(AxError::Unsupported)
    }
    fn seek(&self, _pos: SeekFrom) -> AxResult<u64> {
        Err(AxError::Unsupported)
    }
    fn readable(&self) -> bool {
        true
    }
    fn writable(&self) -> bool {
        true
    }
    fn ready_to_read(&self) -> bool {
        let inner = self.inner.lock();
        if inner.ctx.count > 0 {
            true
        } else {
            false
        }
    }
    fn ready_to_write(&self) -> bool {
        true
    }
    fn executable(&self) -> bool {
        false
    }
    fn get_type(&self) -> FileIOType {
        FileIOType::FileDesc
    }
}
