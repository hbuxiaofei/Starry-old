extern crate alloc;
use alloc::sync::Arc;
use core::mem::size_of;
use axerrno::{AxError, AxResult};
use axfs::api::{FileIO, OpenFlags, FileIOType, SeekFrom};
use axsync::Mutex;
use axtask::yield_now;
use axlog::error;

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct EfdFlags: u32 {
        const EFD_SEMAPHORE = 0o0000001;
        const EFD_CLOEXEC = 0o2000000;
        const EFD_NONBLOCK = 0o0004000;
    }
}

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

        if buf.len() < core::mem::size_of::<u64>() {
            return Err(AxError::InvalidInput);
        }

        let mut cnt = 0;
        loop {
            let mut inner = self.inner.lock();
            let flags = EfdFlags::from_bits_truncate(inner.ctx.flags);
            if inner.ctx.count > 0 || flags.contains(EfdFlags::EFD_NONBLOCK) {
                if inner.ctx.count == 0 {
                    return Err(AxError::WouldBlock);
                }
                if flags.contains(EfdFlags::EFD_SEMAPHORE) {
                    cnt = 1;
                } else {
                    cnt = inner.ctx.count; // EFD_SEMAPHORE ?
                }
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

        if buf.len() < core::mem::size_of::<u64>() {
            return Err(AxError::InvalidInput);
        }

        let ucnt: u64 = u64::from_ne_bytes(buf.try_into().unwrap());
        if ucnt == u64::MAX {
            return Err(AxError::InvalidInput);
        }
        loop {
            let inner = self.inner.lock();
            if u64::MAX - inner.ctx.count > ucnt {
                break
            }
            let flags = EfdFlags::from_bits_truncate(inner.ctx.flags);
            if flags.contains(EfdFlags::EFD_NONBLOCK) {
                return Err(AxError::WouldBlock);
            }
            drop(inner);
            yield_now();
        }
        let mut inner = self.inner.lock();
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
        let inner = self.inner.lock();
        if inner.ctx.count < u64::MAX - 1 {
            true
        } else {
            false
        }
    }
    fn executable(&self) -> bool {
        false
    }
    fn get_status(&self) -> OpenFlags {
        let mut status = OpenFlags::RDWR;

        let inner = self.inner.lock();
        let flags = EfdFlags::from_bits_truncate(inner.ctx.flags);
        if flags.contains(EfdFlags::EFD_NONBLOCK) {
            status.insert(OpenFlags::NON_BLOCK);
        }
        if flags.contains(EfdFlags::EFD_CLOEXEC) {
            status.insert(OpenFlags::CLOEXEC);
        }

        status
    }
    fn get_type(&self) -> FileIOType {
        FileIOType::FileDesc
    }
}
