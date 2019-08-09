use minimp4_sys::MP4E__open;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use std::os::raw::c_void;
use std::slice::from_raw_parts;

pub struct Mp4Muxer {
    buffer: Cursor<Vec<u8>>,
}

impl Mp4Muxer {
    pub fn new() -> Self {
        Self {
            buffer: Cursor::new(Vec::new()),
        }
    }

    pub fn init(&mut self) {
        unsafe {
            let self_ptr = self as *mut Self as *mut c_void;
            MP4E__open(0, self_ptr, Some(Self::write));
        }
    }

    pub fn write_data(&mut self, offset: i64, buf: &[u8]) {
        self.buffer.set_position(offset as u64);
        self.buffer.write_all(buf).unwrap();
    }

    extern "C" fn write(offset: i64, buffer: *const c_void, size: usize, token: *mut c_void) {
        let p_self = token as *mut Self;
        unsafe {
            let buf = from_raw_parts(buffer as *const u8, size);
            (&mut *p_self).write_data(offset, buf);
        }
    }
}
