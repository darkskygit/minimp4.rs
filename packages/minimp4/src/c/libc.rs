#![allow(clippy::missing_const_for_fn)]

pub mod types {
    #[allow(non_camel_case_types)]
    pub type c_ushort = libc::c_ushort;
    #[allow(non_camel_case_types)]
    pub type c_int = libc::c_int;
    #[allow(non_camel_case_types)]
    pub type c_uint = libc::c_uint;
    #[allow(non_camel_case_types)]
    pub type c_long = libc::c_long;
    #[allow(non_camel_case_types)]
    pub type c_longlong = libc::c_longlong;
    #[allow(non_camel_case_types)]
    pub type c_ulong = libc::c_ulong;
    #[allow(non_camel_case_types)]
    pub type c_char = libc::c_char;
    #[allow(non_camel_case_types)]
    pub type c_uchar = libc::c_uchar;
    #[allow(non_camel_case_types)]
    pub type c_void = libc::c_void;
}
use libc::fprintf;
use types::*;

#[allow(non_camel_case_types)]
type size_t = libc::size_t;

#[no_mangle]
unsafe extern "C" fn minimp4_strlen(s: *const c_char) -> size_t {
    libc::strlen(s)
}

#[no_mangle]
unsafe extern "C" fn minimp4_malloc(size: size_t) -> *mut c_void {
    libc::malloc(size)
}

#[no_mangle]
unsafe extern "C" fn minimp4_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    libc::realloc(ptr, size)
}

#[no_mangle]
unsafe extern "C" fn minimp4_free(ptr: *mut c_void) {
    libc::free(ptr)
}

#[no_mangle]
unsafe extern "C" fn minimp4_memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void {
    libc::memcpy(dest, src, n)
}

#[no_mangle]
unsafe extern "C" fn minimp4_memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    libc::memset(s, c, n)
}

#[no_mangle]
unsafe extern "C" fn __assert_fail(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    fprintf(
        libc::STDERR_FILENO as *mut libc::FILE,
        b"assertion \"%s\" failed: file \"%s\", line %d\n\0".as_ptr() as *const c_char,
        s,
        c,
        n,
    );
    libc::abort()
}
