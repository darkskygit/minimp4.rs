// Many functions in this file were copied from libc with the following license:

/*-
 * Copyright (c) 1990 The Regents of the University of California.
 * All rights reserved.
 *
 * This code is derived from software contributed to Berkeley by
 * Chris Torek.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the University nor the names of its contributors
 *    may be used to endorse or promote products derived from this software
 *    without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#![allow(clippy::missing_const_for_fn)]

use std::{
    alloc::Layout,
    any::Any,
    collections::HashMap,
    ffi::{CStr, CString},
    sync::{Arc, Mutex, Once},
};

type FILE = *mut std::ffi::c_void;

pub mod types {
    #[allow(non_camel_case_types)]
    pub type c_ushort = u16;
    #[allow(non_camel_case_types)]
    pub type c_int = i32;
    #[allow(non_camel_case_types)]
    pub type c_uint = u32;
    #[allow(non_camel_case_types)]
    pub type c_long = i64;
    #[allow(non_camel_case_types)]
    pub type c_ulong = u64;
    #[allow(non_camel_case_types)]
    pub type c_char = std::ffi::c_char;
    #[allow(non_camel_case_types)]
    pub type c_uchar = u8;

    #[allow(non_camel_case_types)]
    #[repr(u8)]
    pub enum c_void {
        __variant1,
        __variant2,
    }
}
use self::types::*;

#[allow(non_camel_case_types)]
type size_t = c_ulong;

#[derive(Default)]
struct Allocator {
    allocations: HashMap<*const c_void, Layout>,
}

impl Allocator {
    fn singleton() -> Arc<Mutex<Allocator>> {
        static START: Once = Once::new();
        static mut INSTANCE: Option<Arc<Mutex<Allocator>>> = None;
        START.call_once(|| unsafe {
            INSTANCE = Some(Arc::new(Mutex::new(Allocator::default())));
        });
        unsafe {
            let singleton = INSTANCE.as_ref().unwrap();
            singleton.clone()
        }
    }

    pub fn malloc(&mut self, size: usize) -> *mut c_void {
        if size > 0 {
            let layout = std::alloc::Layout::array::<u8>(size).unwrap().align_to(8).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };
            self.allocations.insert(ptr as *const c_void, layout);
            ptr.cast::<c_void>()
        } else {
            std::ptr::null_mut()
        }
    }

    pub unsafe fn realloc(&mut self, ptr: *const c_void, size: usize) -> *mut c_void {
        let new_memory = self.malloc(size);
        if !new_memory.is_null() {
            let layout = self.allocations.get(&ptr).unwrap();
            minimp4_memcpy(new_memory, ptr, layout.size() as size_t);
        }
        self.free(ptr);
        new_memory
    }

    #[allow(dead_code)]
    pub unsafe fn size(&mut self, ptr: *const c_void) -> usize {
        self.allocations.get(&ptr).unwrap().size()
    }

    pub unsafe fn free(&mut self, ptr: *const c_void) {
        if !ptr.is_null() {
            let layout = self.allocations.remove(&ptr).unwrap();
            unsafe { std::alloc::dealloc(ptr as *mut u8, layout) };
        }
    }

    #[allow(dead_code)]
    pub fn size_allocated(&self) -> usize {
        let mut size = 0;
        for allocation in self.allocations.values() {
            size += allocation.size();
        }
        size
    }
}

#[no_mangle]
pub unsafe extern "C" fn minimp4_strlen(str: *const c_char) -> c_ulong {
    let mut s: *const c_char;
    s = str;
    while *s != 0 {
        s = s.offset(1);
    }
    s.offset_from(str) as c_long as c_ulong
}

#[no_mangle]
unsafe extern "C" fn minimp4_malloc(size: size_t) -> *mut c_void {
    let singleton = Allocator::singleton();
    let mut allocator = singleton.lock().unwrap();
    allocator.malloc(size as usize)
}

#[no_mangle]
unsafe extern "C" fn minimp4_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    if !ptr.is_null() {
        let singleton = Allocator::singleton();
        let mut allocator = singleton.lock().unwrap();
        allocator.realloc(ptr, size as usize)
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
unsafe extern "C" fn minimp4_free(ptr: *mut c_void) {
    if !ptr.is_null() && ptr as usize != 1 {
        let singleton = Allocator::singleton();
        let mut allocator = singleton.lock().unwrap();
        allocator.free(ptr);
    }
}

#[no_mangle]
unsafe extern "C" fn minimp4_memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    std::ptr::copy_nonoverlapping(src, dest, n as usize);
    dest
}

#[no_mangle]
unsafe extern "C" fn minimp4_memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    for offset in 0..n {
        (*(s.cast::<u8>()).offset(offset as isize)) = c as u8;
    }
    s
}

#[cfg(test)]
mod tests {
    use std::ffi::CString;

    use super::{c_uint, minimp4_strlen, Allocator};

    #[test]
    fn allocator() {
        let mut allocator = Allocator::default();
        let mut allocations = vec![];
        for _ in 0..30 {
            let data = allocator.malloc(255).cast::<u8>();
            unsafe {
                for i in 0..255 {
                    *data.offset(i) = i as u8;
                }
                for i in 0..255 {
                    assert_eq!(*data.offset(i), i as u8);
                }
                assert_eq!(allocator.size(data as *const super::c_void), 255);
            }
            allocations.push(data);
        }
        assert_eq!(allocator.size_allocated(), 30 * 255);
        for allocation in &allocations {
            unsafe { allocator.free(*allocation as *const super::c_void) }
        }
        assert_eq!(allocator.size_allocated(), 0);
    }

    #[test]
    fn strlen() {
        unsafe {
            let empty = CString::new("").unwrap();
            assert_eq!(minimp4_strlen(empty.as_ptr()), 0);
            let hello_world = CString::new("hello world").unwrap();
            assert_eq!(minimp4_strlen(hello_world.as_ptr()), 11);
        }
    }
}
