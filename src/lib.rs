use libc::malloc;
use minimp4_sys::{mp4_h26x_write_init, mp4_h26x_writer_t, MP4E__close, MP4E__open, MP4E_mux_t, MP4E__set_text_comment};
use std::convert::TryInto;
use std::ffi::CString;
use std::io::{Seek, SeekFrom, Write};
use std::mem::size_of;
use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;
use std::slice::from_raw_parts;

extern "C" {
    fn write_mp4(
        mp4wr: *const mp4_h26x_writer_t,
        fps: c_int,
        data: *const c_void,
        data_size: c_int,
    );
}

pub struct Mp4Muxer<W> {
    writer: W,
    muxer: *mut MP4E_mux_t,
    muxer_writer: *mut mp4_h26x_writer_t,
    str_buffer: Vec<CString>,
}

impl<W: Write + Seek> Mp4Muxer<W> {
    pub fn new(writer: W) -> Self {
        unsafe {
            Self {
                writer,
                muxer: null_mut(),
                muxer_writer: malloc(size_of::<mp4_h26x_writer_t>()) as *mut mp4_h26x_writer_t,
                str_buffer: Vec::new(),
            }
        }
    }

    pub fn init_video(&mut self, width: i32, height: i32, is_hevc: bool, track_name: &str) {
        self.str_buffer.push(CString::new(track_name).unwrap());
        unsafe {
            if self.muxer.is_null() {
                let self_ptr = self as *mut Self as *mut c_void;
                self.muxer = MP4E__open(0, self_ptr, Some(Self::write));
            }
            mp4_h26x_write_init(
                self.muxer_writer,
                self.muxer,
                width,
                height,
                if is_hevc { 1 } else { 0 },
                self.str_buffer.last().unwrap().as_ptr(),
            );
        }
    }

    pub fn write_video(&self, data: &[u8]) {
        unsafe {
            write_mp4(
                self.muxer_writer,
                60,
                data.as_ptr() as *const c_void,
                data.len().try_into().unwrap(),
            );
        }
    }

    pub fn write_comment(&mut self, comment: &str) {
        self.str_buffer.push(CString::new(comment).unwrap());
        unsafe {
            MP4E__set_text_comment(
                self.muxer,
                self.str_buffer.last().unwrap().as_ptr(),
            );
        }
    }
    
    pub fn close(&self) -> &W {
        unsafe {
            MP4E__close(self.muxer);
        }
        &self.writer
    }

    pub fn write_data(&mut self, offset: i64, buf: &[u8]) {
        self.writer.seek(SeekFrom::Start(offset as u64)).unwrap();
        self.writer.write_all(buf).unwrap();
    }

    extern "C" fn write(offset: i64, buffer: *const c_void, size: usize, token: *mut c_void) {
        let p_self = token as *mut Self;
        unsafe {
            let buf = from_raw_parts(buffer as *const u8, size);
            (&mut *p_self).write_data(offset, buf);
        }
    }
}
