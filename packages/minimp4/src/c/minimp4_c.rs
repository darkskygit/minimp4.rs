#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn minimp4_memcpy(__dest: *mut c_void, __src: *const c_void, __n: size_t) -> *mut c_void;
    fn minimp4_memset(__s: *mut c_void, __c: c_int, __n: size_t) -> *mut c_void;
    fn memcmp(_: *const c_void, _: *const c_void, _: c_ulong) -> c_int;
    fn __assert_fail(__assertion: *const c_char, __file: *const c_char, __line: c_uint, __function: *const c_char)
        -> !;
    fn memchr(_: *const c_void, _: c_int, _: c_ulong) -> *mut c_void;
    fn strdup(_: *const c_char) -> *mut c_char;
    fn minimp4_strlen(__s: *const c_char) -> size_t;
    fn minimp4_malloc(__size: size_t) -> *mut c_void;
    fn minimp4_realloc(__ptr: *mut c_void, __size: size_t) -> *mut c_void;
    fn minimp4_free(__ptr: *mut c_void);
}
pub type size_t = c_ulong;
pub type __uint8_t = c_uchar;
pub type __uint16_t = c_ushort;
pub type __uint32_t = c_uint;
pub type __int64_t = c_long;
pub type __uint64_t = c_ulong;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type boxsize_t = uint64_t;
pub type MP4D_file_offset_t = boxsize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MP4E_mux_tag {
    pub tracks: minimp4_vector_t,
    pub write_pos: int64_t,
    pub write_callback: Option<unsafe extern "C" fn(int64_t, *const c_void, size_t, *mut c_void) -> c_int>,
    pub token: *mut c_void,
    pub text_comment: *mut c_char,
    pub sequential_mode_flag: c_int,
    pub enable_fragmentation: c_int,
    pub fragments_count: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct minimp4_vector_t {
    pub data: *mut c_uchar,
    pub bytes: c_int,
    pub capacity: c_int,
}
pub type MP4E_mux_t = MP4E_mux_tag;
pub type track_media_kind_t = c_uint;
pub const e_private: track_media_kind_t = 2;
pub const e_video: track_media_kind_t = 1;
pub const e_audio: track_media_kind_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MP4E_track_t {
    pub object_type_indication: c_uint,
    pub language: [c_uchar; 4],
    pub track_media_kind: track_media_kind_t,
    pub time_scale: c_uint,
    pub default_duration: c_uint,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub a: C2RustUnnamed_1,
    pub v: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub width: c_int,
    pub height: c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub channelcount: c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MP4D_sample_to_chunk_t_tag {
    pub first_chunk: c_uint,
    pub samples_per_chunk: c_uint,
}
pub type MP4D_sample_to_chunk_t = MP4D_sample_to_chunk_t_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MP4D_track_t {
    pub sample_count: c_uint,
    pub dsi: *mut c_uchar,
    pub dsi_bytes: c_uint,
    pub object_type_indication: c_uint,
    pub handler_type: c_uint,
    pub duration_hi: c_uint,
    pub duration_lo: c_uint,
    pub timescale: c_uint,
    pub avg_bitrate_bps: c_uint,
    pub language: [c_uchar; 4],
    pub stream_type: c_uint,
    pub SampleDescription: C2RustUnnamed_2,
    pub entry_size: *mut c_uint,
    pub sample_to_chunk_count: c_uint,
    pub sample_to_chunk: *mut MP4D_sample_to_chunk_t_tag,
    pub chunk_count: c_uint,
    pub chunk_offset: *mut MP4D_file_offset_t,
    pub timestamp: *mut c_uint,
    pub duration: *mut c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub audio: C2RustUnnamed_4,
    pub video: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub width: c_uint,
    pub height: c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub channelcount: c_uint,
    pub samplerate_hz: c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MP4D_demux_tag {
    pub read_pos: int64_t,
    pub read_size: int64_t,
    pub track: *mut MP4D_track_t,
    pub read_callback: Option<unsafe extern "C" fn(int64_t, *mut c_void, size_t, *mut c_void) -> c_int>,
    pub token: *mut c_void,
    pub track_count: c_uint,
    pub duration_hi: c_uint,
    pub duration_lo: c_uint,
    pub timescale: c_uint,
    pub tag: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub title: *mut c_uchar,
    pub artist: *mut c_uchar,
    pub album: *mut c_uchar,
    pub year: *mut c_uchar,
    pub comment: *mut c_uchar,
    pub genre: *mut c_uchar,
}
pub type MP4D_demux_t = MP4D_demux_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct h264_sps_id_patcher_t {
    pub sps_cache: [*mut c_void; 32],
    pub pps_cache: [*mut c_void; 256],
    pub sps_bytes: [c_int; 32],
    pub pps_bytes: [c_int; 256],
    pub map_sps: [c_int; 32],
    pub map_pps: [c_int; 256],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp4_h26x_writer_tag {
    pub sps_patcher: h264_sps_id_patcher_t,
    pub mux: *mut MP4E_mux_t,
    pub mux_track_id: c_int,
    pub is_hevc: c_int,
    pub need_vps: c_int,
    pub need_sps: c_int,
    pub need_pps: c_int,
    pub need_idr: c_int,
}
pub type mp4_h26x_writer_t = mp4_h26x_writer_tag;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct track_t {
    pub info: MP4E_track_t,
    pub smpl: minimp4_vector_t,
    pub pending_sample: minimp4_vector_t,
    pub vsps: minimp4_vector_t,
    pub vpps: minimp4_vector_t,
    pub vvps: minimp4_vector_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sample_t {
    pub size: boxsize_t,
    pub offset: boxsize_t,
    pub duration: c_uint,
    pub flag_random_access: c_uint,
}
pub const BOX_mdat: C2RustUnnamed_10 = 1835295092;
pub const BOX_trun: C2RustUnnamed_10 = 1953658222;
pub const BOX_tfhd: C2RustUnnamed_10 = 1952868452;
pub const BOX_traf: C2RustUnnamed_10 = 1953653094;
pub const BOX_mfhd: C2RustUnnamed_10 = 1835427940;
pub const BOX_moof: C2RustUnnamed_10 = 1836019558;
pub type C2RustUnnamed_6 = c_uint;
pub const default_sample_flags_present: C2RustUnnamed_6 = 32;
pub const default_sample_duration_present: C2RustUnnamed_6 = 8;
pub const BOX_trex: C2RustUnnamed_10 = 1953654136;
pub const BOX_mehd: C2RustUnnamed_10 = 1835362404;
pub const BOX_mvex: C2RustUnnamed_10 = 1836475768;
pub const BOX_data: C2RustUnnamed_10 = 1684108385;
pub const BOX_ccmt: C2RustUnnamed_10 = 2841865588;
pub const BOX_ilst: C2RustUnnamed_10 = 1768715124;
pub const BOX_hdlr: C2RustUnnamed_10 = 1751411826;
pub const BOX_meta: C2RustUnnamed_10 = 1835365473;
pub const BOX_udta: C2RustUnnamed_10 = 1969517665;
pub const BOX_stss: C2RustUnnamed_10 = 1937011571;
pub const BOX_co64: C2RustUnnamed_10 = 1668232756;
pub const BOX_stco: C2RustUnnamed_10 = 1937007471;
pub const BOX_stsz: C2RustUnnamed_10 = 1937011578;
pub const BOX_stsc: C2RustUnnamed_10 = 1937011555;
pub const BOX_stts: C2RustUnnamed_10 = 1937011827;
pub const BOX_hvcC: C2RustUnnamed_10 = 1752589123;
pub const BOX_avcC: C2RustUnnamed_10 = 1635148611;
pub const BOX_hvc1: C2RustUnnamed_10 = 1752589105;
pub const BOX_avc1: C2RustUnnamed_10 = 1635148593;
pub const BOX_esds: C2RustUnnamed_10 = 1702061171;
pub const BOX_mp4s: C2RustUnnamed_10 = 1836070003;
pub const BOX_mp4a: C2RustUnnamed_10 = 1836069985;
pub const BOX_stsd: C2RustUnnamed_10 = 1937011556;
pub const BOX_stbl: C2RustUnnamed_10 = 1937007212;
pub const BOX_url: C2RustUnnamed_10 = 1970433056;
pub const BOX_dref: C2RustUnnamed_10 = 1685218662;
pub const BOX_dinf: C2RustUnnamed_10 = 1684631142;
pub const BOX_vmhd: C2RustUnnamed_10 = 1986881636;
pub const BOX_smhd: C2RustUnnamed_10 = 1936549988;
pub const BOX_minf: C2RustUnnamed_10 = 1835626086;
pub const BOX_mdhd: C2RustUnnamed_10 = 1835296868;
pub const BOX_mdia: C2RustUnnamed_10 = 1835297121;
pub const BOX_tkhd: C2RustUnnamed_10 = 1953196132;
pub const BOX_trak: C2RustUnnamed_10 = 1953653099;
pub const BOX_mvhd: C2RustUnnamed_10 = 1836476516;
pub const BOX_moov: C2RustUnnamed_10 = 1836019574;
pub const BOX_free: C2RustUnnamed_10 = 1718773093;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bit_reader_t {
    pub cache: c_uint,
    pub cache_free_bits: c_int,
    pub buf: *const uint16_t,
    pub origin: *const uint16_t,
    pub origin_bytes: c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bs_t {
    pub shift: c_int,
    pub cache: uint32_t,
    pub buf: *mut bs_item_t,
    pub origin: *mut bs_item_t,
}
pub type bs_item_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub bytes: boxsize_t,
    pub format: boxtype_t,
}
pub type boxtype_t = c_uint;
pub const BOX_OD: boxtype_t = 1;
pub const BOX_ATOM: boxtype_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub name: uint32_t,
    pub type_0: boxtype_t,
}
pub const BOX_mp4v: C2RustUnnamed_10 = 1836070006;
pub const BOX_tref: C2RustUnnamed_10 = 1953654118;
pub const OD_DSI: C2RustUnnamed_10 = 606348341;
pub const OD_DCD: C2RustUnnamed_10 = 606348340;
pub const OD_ESD: C2RustUnnamed_10 = 606348339;
pub const BOX_cgen: C2RustUnnamed_10 = 2842125678;
pub const BOX_cday: C2RustUnnamed_10 = 2841928057;
pub const BOX_cnam: C2RustUnnamed_10 = 2842583405;
pub const BOX_cART: C2RustUnnamed_10 = 2839630420;
pub const BOX_calb: C2RustUnnamed_10 = 2841734242;
pub const BOX_btrt: C2RustUnnamed_10 = 1651798644;
pub const BOX_ctts: C2RustUnnamed_10 = 1668576371;
pub const BOX_stz2: C2RustUnnamed_10 = 1937013298;
pub const OD_BASE: C2RustUnnamed_10 = 606348336;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub name: uint32_t,
    pub max_version: c_uint,
    pub use_track_flag: c_uint,
}
pub type C2RustUnnamed_10 = c_uint;
pub const BOX_yrrc: C2RustUnnamed_10 = 2037543523;
pub const BOX_albm: C2RustUnnamed_10 = 1634493037;
pub const BOX_name: C2RustUnnamed_10 = 1851878757;
pub const BOX_mean: C2RustUnnamed_10 = 1835360622;
pub const BOX_perf: C2RustUnnamed_10 = 1885696614;
pub const BOX_dscp: C2RustUnnamed_10 = 1685283696;
pub const BOX_titl: C2RustUnnamed_10 = 1953068140;
pub const BOX_auth: C2RustUnnamed_10 = 1635087464;
pub const BOX_gnre: C2RustUnnamed_10 = 1735291493;
pub const BOX_pgap: C2RustUnnamed_10 = 1885823344;
pub const BOX_purd: C2RustUnnamed_10 = 1886745188;
pub const BOX_tvsn: C2RustUnnamed_10 = 1953919854;
pub const BOX_tvsh: C2RustUnnamed_10 = 1953919848;
pub const BOX_tvnn: C2RustUnnamed_10 = 1953918574;
pub const BOX_tves: C2RustUnnamed_10 = 1953916275;
pub const BOX_tven: C2RustUnnamed_10 = 1953916270;
pub const BOX_clyr: C2RustUnnamed_10 = 2842458482;
pub const BOX_desc: C2RustUnnamed_10 = 1684370275;
pub const BOX_egid: C2RustUnnamed_10 = 1701276004;
pub const BOX_purl: C2RustUnnamed_10 = 1886745196;
pub const BOX_keyw: C2RustUnnamed_10 = 1801812343;
pub const BOX_catg: C2RustUnnamed_10 = 1667331175;
pub const BOX_pcst: C2RustUnnamed_10 = 1885565812;
pub const BOX_stik: C2RustUnnamed_10 = 1937009003;
pub const BOX_cgrp: C2RustUnnamed_10 = 2842129008;
pub const BOX_rtng: C2RustUnnamed_10 = 1920233063;
pub const BOX_covr: C2RustUnnamed_10 = 1668249202;
pub const BOX_cpil: C2RustUnnamed_10 = 1668311404;
pub const BOX_tmpo: C2RustUnnamed_10 = 1953329263;
pub const BOX_ctoo: C2RustUnnamed_10 = 2842980207;
pub const BOX_cwrt: C2RustUnnamed_10 = 2843177588;
pub const BOX_disk: C2RustUnnamed_10 = 1684632427;
pub const BOX_trkn: C2RustUnnamed_10 = 1953655662;
pub const BOX_aART: C2RustUnnamed_10 = 1631670868;
pub const BOX_cart: C2RustUnnamed_10 = 2841735796;
pub const OD_SLC: C2RustUnnamed_10 = 606348342;
pub const BOX_tfdt: C2RustUnnamed_10 = 1952867444;
pub const BOX_d263: C2RustUnnamed_10 = 1681012275;
pub const BOX_s263: C2RustUnnamed_10 = 1932670515;
pub const BOX_damr: C2RustUnnamed_10 = 1684106610;
pub const BOX_sawb: C2RustUnnamed_10 = 1935767394;
pub const BOX_samr: C2RustUnnamed_10 = 1935764850;
pub const BOX_hev1: C2RustUnnamed_10 = 1751479857;
pub const BOX_seib: C2RustUnnamed_10 = 1936025954;
pub const BOX_m4ds: C2RustUnnamed_10 = 1832150131;
pub const BOX_svcC: C2RustUnnamed_10 = 1937138499;
pub const BOX_svc1: C2RustUnnamed_10 = 1937138481;
pub const BOX_avc2: C2RustUnnamed_10 = 1635148594;
pub const BOX_ipir: C2RustUnnamed_10 = 1768974706;
pub const BOX_sync: C2RustUnnamed_10 = 1937337955;
pub const BOX_nmhd: C2RustUnnamed_10 = 1852663908;
pub const BOX_mpod: C2RustUnnamed_10 = 1836085092;
pub const BOX_odhd: C2RustUnnamed_10 = 1868851300;
pub const BOX_iods: C2RustUnnamed_10 = 1768907891;
pub const BOX_dpnd: C2RustUnnamed_10 = 1685089892;
pub const BOX_sdhd: C2RustUnnamed_10 = 1935960164;
pub const BOX_padb: C2RustUnnamed_10 = 1885430882;
pub const BOX_ftyp: C2RustUnnamed_10 = 1718909296;
pub const BOX_gnra: C2RustUnnamed_10 = 1735291489;
pub const BOX_gnrv: C2RustUnnamed_10 = 1735291510;
pub const BOX_urn: C2RustUnnamed_10 = 1970433568;
pub const BOX_skip: C2RustUnnamed_10 = 1936419184;
pub const BOX_stsh: C2RustUnnamed_10 = 1937011560;
pub const BOX_hint: C2RustUnnamed_10 = 1751740020;
pub const BOX_hmhd: C2RustUnnamed_10 = 1752000612;
pub const BOX_uuid: C2RustUnnamed_10 = 1970628964;
pub const BOX_elst: C2RustUnnamed_10 = 1701606260;
pub const BOX_edts: C2RustUnnamed_10 = 1701082227;
pub const BOX_stdp: C2RustUnnamed_10 = 1937007728;
pub const BOX_urn_: C2RustUnnamed_10 = 1970433568;
pub const BOX_url_: C2RustUnnamed_10 = 1970433056;
pub const BOX_cprt: C2RustUnnamed_10 = 1668313716;
pub const BOX_crhd: C2RustUnnamed_10 = 1668442212;
#[no_mangle]
pub unsafe extern "C" fn isspace_(mut x: c_int) -> c_int {
    (x <= 32 as c_int) as c_int
}
unsafe extern "C" fn get_nal_size(mut buf: *mut uint8_t, mut size: size_t) -> size_t {
    let mut pos: size_t = 3 as c_int as size_t;
    while size.wrapping_sub(pos) > 3 as c_int as c_ulong {
        if *buf.offset(pos as isize) as c_int == 0 as c_int
            && *buf.offset(pos.wrapping_add(1 as c_int as c_ulong) as isize) as c_int == 0 as c_int
            && *buf.offset(pos.wrapping_add(2 as c_int as c_ulong) as isize) as c_int == 1 as c_int
        {
            return pos;
        }
        if *buf.offset(pos as isize) as c_int == 0 as c_int
            && *buf.offset(pos.wrapping_add(1 as c_int as c_ulong) as isize) as c_int == 0 as c_int
            && *buf.offset(pos.wrapping_add(2 as c_int as c_ulong) as isize) as c_int == 0 as c_int
            && *buf.offset(pos.wrapping_add(3 as c_int as c_ulong) as isize) as c_int == 1 as c_int
        {
            return pos;
        }
        pos = pos.wrapping_add(1);
    }
    size
}
#[no_mangle]
pub unsafe extern "C" fn write_mp4(
    mut mp4wr: *mut mp4_h26x_writer_t,
    mut fps: c_int,
    mut data: *const uint8_t,
    mut data_size: size_t,
) {
    while data_size > 0 as c_int as c_ulong {
        let mut nal_size: size_t = get_nal_size(data as *mut uint8_t, data_size);
        if nal_size < 4 as c_int as c_ulong {
            data = data.offset(1 as c_int as isize);
            data_size = (data_size as c_ulong).wrapping_sub(1 as c_int as c_ulong) as size_t as size_t;
        } else {
            mp4_h26x_write_nal(mp4wr, data, nal_size as c_int, (90000 as c_int / fps) as c_uint);
            data = data.offset(nal_size as isize);
            data_size = (data_size as c_ulong).wrapping_sub(nal_size) as size_t as size_t;
        }
    }
}
static mut box_ftyp: [c_uchar; 24] = [
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0x18 as c_int as c_uchar,
    'f' as i32 as c_uchar,
    't' as i32 as c_uchar,
    'y' as i32 as c_uchar,
    'p' as i32 as c_uchar,
    'm' as i32 as c_uchar,
    'p' as i32 as c_uchar,
    '4' as i32 as c_uchar,
    '2' as i32 as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    0 as c_int as c_uchar,
    'm' as i32 as c_uchar,
    'p' as i32 as c_uchar,
    '4' as i32 as c_uchar,
    '2' as i32 as c_uchar,
    'i' as i32 as c_uchar,
    's' as i32 as c_uchar,
    'o' as i32 as c_uchar,
    'm' as i32 as c_uchar,
];
unsafe extern "C" fn minimp4_vector_init(mut h: *mut minimp4_vector_t, mut capacity: c_int) -> c_int {
    (*h).bytes = 0 as c_int;
    (*h).capacity = capacity;
    (*h).data = if capacity != 0 {
        minimp4_malloc(capacity as size_t) as *mut c_uchar
    } else {
        std::ptr::null_mut::<c_uchar>()
    };
    (capacity == 0 || !((*h).data).is_null()) as c_int
}
unsafe extern "C" fn minimp4_vector_reset(mut h: *mut minimp4_vector_t) {
    if !((*h).data).is_null() {
        minimp4_free((*h).data as *mut c_void);
    }
    minimp4_memset(
        h as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<minimp4_vector_t>() as c_ulong,
    );
}
unsafe extern "C" fn minimp4_vector_grow(mut h: *mut minimp4_vector_t, mut bytes: c_int) -> c_int {
    let mut p: *mut c_void = std::ptr::null_mut::<c_void>();
    let mut new_size: c_int = (*h).capacity * 2 as c_int + 1024 as c_int;
    if new_size < (*h).capacity + bytes {
        new_size = (*h).capacity + bytes + 1024 as c_int;
    }
    p = minimp4_realloc((*h).data as *mut c_void, new_size as size_t);
    if p.is_null() {
        return 0 as c_int;
    }
    (*h).data = p as *mut c_uchar;
    (*h).capacity = new_size;
    1 as c_int
}
unsafe extern "C" fn minimp4_vector_alloc_tail(mut h: *mut minimp4_vector_t, mut bytes: c_int) -> *mut c_uchar {
    let mut p: *mut c_uchar = std::ptr::null_mut::<c_uchar>();
    if ((*h).data).is_null() && minimp4_vector_init(h, 2 as c_int * bytes + 1024 as c_int) == 0 {
        return std::ptr::null_mut::<c_uchar>();
    }
    if (*h).capacity - (*h).bytes < bytes && minimp4_vector_grow(h, bytes) == 0 {
        return std::ptr::null_mut::<c_uchar>();
    }
    if !((*h).data).is_null() {
    } else {
        __assert_fail(
            b"h->data\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            787 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 66], &[c_char; 66]>(
                b"unsigned char *minimp4_vector_alloc_tail(minimp4_vector_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if !((*h).data).is_null() {
    } else {
        __assert_fail(
            b"h->data\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            787 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 66], &[c_char; 66]>(
                b"unsigned char *minimp4_vector_alloc_tail(minimp4_vector_t *, int)\0",
            ))
            .as_ptr(),
        );
    };
    if (*h).capacity - (*h).bytes >= bytes {
    } else {
        __assert_fail(
            b"(h->capacity - h->bytes) >= bytes\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            788 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 66], &[c_char; 66]>(
                b"unsigned char *minimp4_vector_alloc_tail(minimp4_vector_t *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if (*h).capacity - (*h).bytes >= bytes {
    } else {
        __assert_fail(
            b"(h->capacity - h->bytes) >= bytes\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            788 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 66], &[c_char; 66]>(
                b"unsigned char *minimp4_vector_alloc_tail(minimp4_vector_t *, int)\0",
            ))
            .as_ptr(),
        );
    };
    p = ((*h).data).offset((*h).bytes as isize);
    (*h).bytes += bytes;
    p
}
unsafe extern "C" fn minimp4_vector_put(
    mut h: *mut minimp4_vector_t,
    mut buf: *const c_void,
    mut bytes: c_int,
) -> *mut c_uchar {
    let mut tail: *mut c_uchar = minimp4_vector_alloc_tail(h, bytes);
    if !tail.is_null() {
        minimp4_memcpy(tail as *mut c_void, buf, bytes as size_t);
    }
    tail
}
#[no_mangle]
pub unsafe extern "C" fn MP4E_open(
    mut sequential_mode_flag: c_int,
    mut enable_fragmentation: c_int,
    mut token: *mut c_void,
    mut write_callback: Option<unsafe extern "C" fn(int64_t, *const c_void, size_t, *mut c_void) -> c_int>,
) -> *mut MP4E_mux_t {
    if write_callback.expect("non-null function pointer")(
        0 as c_int as int64_t,
        box_ftyp.as_ptr() as *const c_void,
        ::core::mem::size_of::<[c_uchar; 24]>() as c_ulong,
        token,
    ) != 0
    {
        return std::ptr::null_mut::<MP4E_mux_t>();
    }
    let mut mux: *mut MP4E_mux_t = minimp4_malloc(::core::mem::size_of::<MP4E_mux_t>() as c_ulong) as *mut MP4E_mux_t;
    if mux.is_null() {
        return mux;
    }
    (*mux).sequential_mode_flag = (sequential_mode_flag != 0 || enable_fragmentation != 0) as c_int;
    (*mux).enable_fragmentation = enable_fragmentation;
    (*mux).fragments_count = 0 as c_int;
    (*mux).write_callback = write_callback;
    (*mux).token = token;
    (*mux).text_comment = std::ptr::null_mut::<c_char>();
    (*mux).write_pos = ::core::mem::size_of::<[c_uchar; 24]>() as c_ulong as int64_t;
    if (*mux).sequential_mode_flag == 0 {
        if ((*mux).write_callback).expect("non-null function pointer")(
            (*mux).write_pos,
            box_ftyp.as_ptr() as *const c_void,
            8 as c_int as size_t,
            (*mux).token,
        ) != 0
        {
            minimp4_free(mux as *mut c_void);
            return std::ptr::null_mut::<MP4E_mux_t>();
        }
        (*mux).write_pos += 16 as c_int as c_long;
    }
    minimp4_vector_init(
        &mut (*mux).tracks,
        (2 as c_int as c_ulong).wrapping_mul(::core::mem::size_of::<track_t>() as c_ulong) as c_int,
    );
    mux
}
#[no_mangle]
pub unsafe extern "C" fn MP4E_add_track(mut mux: *mut MP4E_mux_t, mut track_data: *const MP4E_track_t) -> c_int {
    let mut tr: *mut track_t = std::ptr::null_mut::<track_t>();
    let mut ntr: c_int =
        ((*mux).tracks.bytes as c_ulong).wrapping_div(::core::mem::size_of::<track_t>() as c_ulong) as c_int;
    if mux.is_null() || track_data.is_null() {
        return -(1 as c_int);
    }
    tr = minimp4_vector_alloc_tail(
        &mut (*mux).tracks,
        ::core::mem::size_of::<track_t>() as c_ulong as c_int,
    ) as *mut track_t;
    if tr.is_null() {
        return -(2 as c_int);
    }
    minimp4_memset(
        tr as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<track_t>() as c_ulong,
    );
    minimp4_memcpy(
        &mut (*tr).info as *mut MP4E_track_t as *mut c_void,
        track_data as *const c_void,
        ::core::mem::size_of::<MP4E_track_t>() as c_ulong,
    );
    if minimp4_vector_init(&mut (*tr).smpl, 256 as c_int) == 0 {
        return -(2 as c_int);
    }
    minimp4_vector_init(&mut (*tr).vsps, 0 as c_int);
    minimp4_vector_init(&mut (*tr).vpps, 0 as c_int);
    minimp4_vector_init(&mut (*tr).pending_sample, 0 as c_int);
    ntr
}
unsafe extern "C" fn append_mem(mut v: *mut minimp4_vector_t, mut mem: *const c_void, mut bytes: c_int) -> c_int {
    let mut i: c_int = 0;
    let mut size: [c_uchar; 2] = [0; 2];
    let mut p: *const c_uchar = (*v).data;
    i = 0 as c_int;
    while (i + 2 as c_int) < (*v).bytes {
        let mut cb: c_int =
            *p.offset(i as isize) as c_int * 256 as c_int + *p.offset((i + 1 as c_int) as isize) as c_int;
        if cb == bytes
            && memcmp(
                p.offset(i as isize).offset(2 as c_int as isize) as *const c_void,
                mem,
                cb as c_ulong,
            ) == 0
        {
            return 1 as c_int;
        }
        i += 2 as c_int + cb;
    }
    size[0 as c_int as usize] = (bytes >> 8 as c_int) as c_uchar;
    size[1 as c_int as usize] = bytes as c_uchar;
    (!(minimp4_vector_put(v, size.as_mut_ptr() as *const c_void, 2 as c_int)).is_null()
        && !(minimp4_vector_put(v, mem, bytes)).is_null()) as c_int
}
unsafe extern "C" fn items_count(mut v: *mut minimp4_vector_t) -> c_int {
    let mut i: c_int = 0;
    let mut count: c_int = 0 as c_int;
    let mut p: *const c_uchar = (*v).data;
    i = 0 as c_int;
    while (i + 2 as c_int) < (*v).bytes {
        let mut cb: c_int =
            *p.offset(i as isize) as c_int * 256 as c_int + *p.offset((i + 1 as c_int) as isize) as c_int;
        count += 1;
        i += 2 as c_int + cb;
    }
    count
}
#[no_mangle]
pub unsafe extern "C" fn MP4E_set_dsi(
    mut mux: *mut MP4E_mux_t,
    mut track_id: c_int,
    mut dsi: *const c_void,
    mut bytes: c_int,
) -> c_int {
    let mut tr: *mut track_t = ((*mux).tracks.data as *mut track_t).offset(track_id as isize);
    if (*tr).info.track_media_kind as c_uint == e_audio as c_int as c_uint
        || (*tr).info.track_media_kind as c_uint == e_private as c_int as c_uint
    {
    } else {
        __assert_fail(
            b"tr->info.track_media_kind == e_audio || tr->info.track_media_kind == e_private\0" as *const u8
                as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            905 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 55], &[c_char; 55]>(
                b"int MP4E_set_dsi(MP4E_mux_t *, int, const void *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if (*tr).info.track_media_kind as c_uint == e_audio as c_int as c_uint
        || (*tr).info.track_media_kind as c_uint == e_private as c_int as c_uint
    {
    } else {
        __assert_fail(
            b"tr->info.track_media_kind == e_audio || tr->info.track_media_kind == e_private\0" as *const u8
                as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            905 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 55], &[c_char; 55]>(
                b"int MP4E_set_dsi(MP4E_mux_t *, int, const void *, int)\0",
            ))
            .as_ptr(),
        );
    };
    if (*tr).vsps.bytes != 0 {
        return -(4 as c_int);
    }
    if append_mem(&mut (*tr).vsps, dsi, bytes) != 0 {
        0 as c_int
    } else {
        -(2 as c_int)
    }
}
#[no_mangle]
pub unsafe extern "C" fn MP4E_set_vps(
    mut mux: *mut MP4E_mux_t,
    mut track_id: c_int,
    mut vps: *const c_void,
    mut bytes: c_int,
) -> c_int {
    let mut tr: *mut track_t = ((*mux).tracks.data as *mut track_t).offset(track_id as isize);
    if (*tr).info.track_media_kind as c_uint == e_video as c_int as c_uint {
    } else {
        __assert_fail(
            b"tr->info.track_media_kind == e_video\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            915 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 55], &[c_char; 55]>(
                b"int MP4E_set_vps(MP4E_mux_t *, int, const void *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if (*tr).info.track_media_kind as c_uint == e_video as c_int as c_uint {
    } else {
        __assert_fail(
            b"tr->info.track_media_kind == e_video\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            915 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 55], &[c_char; 55]>(
                b"int MP4E_set_vps(MP4E_mux_t *, int, const void *, int)\0",
            ))
            .as_ptr(),
        );
    };
    if append_mem(&mut (*tr).vvps, vps, bytes) != 0 {
        0 as c_int
    } else {
        -(2 as c_int)
    }
}
#[no_mangle]
pub unsafe extern "C" fn MP4E_set_sps(
    mut mux: *mut MP4E_mux_t,
    mut track_id: c_int,
    mut sps: *const c_void,
    mut bytes: c_int,
) -> c_int {
    let mut tr: *mut track_t = ((*mux).tracks.data as *mut track_t).offset(track_id as isize);
    if (*tr).info.track_media_kind as c_uint == e_video as c_int as c_uint {
    } else {
        __assert_fail(
            b"tr->info.track_media_kind == e_video\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            922 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 55], &[c_char; 55]>(
                b"int MP4E_set_sps(MP4E_mux_t *, int, const void *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if (*tr).info.track_media_kind as c_uint == e_video as c_int as c_uint {
    } else {
        __assert_fail(
            b"tr->info.track_media_kind == e_video\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            922 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 55], &[c_char; 55]>(
                b"int MP4E_set_sps(MP4E_mux_t *, int, const void *, int)\0",
            ))
            .as_ptr(),
        );
    };
    if append_mem(&mut (*tr).vsps, sps, bytes) != 0 {
        0 as c_int
    } else {
        -(2 as c_int)
    }
}
#[no_mangle]
pub unsafe extern "C" fn MP4E_set_pps(
    mut mux: *mut MP4E_mux_t,
    mut track_id: c_int,
    mut pps: *const c_void,
    mut bytes: c_int,
) -> c_int {
    let mut tr: *mut track_t = ((*mux).tracks.data as *mut track_t).offset(track_id as isize);
    if (*tr).info.track_media_kind as c_uint == e_video as c_int as c_uint {
    } else {
        __assert_fail(
            b"tr->info.track_media_kind == e_video\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            929 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 55], &[c_char; 55]>(
                b"int MP4E_set_pps(MP4E_mux_t *, int, const void *, int)\0",
            ))
            .as_ptr(),
        );
    }
    if (*tr).info.track_media_kind as c_uint == e_video as c_int as c_uint {
    } else {
        __assert_fail(
            b"tr->info.track_media_kind == e_video\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            929 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 55], &[c_char; 55]>(
                b"int MP4E_set_pps(MP4E_mux_t *, int, const void *, int)\0",
            ))
            .as_ptr(),
        );
    };
    if append_mem(&mut (*tr).vpps, pps, bytes) != 0 {
        0 as c_int
    } else {
        -(2 as c_int)
    }
}
unsafe extern "C" fn get_duration(mut tr: *const track_t) -> c_uint {
    let mut i: c_uint = 0;
    let mut sum_duration: c_uint = 0 as c_int as c_uint;
    let mut s: *const sample_t = (*tr).smpl.data as *const sample_t;
    i = 0 as c_int as c_uint;
    while (i as c_ulong) < ((*tr).smpl.bytes as c_ulong).wrapping_div(::core::mem::size_of::<sample_t>() as c_ulong) {
        sum_duration = sum_duration.wrapping_add((*s.offset(i as isize)).duration);
        i = i.wrapping_add(1);
    }
    sum_duration
}
unsafe extern "C" fn write_pending_data(mut mux: *mut MP4E_mux_t, mut tr: *mut track_t) -> c_int {
    if (*tr).pending_sample.bytes > 0 as c_int
        && (*tr).smpl.bytes as c_ulong >= ::core::mem::size_of::<sample_t>() as c_ulong
    {
        let mut smpl_desc: *mut sample_t = std::ptr::null_mut::<sample_t>();
        let mut base: [c_uchar; 8] = [0; 8];
        let mut p: *mut c_uchar = base.as_mut_ptr();
        if (*mux).sequential_mode_flag != 0 {
        } else {
            __assert_fail(
                b"mux->sequential_mode_flag\0" as *const u8 as *const c_char,
                b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                953 as c_int as c_uint,
                (*::core::mem::transmute::<&[u8; 48], &[c_char; 48]>(
                    b"int write_pending_data(MP4E_mux_t *, track_t *)\0",
                ))
                .as_ptr(),
            );
        }
        if (*mux).sequential_mode_flag != 0 {
        } else {
            __assert_fail(
                b"mux->sequential_mode_flag\0" as *const u8 as *const c_char,
                b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                953 as c_int as c_uint,
                (*::core::mem::transmute::<&[u8; 48], &[c_char; 48]>(
                    b"int write_pending_data(MP4E_mux_t *, track_t *)\0",
                ))
                .as_ptr(),
            );
        };
        if (*mux).sequential_mode_flag != 0 {
        } else {
            __assert_fail(
                b"mux->sequential_mode_flag\0" as *const u8 as *const c_char,
                b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                956 as c_int as c_uint,
                (*::core::mem::transmute::<&[u8; 48], &[c_char; 48]>(
                    b"int write_pending_data(MP4E_mux_t *, track_t *)\0",
                ))
                .as_ptr(),
            );
        }
        if (*mux).sequential_mode_flag != 0 {
        } else {
            __assert_fail(
                b"mux->sequential_mode_flag\0" as *const u8 as *const c_char,
                b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                956 as c_int as c_uint,
                (*::core::mem::transmute::<&[u8; 48], &[c_char; 48]>(
                    b"int write_pending_data(MP4E_mux_t *, track_t *)\0",
                ))
                .as_ptr(),
            );
        };
        let fresh0 = p;
        p = p.offset(1);
        *fresh0 = (((*tr).pending_sample.bytes + 8 as c_int) >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh1 = p;
        p = p.offset(1);
        *fresh1 = (((*tr).pending_sample.bytes + 8 as c_int) >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = (((*tr).pending_sample.bytes + 8 as c_int) >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = (((*tr).pending_sample.bytes + 8 as c_int) >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh4 = p;
        p = p.offset(1);
        *fresh4 = (BOX_mdat as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh5 = p;
        p = p.offset(1);
        *fresh5 = (BOX_mdat as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh6 = p;
        p = p.offset(1);
        *fresh6 = (BOX_mdat as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh7 = p;
        p = p.offset(1);
        *fresh7 = (BOX_mdat as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let mut err: c_int = ((*mux).write_callback).expect("non-null function pointer")(
            (*mux).write_pos,
            base.as_mut_ptr() as *const c_void,
            p.offset_from(base.as_mut_ptr()) as c_long as size_t,
            (*mux).token,
        );
        if err != 0 {
            return err;
        }
        (*mux).write_pos += p.offset_from(base.as_mut_ptr()) as c_long;
        smpl_desc =
            (minimp4_vector_alloc_tail(&mut (*tr).smpl, 0 as c_int) as *mut sample_t).offset(-(1 as c_int as isize));
        (*smpl_desc).size = (*tr).pending_sample.bytes as boxsize_t;
        (*smpl_desc).offset = (*mux).write_pos as boxsize_t;
        let mut err_0: c_int = ((*mux).write_callback).expect("non-null function pointer")(
            (*mux).write_pos,
            (*tr).pending_sample.data as *const c_void,
            (*tr).pending_sample.bytes as size_t,
            (*mux).token,
        );
        if err_0 != 0 {
            return err_0;
        }
        (*mux).write_pos += (*tr).pending_sample.bytes as c_long;
        (*tr).pending_sample.bytes = 0 as c_int;
    }
    0 as c_int
}
unsafe extern "C" fn add_sample_descriptor(
    mut mux: *mut MP4E_mux_t,
    mut tr: *mut track_t,
    mut data_bytes: c_int,
    mut duration: c_int,
    mut kind: c_int,
) -> c_int {
    let mut smp: sample_t = sample_t {
        size: 0,
        offset: 0,
        duration: 0,
        flag_random_access: 0,
    };
    smp.size = data_bytes as boxsize_t;
    smp.offset = (*mux).write_pos as boxsize_t;
    smp.duration = if duration != 0 {
        duration as c_uint
    } else {
        (*tr).info.default_duration
    };
    smp.flag_random_access = (kind == 1 as c_int) as c_int as c_uint;
    (std::ptr::null_mut::<c_void>() as *mut c_uchar
        != minimp4_vector_put(
            &mut (*tr).smpl,
            &mut smp as *mut sample_t as *const c_void,
            ::core::mem::size_of::<sample_t>() as c_ulong as c_int,
        )) as c_int
}
unsafe extern "C" fn mp4e_write_fragment_header(
    mut mux: *mut MP4E_mux_t,
    mut track_num: c_int,
    mut data_bytes: c_int,
    mut duration: c_int,
    mut kind: c_int,
) -> c_int {
    let mut base: [c_uchar; 888] = [0; 888];
    let mut p: *mut c_uchar = base.as_mut_ptr();
    let mut stack_base: [*mut c_uchar; 20] = [std::ptr::null_mut::<c_uchar>(); 20];
    let mut stack: *mut *mut c_uchar = stack_base.as_mut_ptr();
    let mut pdata_offset: *mut c_uchar = std::ptr::null_mut::<c_uchar>();
    let mut flags: c_uint = 0;
    let mut tr: *mut track_t = ((*mux).tracks.data as *mut track_t).offset(track_num as isize);
    let fresh8 = stack;
    stack = stack.offset(1);
    *fresh8 = p;
    p = p.offset(4 as c_int as isize);
    let fresh9 = p;
    p = p.offset(1);
    *fresh9 = (BOX_moof as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh10 = p;
    p = p.offset(1);
    *fresh10 = (BOX_moof as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh11 = p;
    p = p.offset(1);
    *fresh11 = (BOX_moof as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh12 = p;
    p = p.offset(1);
    *fresh12 = (BOX_moof as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh13 = stack;
    stack = stack.offset(1);
    *fresh13 = p;
    p = p.offset(4 as c_int as isize);
    let fresh14 = p;
    p = p.offset(1);
    *fresh14 = (BOX_mfhd as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh15 = p;
    p = p.offset(1);
    *fresh15 = (BOX_mfhd as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh16 = p;
    p = p.offset(1);
    *fresh16 = (BOX_mfhd as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh17 = p;
    p = p.offset(1);
    *fresh17 = (BOX_mfhd as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh18 = p;
    p = p.offset(1);
    *fresh18 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh19 = p;
    p = p.offset(1);
    *fresh19 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh20 = p;
    p = p.offset(1);
    *fresh20 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh21 = p;
    p = p.offset(1);
    *fresh21 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh22 = p;
    p = p.offset(1);
    *fresh22 = ((*mux).fragments_count >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh23 = p;
    p = p.offset(1);
    *fresh23 = ((*mux).fragments_count >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh24 = p;
    p = p.offset(1);
    *fresh24 = ((*mux).fragments_count >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh25 = p;
    p = p.offset(1);
    *fresh25 = ((*mux).fragments_count >> (8 as c_int * 0 as c_int)) as c_uchar;
    stack = stack.offset(-1);
    *(*stack).offset(0 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
    *(*stack).offset(1 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
    *(*stack).offset(2 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    let fresh26 = stack;
    stack = stack.offset(1);
    *fresh26 = p;
    p = p.offset(4 as c_int as isize);
    let fresh27 = p;
    p = p.offset(1);
    *fresh27 = (BOX_traf as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh28 = p;
    p = p.offset(1);
    *fresh28 = (BOX_traf as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh29 = p;
    p = p.offset(1);
    *fresh29 = (BOX_traf as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh30 = p;
    p = p.offset(1);
    *fresh30 = (BOX_traf as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    flags = 0 as c_int as c_uint;
    if (*tr).info.track_media_kind as c_uint == e_video as c_int as c_uint {
        flags |= 0x20 as c_int as c_uint;
    } else {
        flags |= 0x8 as c_int as c_uint;
    }
    flags = (if (*tr).info.track_media_kind as c_uint == e_video as c_int as c_uint {
        0x20020 as c_int
    } else {
        0x20008 as c_int
    }) as c_uint;
    let fresh31 = stack;
    stack = stack.offset(1);
    *fresh31 = p;
    p = p.offset(4 as c_int as isize);
    let fresh32 = p;
    p = p.offset(1);
    *fresh32 = (BOX_tfhd as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh33 = p;
    p = p.offset(1);
    *fresh33 = (BOX_tfhd as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh34 = p;
    p = p.offset(1);
    *fresh34 = (BOX_tfhd as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh35 = p;
    p = p.offset(1);
    *fresh35 = (BOX_tfhd as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh36 = p;
    p = p.offset(1);
    *fresh36 = (flags >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh37 = p;
    p = p.offset(1);
    *fresh37 = (flags >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh38 = p;
    p = p.offset(1);
    *fresh38 = (flags >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh39 = p;
    p = p.offset(1);
    *fresh39 = (flags >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh40 = p;
    p = p.offset(1);
    *fresh40 = ((track_num + 1 as c_int) >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh41 = p;
    p = p.offset(1);
    *fresh41 = ((track_num + 1 as c_int) >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh42 = p;
    p = p.offset(1);
    *fresh42 = ((track_num + 1 as c_int) >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh43 = p;
    p = p.offset(1);
    *fresh43 = ((track_num + 1 as c_int) >> (8 as c_int * 0 as c_int)) as c_uchar;
    if (*tr).info.track_media_kind as c_uint == e_video as c_int as c_uint {
        let fresh44 = p;
        p = p.offset(1);
        *fresh44 = (0x1010000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh45 = p;
        p = p.offset(1);
        *fresh45 = (0x1010000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh46 = p;
        p = p.offset(1);
        *fresh46 = (0x1010000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh47 = p;
        p = p.offset(1);
        *fresh47 = (0x1010000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    } else {
        let fresh48 = p;
        p = p.offset(1);
        *fresh48 = (duration >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh49 = p;
        p = p.offset(1);
        *fresh49 = (duration >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh50 = p;
        p = p.offset(1);
        *fresh50 = (duration >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh51 = p;
        p = p.offset(1);
        *fresh51 = (duration >> (8 as c_int * 0 as c_int)) as c_uchar;
    }
    stack = stack.offset(-1);
    *(*stack).offset(0 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
    *(*stack).offset(1 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
    *(*stack).offset(2 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    if (*tr).info.track_media_kind as c_uint == e_audio as c_int as c_uint {
        flags = 0 as c_int as c_uint;
        flags |= 0x1 as c_int as c_uint;
        flags |= 0x200 as c_int as c_uint;
        let fresh52 = stack;
        stack = stack.offset(1);
        *fresh52 = p;
        p = p.offset(4 as c_int as isize);
        let fresh53 = p;
        p = p.offset(1);
        *fresh53 = (BOX_trun as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh54 = p;
        p = p.offset(1);
        *fresh54 = (BOX_trun as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh55 = p;
        p = p.offset(1);
        *fresh55 = (BOX_trun as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh56 = p;
        p = p.offset(1);
        *fresh56 = (BOX_trun as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh57 = p;
        p = p.offset(1);
        *fresh57 = (flags >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh58 = p;
        p = p.offset(1);
        *fresh58 = (flags >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh59 = p;
        p = p.offset(1);
        *fresh59 = (flags >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh60 = p;
        p = p.offset(1);
        *fresh60 = (flags >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh61 = p;
        p = p.offset(1);
        *fresh61 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh62 = p;
        p = p.offset(1);
        *fresh62 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh63 = p;
        p = p.offset(1);
        *fresh63 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh64 = p;
        p = p.offset(1);
        *fresh64 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        pdata_offset = p;
        p = p.offset(4 as c_int as isize);
        let fresh65 = p;
        p = p.offset(1);
        *fresh65 = (data_bytes >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh66 = p;
        p = p.offset(1);
        *fresh66 = (data_bytes >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh67 = p;
        p = p.offset(1);
        *fresh67 = (data_bytes >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh68 = p;
        p = p.offset(1);
        *fresh68 = (data_bytes >> (8 as c_int * 0 as c_int)) as c_uchar;
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    } else if kind == 1 as c_int {
        flags = 0 as c_int as c_uint;
        flags |= 0x1 as c_int as c_uint;
        flags |= 0x4 as c_int as c_uint;
        flags |= 0x100 as c_int as c_uint;
        flags |= 0x200 as c_int as c_uint;
        let fresh69 = stack;
        stack = stack.offset(1);
        *fresh69 = p;
        p = p.offset(4 as c_int as isize);
        let fresh70 = p;
        p = p.offset(1);
        *fresh70 = (BOX_trun as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh71 = p;
        p = p.offset(1);
        *fresh71 = (BOX_trun as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh72 = p;
        p = p.offset(1);
        *fresh72 = (BOX_trun as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh73 = p;
        p = p.offset(1);
        *fresh73 = (BOX_trun as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh74 = p;
        p = p.offset(1);
        *fresh74 = (flags >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh75 = p;
        p = p.offset(1);
        *fresh75 = (flags >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh76 = p;
        p = p.offset(1);
        *fresh76 = (flags >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh77 = p;
        p = p.offset(1);
        *fresh77 = (flags >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh78 = p;
        p = p.offset(1);
        *fresh78 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh79 = p;
        p = p.offset(1);
        *fresh79 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh80 = p;
        p = p.offset(1);
        *fresh80 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh81 = p;
        p = p.offset(1);
        *fresh81 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        pdata_offset = p;
        p = p.offset(4 as c_int as isize);
        let fresh82 = p;
        p = p.offset(1);
        *fresh82 = (0x2000000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh83 = p;
        p = p.offset(1);
        *fresh83 = (0x2000000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh84 = p;
        p = p.offset(1);
        *fresh84 = (0x2000000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh85 = p;
        p = p.offset(1);
        *fresh85 = (0x2000000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh86 = p;
        p = p.offset(1);
        *fresh86 = (duration >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh87 = p;
        p = p.offset(1);
        *fresh87 = (duration >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh88 = p;
        p = p.offset(1);
        *fresh88 = (duration >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh89 = p;
        p = p.offset(1);
        *fresh89 = (duration >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh90 = p;
        p = p.offset(1);
        *fresh90 = (data_bytes >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh91 = p;
        p = p.offset(1);
        *fresh91 = (data_bytes >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh92 = p;
        p = p.offset(1);
        *fresh92 = (data_bytes >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh93 = p;
        p = p.offset(1);
        *fresh93 = (data_bytes >> (8 as c_int * 0 as c_int)) as c_uchar;
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    } else {
        flags = 0 as c_int as c_uint;
        flags |= 0x1 as c_int as c_uint;
        flags |= 0x100 as c_int as c_uint;
        flags |= 0x200 as c_int as c_uint;
        let fresh94 = stack;
        stack = stack.offset(1);
        *fresh94 = p;
        p = p.offset(4 as c_int as isize);
        let fresh95 = p;
        p = p.offset(1);
        *fresh95 = (BOX_trun as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh96 = p;
        p = p.offset(1);
        *fresh96 = (BOX_trun as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh97 = p;
        p = p.offset(1);
        *fresh97 = (BOX_trun as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh98 = p;
        p = p.offset(1);
        *fresh98 = (BOX_trun as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh99 = p;
        p = p.offset(1);
        *fresh99 = (flags >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh100 = p;
        p = p.offset(1);
        *fresh100 = (flags >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh101 = p;
        p = p.offset(1);
        *fresh101 = (flags >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh102 = p;
        p = p.offset(1);
        *fresh102 = (flags >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh103 = p;
        p = p.offset(1);
        *fresh103 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh104 = p;
        p = p.offset(1);
        *fresh104 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh105 = p;
        p = p.offset(1);
        *fresh105 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh106 = p;
        p = p.offset(1);
        *fresh106 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        pdata_offset = p;
        p = p.offset(4 as c_int as isize);
        let fresh107 = p;
        p = p.offset(1);
        *fresh107 = (duration >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh108 = p;
        p = p.offset(1);
        *fresh108 = (duration >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh109 = p;
        p = p.offset(1);
        *fresh109 = (duration >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh110 = p;
        p = p.offset(1);
        *fresh110 = (duration >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh111 = p;
        p = p.offset(1);
        *fresh111 = (data_bytes >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh112 = p;
        p = p.offset(1);
        *fresh112 = (data_bytes >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh113 = p;
        p = p.offset(1);
        *fresh113 = (data_bytes >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh114 = p;
        p = p.offset(1);
        *fresh114 = (data_bytes >> (8 as c_int * 0 as c_int)) as c_uchar;
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    }
    stack = stack.offset(-1);
    *(*stack).offset(0 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
    *(*stack).offset(1 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
    *(*stack).offset(2 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    stack = stack.offset(-1);
    *(*stack).offset(0 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
    *(*stack).offset(1 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
    *(*stack).offset(2 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    *pdata_offset.offset(0 as c_int as isize) = ((p.offset_from(base.as_mut_ptr()) as c_long + 8 as c_int as c_long)
        >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
    *pdata_offset.offset(1 as c_int as isize) = ((p.offset_from(base.as_mut_ptr()) as c_long + 8 as c_int as c_long)
        >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
    *pdata_offset.offset(2 as c_int as isize) = ((p.offset_from(base.as_mut_ptr()) as c_long + 8 as c_int as c_long)
        >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
    *pdata_offset.offset(3 as c_int as isize) =
        (p.offset_from(base.as_mut_ptr()) as c_long + 8 as c_int as c_long) as c_char as c_uchar;
    let mut err: c_int = ((*mux).write_callback).expect("non-null function pointer")(
        (*mux).write_pos,
        base.as_mut_ptr() as *const c_void,
        p.offset_from(base.as_mut_ptr()) as c_long as size_t,
        (*mux).token,
    );
    if err != 0 {
        return err;
    }
    (*mux).write_pos += p.offset_from(base.as_mut_ptr()) as c_long;
    0 as c_int
}
unsafe extern "C" fn mp4e_write_mdat_box(mut mux: *mut MP4E_mux_t, mut size: uint32_t) -> c_int {
    let mut base: [c_uchar; 8] = [0; 8];
    let mut p: *mut c_uchar = base.as_mut_ptr();
    let fresh115 = p;
    p = p.offset(1);
    *fresh115 = (size >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh116 = p;
    p = p.offset(1);
    *fresh116 = (size >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh117 = p;
    p = p.offset(1);
    *fresh117 = (size >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh118 = p;
    p = p.offset(1);
    *fresh118 = (size >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh119 = p;
    p = p.offset(1);
    *fresh119 = (BOX_mdat as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh120 = p;
    p = p.offset(1);
    *fresh120 = (BOX_mdat as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh121 = p;
    p = p.offset(1);
    *fresh121 = (BOX_mdat as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh122 = p;
    p = p.offset(1);
    *fresh122 = (BOX_mdat as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let mut err: c_int = ((*mux).write_callback).expect("non-null function pointer")(
        (*mux).write_pos,
        base.as_mut_ptr() as *const c_void,
        p.offset_from(base.as_mut_ptr()) as c_long as size_t,
        (*mux).token,
    );
    if err != 0 {
        return err;
    }
    (*mux).write_pos += p.offset_from(base.as_mut_ptr()) as c_long;
    0 as c_int
}
#[no_mangle]
pub unsafe extern "C" fn MP4E_put_sample(
    mut mux: *mut MP4E_mux_t,
    mut track_num: c_int,
    mut data: *const c_void,
    mut data_bytes: c_int,
    mut duration: c_int,
    mut kind: c_int,
) -> c_int {
    let mut tr: *mut track_t = std::ptr::null_mut::<track_t>();
    if mux.is_null() || data.is_null() {
        return -(1 as c_int);
    }
    tr = ((*mux).tracks.data as *mut track_t).offset(track_num as isize);
    if (*mux).enable_fragmentation != 0 {
        let fresh123 = (*mux).fragments_count;
        (*mux).fragments_count += 1;
        if fresh123 == 0 {
            let mut err: c_int = mp4e_flush_index(mux);
            if err != 0 {
                return err;
            }
        }
        let mut err_0: c_int = mp4e_write_fragment_header(mux, track_num, data_bytes, duration, kind);
        if err_0 != 0 {
            return err_0;
        }
        let mut err_1: c_int = mp4e_write_mdat_box(mux, (data_bytes + 8 as c_int) as uint32_t);
        if err_1 != 0 {
            return err_1;
        }
        let mut err_2: c_int = ((*mux).write_callback).expect("non-null function pointer")(
            (*mux).write_pos,
            data,
            data_bytes as size_t,
            (*mux).token,
        );
        if err_2 != 0 {
            return err_2;
        }
        (*mux).write_pos += data_bytes as c_long;
        return 0 as c_int;
    }
    if kind != 2 as c_int {
        if (*mux).sequential_mode_flag != 0 {
            let mut err_3: c_int = write_pending_data(mux, tr);
            if err_3 != 0 {
                return err_3;
            }
        }
        if add_sample_descriptor(mux, tr, data_bytes, duration, kind) == 0 {
            return -(2 as c_int);
        }
    } else if (*mux).sequential_mode_flag == 0 {
        let mut smpl_desc: *mut sample_t = std::ptr::null_mut::<sample_t>();
        if ((*tr).smpl.bytes as c_ulong) < ::core::mem::size_of::<sample_t>() as c_ulong {
            return -(2 as c_int);
        }
        smpl_desc =
            (((*tr).smpl.data).offset((*tr).smpl.bytes as isize) as *mut sample_t).offset(-(1 as c_int as isize));
        (*smpl_desc).size =
            ((*smpl_desc).size as c_ulong).wrapping_add(data_bytes as c_ulong) as boxsize_t as boxsize_t;
    }
    if (*mux).sequential_mode_flag != 0 {
        if (minimp4_vector_put(&mut (*tr).pending_sample, data, data_bytes)).is_null() {
            return -(2 as c_int);
        }
    } else {
        let mut err_4: c_int = ((*mux).write_callback).expect("non-null function pointer")(
            (*mux).write_pos,
            data,
            data_bytes as size_t,
            (*mux).token,
        );
        if err_4 != 0 {
            return err_4;
        }
        (*mux).write_pos += data_bytes as c_long;
    }
    0 as c_int
}
unsafe extern "C" fn od_size_of_size(mut size: c_int) -> c_int {
    let mut i: c_int = 0;
    let mut size_of_size: c_int = 1 as c_int;
    i = size;
    while i > 0x7f as c_int {
        size_of_size += 1;
        i -= 0x7f as c_int;
    }
    size_of_size
}
#[no_mangle]
pub unsafe extern "C" fn MP4E_set_text_comment(mut mux: *mut MP4E_mux_t, mut comment: *const c_char) -> c_int {
    if mux.is_null() || comment.is_null() {
        return -(1 as c_int);
    }
    if !((*mux).text_comment).is_null() {
        minimp4_free((*mux).text_comment as *mut c_void);
    }
    (*mux).text_comment = strdup(comment);
    if ((*mux).text_comment).is_null() {
        return -(2 as c_int);
    }
    0 as c_int
}
unsafe extern "C" fn mp4e_flush_index(mut mux: *mut MP4E_mux_t) -> c_int {
    let mut stack_base: [*mut c_uchar; 20] = [std::ptr::null_mut::<c_uchar>(); 20];
    let mut stack: *mut *mut c_uchar = stack_base.as_mut_ptr();
    let mut base: *mut c_uchar = std::ptr::null_mut::<c_uchar>();
    let mut p: *mut c_uchar = std::ptr::null_mut::<c_uchar>();
    let mut ntr: c_uint = 0;
    let mut index_bytes: c_uint = 0;
    let mut ntracks: c_uint =
        ((*mux).tracks.bytes as c_ulong).wrapping_div(::core::mem::size_of::<track_t>() as c_ulong) as c_uint;
    let mut i: c_int = 0;
    let mut err: c_int = 0;
    index_bytes = 256 as c_int as c_uint;
    if !((*mux).text_comment).is_null() {
        index_bytes = (index_bytes as c_ulong)
            .wrapping_add((128 as c_int as c_ulong).wrapping_add(minimp4_strlen((*mux).text_comment)))
            as c_uint as c_uint;
    }
    ntr = 0 as c_int as c_uint;
    while ntr < ntracks {
        let mut tr: *mut track_t = ((*mux).tracks.data as *mut track_t).offset(ntr as isize);
        index_bytes = index_bytes.wrapping_add(512 as c_int as c_uint);
        index_bytes = (index_bytes as c_ulong).wrapping_add(
            ((*tr).smpl.bytes as c_ulong)
                .wrapping_mul(
                    (::core::mem::size_of::<sample_t>() as c_ulong)
                        .wrapping_add(4 as c_int as c_ulong)
                        .wrapping_add(4 as c_int as c_ulong),
                )
                .wrapping_div(::core::mem::size_of::<sample_t>() as c_ulong),
        ) as c_uint as c_uint;
        index_bytes = index_bytes.wrapping_add((*tr).vsps.bytes as c_uint);
        index_bytes = index_bytes.wrapping_add((*tr).vpps.bytes as c_uint);
        let mut err_0: c_int = write_pending_data(mux, tr);
        if err_0 != 0 {
            return err_0;
        }
        ntr = ntr.wrapping_add(1);
    }
    base = minimp4_malloc(index_bytes as size_t) as *mut c_uchar;
    if base.is_null() {
        return -(2 as c_int);
    }
    p = base;
    if (*mux).sequential_mode_flag == 0 {
        let mut size: int64_t =
            ((*mux).write_pos as c_ulong).wrapping_sub(::core::mem::size_of::<[c_uchar; 24]>() as c_ulong) as int64_t;
        let size_limit: int64_t = 0xfffffffe as c_uint as uint64_t as int64_t;
        if size > size_limit {
            let fresh124 = p;
            p = p.offset(1);
            *fresh124 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh125 = p;
            p = p.offset(1);
            *fresh125 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh126 = p;
            p = p.offset(1);
            *fresh126 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh127 = p;
            p = p.offset(1);
            *fresh127 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh128 = p;
            p = p.offset(1);
            *fresh128 = (BOX_mdat as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh129 = p;
            p = p.offset(1);
            *fresh129 = (BOX_mdat as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh130 = p;
            p = p.offset(1);
            *fresh130 = (BOX_mdat as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh131 = p;
            p = p.offset(1);
            *fresh131 = (BOX_mdat as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh132 = p;
            p = p.offset(1);
            *fresh132 =
                ((size >> 32 as c_int & 0xffffffff as c_uint as c_long) >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh133 = p;
            p = p.offset(1);
            *fresh133 =
                ((size >> 32 as c_int & 0xffffffff as c_uint as c_long) >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh134 = p;
            p = p.offset(1);
            *fresh134 =
                ((size >> 32 as c_int & 0xffffffff as c_uint as c_long) >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh135 = p;
            p = p.offset(1);
            *fresh135 =
                ((size >> 32 as c_int & 0xffffffff as c_uint as c_long) >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh136 = p;
            p = p.offset(1);
            *fresh136 = ((size & 0xffffffff as c_uint as c_long) >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh137 = p;
            p = p.offset(1);
            *fresh137 = ((size & 0xffffffff as c_uint as c_long) >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh138 = p;
            p = p.offset(1);
            *fresh138 = ((size & 0xffffffff as c_uint as c_long) >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh139 = p;
            p = p.offset(1);
            *fresh139 = ((size & 0xffffffff as c_uint as c_long) >> (8 as c_int * 0 as c_int)) as c_uchar;
        } else {
            let fresh140 = p;
            p = p.offset(1);
            *fresh140 = (8 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh141 = p;
            p = p.offset(1);
            *fresh141 = (8 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh142 = p;
            p = p.offset(1);
            *fresh142 = (8 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh143 = p;
            p = p.offset(1);
            *fresh143 = (8 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh144 = p;
            p = p.offset(1);
            *fresh144 = (BOX_free as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh145 = p;
            p = p.offset(1);
            *fresh145 = (BOX_free as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh146 = p;
            p = p.offset(1);
            *fresh146 = (BOX_free as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh147 = p;
            p = p.offset(1);
            *fresh147 = (BOX_free as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh148 = p;
            p = p.offset(1);
            *fresh148 = ((size - 8 as c_int as c_long) >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh149 = p;
            p = p.offset(1);
            *fresh149 = ((size - 8 as c_int as c_long) >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh150 = p;
            p = p.offset(1);
            *fresh150 = ((size - 8 as c_int as c_long) >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh151 = p;
            p = p.offset(1);
            *fresh151 = ((size - 8 as c_int as c_long) >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh152 = p;
            p = p.offset(1);
            *fresh152 = (BOX_mdat as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh153 = p;
            p = p.offset(1);
            *fresh153 = (BOX_mdat as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh154 = p;
            p = p.offset(1);
            *fresh154 = (BOX_mdat as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh155 = p;
            p = p.offset(1);
            *fresh155 = (BOX_mdat as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        }
        let mut err_1: c_int = ((*mux).write_callback).expect("non-null function pointer")(
            ::core::mem::size_of::<[c_uchar; 24]>() as c_ulong as int64_t,
            base as *const c_void,
            p.offset_from(base) as c_long as size_t,
            (*mux).token,
        );
        if err_1 != 0 {
            return err_1;
        }
        p = base;
    }
    let fresh156 = stack;
    stack = stack.offset(1);
    *fresh156 = p;
    p = p.offset(4 as c_int as isize);
    let fresh157 = p;
    p = p.offset(1);
    *fresh157 = (BOX_moov as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh158 = p;
    p = p.offset(1);
    *fresh158 = (BOX_moov as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh159 = p;
    p = p.offset(1);
    *fresh159 = (BOX_moov as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh160 = p;
    p = p.offset(1);
    *fresh160 = (BOX_moov as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh161 = stack;
    stack = stack.offset(1);
    *fresh161 = p;
    p = p.offset(4 as c_int as isize);
    let fresh162 = p;
    p = p.offset(1);
    *fresh162 = (BOX_mvhd as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh163 = p;
    p = p.offset(1);
    *fresh163 = (BOX_mvhd as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh164 = p;
    p = p.offset(1);
    *fresh164 = (BOX_mvhd as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh165 = p;
    p = p.offset(1);
    *fresh165 = (BOX_mvhd as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh166 = p;
    p = p.offset(1);
    *fresh166 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh167 = p;
    p = p.offset(1);
    *fresh167 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh168 = p;
    p = p.offset(1);
    *fresh168 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh169 = p;
    p = p.offset(1);
    *fresh169 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh170 = p;
    p = p.offset(1);
    *fresh170 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh171 = p;
    p = p.offset(1);
    *fresh171 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh172 = p;
    p = p.offset(1);
    *fresh172 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh173 = p;
    p = p.offset(1);
    *fresh173 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh174 = p;
    p = p.offset(1);
    *fresh174 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh175 = p;
    p = p.offset(1);
    *fresh175 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh176 = p;
    p = p.offset(1);
    *fresh176 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh177 = p;
    p = p.offset(1);
    *fresh177 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    if ntracks != 0 {
        let mut tr_0: *mut track_t = ((*mux).tracks.data as *mut track_t).offset(0 as c_int as isize);
        let mut duration: c_uint = get_duration(tr_0);
        duration = (duration as c_longlong * 1 as c_longlong * 1000 as c_int as c_longlong
            / (*tr_0).info.time_scale as c_longlong) as c_uint;
        let fresh178 = p;
        p = p.offset(1);
        *fresh178 = (1000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh179 = p;
        p = p.offset(1);
        *fresh179 = (1000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh180 = p;
        p = p.offset(1);
        *fresh180 = (1000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh181 = p;
        p = p.offset(1);
        *fresh181 = (1000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh182 = p;
        p = p.offset(1);
        *fresh182 = (duration >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh183 = p;
        p = p.offset(1);
        *fresh183 = (duration >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh184 = p;
        p = p.offset(1);
        *fresh184 = (duration >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh185 = p;
        p = p.offset(1);
        *fresh185 = (duration >> (8 as c_int * 0 as c_int)) as c_uchar;
    }
    let fresh186 = p;
    p = p.offset(1);
    *fresh186 = (0x10000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh187 = p;
    p = p.offset(1);
    *fresh187 = (0x10000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh188 = p;
    p = p.offset(1);
    *fresh188 = (0x10000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh189 = p;
    p = p.offset(1);
    *fresh189 = (0x10000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh190 = p;
    p = p.offset(1);
    *fresh190 = (0x100 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh191 = p;
    p = p.offset(1);
    *fresh191 = (0x100 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh192 = p;
    p = p.offset(1);
    *fresh192 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh193 = p;
    p = p.offset(1);
    *fresh193 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh194 = p;
    p = p.offset(1);
    *fresh194 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh195 = p;
    p = p.offset(1);
    *fresh195 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh196 = p;
    p = p.offset(1);
    *fresh196 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh197 = p;
    p = p.offset(1);
    *fresh197 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh198 = p;
    p = p.offset(1);
    *fresh198 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh199 = p;
    p = p.offset(1);
    *fresh199 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh200 = p;
    p = p.offset(1);
    *fresh200 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh201 = p;
    p = p.offset(1);
    *fresh201 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh202 = p;
    p = p.offset(1);
    *fresh202 = (0x10000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh203 = p;
    p = p.offset(1);
    *fresh203 = (0x10000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh204 = p;
    p = p.offset(1);
    *fresh204 = (0x10000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh205 = p;
    p = p.offset(1);
    *fresh205 = (0x10000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh206 = p;
    p = p.offset(1);
    *fresh206 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh207 = p;
    p = p.offset(1);
    *fresh207 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh208 = p;
    p = p.offset(1);
    *fresh208 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh209 = p;
    p = p.offset(1);
    *fresh209 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh210 = p;
    p = p.offset(1);
    *fresh210 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh211 = p;
    p = p.offset(1);
    *fresh211 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh212 = p;
    p = p.offset(1);
    *fresh212 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh213 = p;
    p = p.offset(1);
    *fresh213 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh214 = p;
    p = p.offset(1);
    *fresh214 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh215 = p;
    p = p.offset(1);
    *fresh215 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh216 = p;
    p = p.offset(1);
    *fresh216 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh217 = p;
    p = p.offset(1);
    *fresh217 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh218 = p;
    p = p.offset(1);
    *fresh218 = (0x10000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh219 = p;
    p = p.offset(1);
    *fresh219 = (0x10000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh220 = p;
    p = p.offset(1);
    *fresh220 = (0x10000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh221 = p;
    p = p.offset(1);
    *fresh221 = (0x10000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh222 = p;
    p = p.offset(1);
    *fresh222 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh223 = p;
    p = p.offset(1);
    *fresh223 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh224 = p;
    p = p.offset(1);
    *fresh224 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh225 = p;
    p = p.offset(1);
    *fresh225 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh226 = p;
    p = p.offset(1);
    *fresh226 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh227 = p;
    p = p.offset(1);
    *fresh227 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh228 = p;
    p = p.offset(1);
    *fresh228 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh229 = p;
    p = p.offset(1);
    *fresh229 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh230 = p;
    p = p.offset(1);
    *fresh230 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh231 = p;
    p = p.offset(1);
    *fresh231 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh232 = p;
    p = p.offset(1);
    *fresh232 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh233 = p;
    p = p.offset(1);
    *fresh233 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh234 = p;
    p = p.offset(1);
    *fresh234 = (0x40000000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh235 = p;
    p = p.offset(1);
    *fresh235 = (0x40000000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh236 = p;
    p = p.offset(1);
    *fresh236 = (0x40000000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh237 = p;
    p = p.offset(1);
    *fresh237 = (0x40000000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh238 = p;
    p = p.offset(1);
    *fresh238 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh239 = p;
    p = p.offset(1);
    *fresh239 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh240 = p;
    p = p.offset(1);
    *fresh240 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh241 = p;
    p = p.offset(1);
    *fresh241 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh242 = p;
    p = p.offset(1);
    *fresh242 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh243 = p;
    p = p.offset(1);
    *fresh243 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh244 = p;
    p = p.offset(1);
    *fresh244 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh245 = p;
    p = p.offset(1);
    *fresh245 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh246 = p;
    p = p.offset(1);
    *fresh246 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh247 = p;
    p = p.offset(1);
    *fresh247 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh248 = p;
    p = p.offset(1);
    *fresh248 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh249 = p;
    p = p.offset(1);
    *fresh249 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh250 = p;
    p = p.offset(1);
    *fresh250 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh251 = p;
    p = p.offset(1);
    *fresh251 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh252 = p;
    p = p.offset(1);
    *fresh252 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh253 = p;
    p = p.offset(1);
    *fresh253 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh254 = p;
    p = p.offset(1);
    *fresh254 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh255 = p;
    p = p.offset(1);
    *fresh255 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh256 = p;
    p = p.offset(1);
    *fresh256 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh257 = p;
    p = p.offset(1);
    *fresh257 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh258 = p;
    p = p.offset(1);
    *fresh258 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh259 = p;
    p = p.offset(1);
    *fresh259 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh260 = p;
    p = p.offset(1);
    *fresh260 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh261 = p;
    p = p.offset(1);
    *fresh261 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
    let fresh262 = p;
    p = p.offset(1);
    *fresh262 = (ntracks.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 3 as c_int)) as c_uchar;
    let fresh263 = p;
    p = p.offset(1);
    *fresh263 = (ntracks.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 2 as c_int)) as c_uchar;
    let fresh264 = p;
    p = p.offset(1);
    *fresh264 = (ntracks.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 1 as c_int)) as c_uchar;
    let fresh265 = p;
    p = p.offset(1);
    *fresh265 = (ntracks.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 0 as c_int)) as c_uchar;
    stack = stack.offset(-1);
    *(*stack).offset(0 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
    *(*stack).offset(1 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
    *(*stack).offset(2 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    let mut current_block_1303: u64;
    ntr = 0 as c_int as c_uint;
    while ntr < ntracks {
        let mut tr_1: *mut track_t = ((*mux).tracks.data as *mut track_t).offset(ntr as isize);
        let mut duration_0: c_uint = get_duration(tr_1);
        let mut samples_count: c_int =
            ((*tr_1).smpl.bytes as c_ulong).wrapping_div(::core::mem::size_of::<sample_t>() as c_ulong) as c_int;
        let mut sample: *const sample_t = (*tr_1).smpl.data as *const sample_t;
        let mut handler_type: c_uint = 0;
        let mut handler_ascii: *const c_char = std::ptr::null::<c_char>();
        if (*mux).enable_fragmentation != 0 {
            samples_count = 0 as c_int;
            current_block_1303 = 12223373342341601825;
        } else if samples_count <= 0 as c_int {
            current_block_1303 = 7034501744547627146;
        } else {
            current_block_1303 = 12223373342341601825;
        }
        match current_block_1303 {
            12223373342341601825 => {
                match (*tr_1).info.track_media_kind as c_uint {
                    0 => {
                        handler_type = 0x736f756e as c_int as c_uint;
                        handler_ascii = b"SoundHandler\0" as *const u8 as *const c_char;
                    }
                    1 => {
                        handler_type = 0x76696465 as c_int as c_uint;
                        handler_ascii = b"VideoHandler\0" as *const u8 as *const c_char;
                    }
                    2 => {
                        handler_type = 0x6765736d as c_int as c_uint;
                    }
                    _ => return -(1 as c_int),
                }
                let fresh266 = stack;
                stack = stack.offset(1);
                *fresh266 = p;
                p = p.offset(4 as c_int as isize);
                let fresh267 = p;
                p = p.offset(1);
                *fresh267 = (BOX_trak as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh268 = p;
                p = p.offset(1);
                *fresh268 = (BOX_trak as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh269 = p;
                p = p.offset(1);
                *fresh269 = (BOX_trak as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh270 = p;
                p = p.offset(1);
                *fresh270 = (BOX_trak as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh271 = stack;
                stack = stack.offset(1);
                *fresh271 = p;
                p = p.offset(4 as c_int as isize);
                let fresh272 = p;
                p = p.offset(1);
                *fresh272 = (BOX_tkhd as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh273 = p;
                p = p.offset(1);
                *fresh273 = (BOX_tkhd as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh274 = p;
                p = p.offset(1);
                *fresh274 = (BOX_tkhd as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh275 = p;
                p = p.offset(1);
                *fresh275 = (BOX_tkhd as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh276 = p;
                p = p.offset(1);
                *fresh276 = (7 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh277 = p;
                p = p.offset(1);
                *fresh277 = (7 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh278 = p;
                p = p.offset(1);
                *fresh278 = (7 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh279 = p;
                p = p.offset(1);
                *fresh279 = (7 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh280 = p;
                p = p.offset(1);
                *fresh280 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh281 = p;
                p = p.offset(1);
                *fresh281 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh282 = p;
                p = p.offset(1);
                *fresh282 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh283 = p;
                p = p.offset(1);
                *fresh283 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh284 = p;
                p = p.offset(1);
                *fresh284 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh285 = p;
                p = p.offset(1);
                *fresh285 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh286 = p;
                p = p.offset(1);
                *fresh286 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh287 = p;
                p = p.offset(1);
                *fresh287 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh288 = p;
                p = p.offset(1);
                *fresh288 = (ntr.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh289 = p;
                p = p.offset(1);
                *fresh289 = (ntr.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh290 = p;
                p = p.offset(1);
                *fresh290 = (ntr.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh291 = p;
                p = p.offset(1);
                *fresh291 = (ntr.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh292 = p;
                p = p.offset(1);
                *fresh292 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh293 = p;
                p = p.offset(1);
                *fresh293 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh294 = p;
                p = p.offset(1);
                *fresh294 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh295 = p;
                p = p.offset(1);
                *fresh295 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh296 = p;
                p = p.offset(1);
                *fresh296 = ((duration_0 as c_longlong * 1 as c_longlong * 1000 as c_int as c_longlong
                    / (*tr_1).info.time_scale as c_longlong) as c_uint
                    >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh297 = p;
                p = p.offset(1);
                *fresh297 = ((duration_0 as c_longlong * 1 as c_longlong * 1000 as c_int as c_longlong
                    / (*tr_1).info.time_scale as c_longlong) as c_uint
                    >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh298 = p;
                p = p.offset(1);
                *fresh298 = ((duration_0 as c_longlong * 1 as c_longlong * 1000 as c_int as c_longlong
                    / (*tr_1).info.time_scale as c_longlong) as c_uint
                    >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh299 = p;
                p = p.offset(1);
                *fresh299 = ((duration_0 as c_longlong * 1 as c_longlong * 1000 as c_int as c_longlong
                    / (*tr_1).info.time_scale as c_longlong) as c_uint
                    >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh300 = p;
                p = p.offset(1);
                *fresh300 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh301 = p;
                p = p.offset(1);
                *fresh301 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh302 = p;
                p = p.offset(1);
                *fresh302 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh303 = p;
                p = p.offset(1);
                *fresh303 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh304 = p;
                p = p.offset(1);
                *fresh304 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh305 = p;
                p = p.offset(1);
                *fresh305 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh306 = p;
                p = p.offset(1);
                *fresh306 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh307 = p;
                p = p.offset(1);
                *fresh307 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh308 = p;
                p = p.offset(1);
                *fresh308 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh309 = p;
                p = p.offset(1);
                *fresh309 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh310 = p;
                p = p.offset(1);
                *fresh310 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh311 = p;
                p = p.offset(1);
                *fresh311 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh312 = p;
                p = p.offset(1);
                *fresh312 = (0x100 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh313 = p;
                p = p.offset(1);
                *fresh313 = (0x100 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh314 = p;
                p = p.offset(1);
                *fresh314 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh315 = p;
                p = p.offset(1);
                *fresh315 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh316 = p;
                p = p.offset(1);
                *fresh316 = (0x10000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh317 = p;
                p = p.offset(1);
                *fresh317 = (0x10000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh318 = p;
                p = p.offset(1);
                *fresh318 = (0x10000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh319 = p;
                p = p.offset(1);
                *fresh319 = (0x10000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh320 = p;
                p = p.offset(1);
                *fresh320 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh321 = p;
                p = p.offset(1);
                *fresh321 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh322 = p;
                p = p.offset(1);
                *fresh322 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh323 = p;
                p = p.offset(1);
                *fresh323 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh324 = p;
                p = p.offset(1);
                *fresh324 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh325 = p;
                p = p.offset(1);
                *fresh325 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh326 = p;
                p = p.offset(1);
                *fresh326 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh327 = p;
                p = p.offset(1);
                *fresh327 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh328 = p;
                p = p.offset(1);
                *fresh328 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh329 = p;
                p = p.offset(1);
                *fresh329 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh330 = p;
                p = p.offset(1);
                *fresh330 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh331 = p;
                p = p.offset(1);
                *fresh331 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh332 = p;
                p = p.offset(1);
                *fresh332 = (0x10000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh333 = p;
                p = p.offset(1);
                *fresh333 = (0x10000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh334 = p;
                p = p.offset(1);
                *fresh334 = (0x10000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh335 = p;
                p = p.offset(1);
                *fresh335 = (0x10000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh336 = p;
                p = p.offset(1);
                *fresh336 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh337 = p;
                p = p.offset(1);
                *fresh337 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh338 = p;
                p = p.offset(1);
                *fresh338 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh339 = p;
                p = p.offset(1);
                *fresh339 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh340 = p;
                p = p.offset(1);
                *fresh340 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh341 = p;
                p = p.offset(1);
                *fresh341 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh342 = p;
                p = p.offset(1);
                *fresh342 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh343 = p;
                p = p.offset(1);
                *fresh343 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh344 = p;
                p = p.offset(1);
                *fresh344 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh345 = p;
                p = p.offset(1);
                *fresh345 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh346 = p;
                p = p.offset(1);
                *fresh346 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh347 = p;
                p = p.offset(1);
                *fresh347 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh348 = p;
                p = p.offset(1);
                *fresh348 = (0x40000000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh349 = p;
                p = p.offset(1);
                *fresh349 = (0x40000000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh350 = p;
                p = p.offset(1);
                *fresh350 = (0x40000000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh351 = p;
                p = p.offset(1);
                *fresh351 = (0x40000000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                if (*tr_1).info.track_media_kind as c_uint == e_audio as c_int as c_uint
                    || (*tr_1).info.track_media_kind as c_uint == e_private as c_int as c_uint
                {
                    let fresh352 = p;
                    p = p.offset(1);
                    *fresh352 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh353 = p;
                    p = p.offset(1);
                    *fresh353 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh354 = p;
                    p = p.offset(1);
                    *fresh354 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh355 = p;
                    p = p.offset(1);
                    *fresh355 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh356 = p;
                    p = p.offset(1);
                    *fresh356 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh357 = p;
                    p = p.offset(1);
                    *fresh357 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh358 = p;
                    p = p.offset(1);
                    *fresh358 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh359 = p;
                    p = p.offset(1);
                    *fresh359 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                } else {
                    let fresh360 = p;
                    p = p.offset(1);
                    *fresh360 = (((*tr_1).info.u.v.width * 0x10000 as c_int) >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh361 = p;
                    p = p.offset(1);
                    *fresh361 = (((*tr_1).info.u.v.width * 0x10000 as c_int) >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh362 = p;
                    p = p.offset(1);
                    *fresh362 = (((*tr_1).info.u.v.width * 0x10000 as c_int) >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh363 = p;
                    p = p.offset(1);
                    *fresh363 = (((*tr_1).info.u.v.width * 0x10000 as c_int) >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh364 = p;
                    p = p.offset(1);
                    *fresh364 = (((*tr_1).info.u.v.height * 0x10000 as c_int) >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh365 = p;
                    p = p.offset(1);
                    *fresh365 = (((*tr_1).info.u.v.height * 0x10000 as c_int) >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh366 = p;
                    p = p.offset(1);
                    *fresh366 = (((*tr_1).info.u.v.height * 0x10000 as c_int) >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh367 = p;
                    p = p.offset(1);
                    *fresh367 = (((*tr_1).info.u.v.height * 0x10000 as c_int) >> (8 as c_int * 0 as c_int)) as c_uchar;
                }
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                let fresh368 = stack;
                stack = stack.offset(1);
                *fresh368 = p;
                p = p.offset(4 as c_int as isize);
                let fresh369 = p;
                p = p.offset(1);
                *fresh369 = (BOX_mdia as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh370 = p;
                p = p.offset(1);
                *fresh370 = (BOX_mdia as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh371 = p;
                p = p.offset(1);
                *fresh371 = (BOX_mdia as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh372 = p;
                p = p.offset(1);
                *fresh372 = (BOX_mdia as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh373 = stack;
                stack = stack.offset(1);
                *fresh373 = p;
                p = p.offset(4 as c_int as isize);
                let fresh374 = p;
                p = p.offset(1);
                *fresh374 = (BOX_mdhd as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh375 = p;
                p = p.offset(1);
                *fresh375 = (BOX_mdhd as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh376 = p;
                p = p.offset(1);
                *fresh376 = (BOX_mdhd as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh377 = p;
                p = p.offset(1);
                *fresh377 = (BOX_mdhd as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh378 = p;
                p = p.offset(1);
                *fresh378 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh379 = p;
                p = p.offset(1);
                *fresh379 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh380 = p;
                p = p.offset(1);
                *fresh380 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh381 = p;
                p = p.offset(1);
                *fresh381 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh382 = p;
                p = p.offset(1);
                *fresh382 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh383 = p;
                p = p.offset(1);
                *fresh383 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh384 = p;
                p = p.offset(1);
                *fresh384 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh385 = p;
                p = p.offset(1);
                *fresh385 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh386 = p;
                p = p.offset(1);
                *fresh386 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh387 = p;
                p = p.offset(1);
                *fresh387 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh388 = p;
                p = p.offset(1);
                *fresh388 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh389 = p;
                p = p.offset(1);
                *fresh389 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh390 = p;
                p = p.offset(1);
                *fresh390 = ((*tr_1).info.time_scale >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh391 = p;
                p = p.offset(1);
                *fresh391 = ((*tr_1).info.time_scale >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh392 = p;
                p = p.offset(1);
                *fresh392 = ((*tr_1).info.time_scale >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh393 = p;
                p = p.offset(1);
                *fresh393 = ((*tr_1).info.time_scale >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh394 = p;
                p = p.offset(1);
                *fresh394 = (duration_0 >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh395 = p;
                p = p.offset(1);
                *fresh395 = (duration_0 >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh396 = p;
                p = p.offset(1);
                *fresh396 = (duration_0 >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh397 = p;
                p = p.offset(1);
                *fresh397 = (duration_0 >> (8 as c_int * 0 as c_int)) as c_uchar;
                let mut lang_code: c_int = ((*tr_1).info.language[0 as c_int as usize] as c_int & 31 as c_int)
                    << 10 as c_int
                    | ((*tr_1).info.language[1 as c_int as usize] as c_int & 31 as c_int) << 5 as c_int
                    | (*tr_1).info.language[2 as c_int as usize] as c_int & 31 as c_int;
                let fresh398 = p;
                p = p.offset(1);
                *fresh398 = (lang_code >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh399 = p;
                p = p.offset(1);
                *fresh399 = (lang_code >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh400 = p;
                p = p.offset(1);
                *fresh400 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh401 = p;
                p = p.offset(1);
                *fresh401 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                let fresh402 = stack;
                stack = stack.offset(1);
                *fresh402 = p;
                p = p.offset(4 as c_int as isize);
                let fresh403 = p;
                p = p.offset(1);
                *fresh403 = (BOX_hdlr as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh404 = p;
                p = p.offset(1);
                *fresh404 = (BOX_hdlr as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh405 = p;
                p = p.offset(1);
                *fresh405 = (BOX_hdlr as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh406 = p;
                p = p.offset(1);
                *fresh406 = (BOX_hdlr as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh407 = p;
                p = p.offset(1);
                *fresh407 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh408 = p;
                p = p.offset(1);
                *fresh408 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh409 = p;
                p = p.offset(1);
                *fresh409 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh410 = p;
                p = p.offset(1);
                *fresh410 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh411 = p;
                p = p.offset(1);
                *fresh411 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh412 = p;
                p = p.offset(1);
                *fresh412 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh413 = p;
                p = p.offset(1);
                *fresh413 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh414 = p;
                p = p.offset(1);
                *fresh414 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh415 = p;
                p = p.offset(1);
                *fresh415 = (handler_type >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh416 = p;
                p = p.offset(1);
                *fresh416 = (handler_type >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh417 = p;
                p = p.offset(1);
                *fresh417 = (handler_type >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh418 = p;
                p = p.offset(1);
                *fresh418 = (handler_type >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh419 = p;
                p = p.offset(1);
                *fresh419 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh420 = p;
                p = p.offset(1);
                *fresh420 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh421 = p;
                p = p.offset(1);
                *fresh421 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh422 = p;
                p = p.offset(1);
                *fresh422 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh423 = p;
                p = p.offset(1);
                *fresh423 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh424 = p;
                p = p.offset(1);
                *fresh424 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh425 = p;
                p = p.offset(1);
                *fresh425 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh426 = p;
                p = p.offset(1);
                *fresh426 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh427 = p;
                p = p.offset(1);
                *fresh427 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh428 = p;
                p = p.offset(1);
                *fresh428 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh429 = p;
                p = p.offset(1);
                *fresh429 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh430 = p;
                p = p.offset(1);
                *fresh430 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                if !handler_ascii.is_null() {
                    i = 0 as c_int;
                    while i < minimp4_strlen(handler_ascii) as c_int + 1 as c_int {
                        let fresh431 = p;
                        p = p.offset(1);
                        *fresh431 =
                            (*handler_ascii.offset(i as isize) as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        i += 1;
                    }
                } else {
                    let fresh432 = p;
                    p = p.offset(1);
                    *fresh432 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh433 = p;
                    p = p.offset(1);
                    *fresh433 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh434 = p;
                    p = p.offset(1);
                    *fresh434 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh435 = p;
                    p = p.offset(1);
                    *fresh435 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                }
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                let fresh436 = stack;
                stack = stack.offset(1);
                *fresh436 = p;
                p = p.offset(4 as c_int as isize);
                let fresh437 = p;
                p = p.offset(1);
                *fresh437 = (BOX_minf as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh438 = p;
                p = p.offset(1);
                *fresh438 = (BOX_minf as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh439 = p;
                p = p.offset(1);
                *fresh439 = (BOX_minf as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh440 = p;
                p = p.offset(1);
                *fresh440 = (BOX_minf as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                if (*tr_1).info.track_media_kind as c_uint == e_audio as c_int as c_uint {
                    let fresh441 = stack;
                    stack = stack.offset(1);
                    *fresh441 = p;
                    p = p.offset(4 as c_int as isize);
                    let fresh442 = p;
                    p = p.offset(1);
                    *fresh442 = (BOX_smhd as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh443 = p;
                    p = p.offset(1);
                    *fresh443 = (BOX_smhd as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh444 = p;
                    p = p.offset(1);
                    *fresh444 = (BOX_smhd as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh445 = p;
                    p = p.offset(1);
                    *fresh445 = (BOX_smhd as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh446 = p;
                    p = p.offset(1);
                    *fresh446 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh447 = p;
                    p = p.offset(1);
                    *fresh447 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh448 = p;
                    p = p.offset(1);
                    *fresh448 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh449 = p;
                    p = p.offset(1);
                    *fresh449 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh450 = p;
                    p = p.offset(1);
                    *fresh450 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh451 = p;
                    p = p.offset(1);
                    *fresh451 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh452 = p;
                    p = p.offset(1);
                    *fresh452 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh453 = p;
                    p = p.offset(1);
                    *fresh453 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    stack = stack.offset(-1);
                    *(*stack).offset(0 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(1 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(2 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                }
                if (*tr_1).info.track_media_kind as c_uint == e_video as c_int as c_uint {
                    let fresh454 = stack;
                    stack = stack.offset(1);
                    *fresh454 = p;
                    p = p.offset(4 as c_int as isize);
                    let fresh455 = p;
                    p = p.offset(1);
                    *fresh455 = (BOX_vmhd as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh456 = p;
                    p = p.offset(1);
                    *fresh456 = (BOX_vmhd as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh457 = p;
                    p = p.offset(1);
                    *fresh457 = (BOX_vmhd as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh458 = p;
                    p = p.offset(1);
                    *fresh458 = (BOX_vmhd as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh459 = p;
                    p = p.offset(1);
                    *fresh459 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh460 = p;
                    p = p.offset(1);
                    *fresh460 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh461 = p;
                    p = p.offset(1);
                    *fresh461 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh462 = p;
                    p = p.offset(1);
                    *fresh462 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh463 = p;
                    p = p.offset(1);
                    *fresh463 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh464 = p;
                    p = p.offset(1);
                    *fresh464 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh465 = p;
                    p = p.offset(1);
                    *fresh465 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh466 = p;
                    p = p.offset(1);
                    *fresh466 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh467 = p;
                    p = p.offset(1);
                    *fresh467 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh468 = p;
                    p = p.offset(1);
                    *fresh468 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh469 = p;
                    p = p.offset(1);
                    *fresh469 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh470 = p;
                    p = p.offset(1);
                    *fresh470 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    stack = stack.offset(-1);
                    *(*stack).offset(0 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(1 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(2 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                }
                let fresh471 = stack;
                stack = stack.offset(1);
                *fresh471 = p;
                p = p.offset(4 as c_int as isize);
                let fresh472 = p;
                p = p.offset(1);
                *fresh472 = (BOX_dinf as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh473 = p;
                p = p.offset(1);
                *fresh473 = (BOX_dinf as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh474 = p;
                p = p.offset(1);
                *fresh474 = (BOX_dinf as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh475 = p;
                p = p.offset(1);
                *fresh475 = (BOX_dinf as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh476 = stack;
                stack = stack.offset(1);
                *fresh476 = p;
                p = p.offset(4 as c_int as isize);
                let fresh477 = p;
                p = p.offset(1);
                *fresh477 = (BOX_dref as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh478 = p;
                p = p.offset(1);
                *fresh478 = (BOX_dref as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh479 = p;
                p = p.offset(1);
                *fresh479 = (BOX_dref as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh480 = p;
                p = p.offset(1);
                *fresh480 = (BOX_dref as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh481 = p;
                p = p.offset(1);
                *fresh481 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh482 = p;
                p = p.offset(1);
                *fresh482 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh483 = p;
                p = p.offset(1);
                *fresh483 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh484 = p;
                p = p.offset(1);
                *fresh484 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh485 = p;
                p = p.offset(1);
                *fresh485 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh486 = p;
                p = p.offset(1);
                *fresh486 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh487 = p;
                p = p.offset(1);
                *fresh487 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh488 = p;
                p = p.offset(1);
                *fresh488 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh489 = stack;
                stack = stack.offset(1);
                *fresh489 = p;
                p = p.offset(4 as c_int as isize);
                let fresh490 = p;
                p = p.offset(1);
                *fresh490 = (BOX_url as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh491 = p;
                p = p.offset(1);
                *fresh491 = (BOX_url as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh492 = p;
                p = p.offset(1);
                *fresh492 = (BOX_url as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh493 = p;
                p = p.offset(1);
                *fresh493 = (BOX_url as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh494 = p;
                p = p.offset(1);
                *fresh494 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh495 = p;
                p = p.offset(1);
                *fresh495 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh496 = p;
                p = p.offset(1);
                *fresh496 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh497 = p;
                p = p.offset(1);
                *fresh497 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                let fresh498 = stack;
                stack = stack.offset(1);
                *fresh498 = p;
                p = p.offset(4 as c_int as isize);
                let fresh499 = p;
                p = p.offset(1);
                *fresh499 = (BOX_stbl as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh500 = p;
                p = p.offset(1);
                *fresh500 = (BOX_stbl as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh501 = p;
                p = p.offset(1);
                *fresh501 = (BOX_stbl as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh502 = p;
                p = p.offset(1);
                *fresh502 = (BOX_stbl as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh503 = stack;
                stack = stack.offset(1);
                *fresh503 = p;
                p = p.offset(4 as c_int as isize);
                let fresh504 = p;
                p = p.offset(1);
                *fresh504 = (BOX_stsd as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh505 = p;
                p = p.offset(1);
                *fresh505 = (BOX_stsd as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh506 = p;
                p = p.offset(1);
                *fresh506 = (BOX_stsd as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh507 = p;
                p = p.offset(1);
                *fresh507 = (BOX_stsd as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh508 = p;
                p = p.offset(1);
                *fresh508 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh509 = p;
                p = p.offset(1);
                *fresh509 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh510 = p;
                p = p.offset(1);
                *fresh510 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh511 = p;
                p = p.offset(1);
                *fresh511 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh512 = p;
                p = p.offset(1);
                *fresh512 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh513 = p;
                p = p.offset(1);
                *fresh513 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh514 = p;
                p = p.offset(1);
                *fresh514 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh515 = p;
                p = p.offset(1);
                *fresh515 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                if (*tr_1).info.track_media_kind as c_uint == e_audio as c_int as c_uint
                    || (*tr_1).info.track_media_kind as c_uint == e_private as c_int as c_uint
                {
                    if (*tr_1).info.track_media_kind as c_uint == e_audio as c_int as c_uint {
                        let fresh516 = stack;
                        stack = stack.offset(1);
                        *fresh516 = p;
                        p = p.offset(4 as c_int as isize);
                        let fresh517 = p;
                        p = p.offset(1);
                        *fresh517 = (BOX_mp4a as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh518 = p;
                        p = p.offset(1);
                        *fresh518 = (BOX_mp4a as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh519 = p;
                        p = p.offset(1);
                        *fresh519 = (BOX_mp4a as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh520 = p;
                        p = p.offset(1);
                        *fresh520 = (BOX_mp4a as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    } else {
                        let fresh521 = stack;
                        stack = stack.offset(1);
                        *fresh521 = p;
                        p = p.offset(4 as c_int as isize);
                        let fresh522 = p;
                        p = p.offset(1);
                        *fresh522 = (BOX_mp4s as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh523 = p;
                        p = p.offset(1);
                        *fresh523 = (BOX_mp4s as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh524 = p;
                        p = p.offset(1);
                        *fresh524 = (BOX_mp4s as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh525 = p;
                        p = p.offset(1);
                        *fresh525 = (BOX_mp4s as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    }
                    let fresh526 = p;
                    p = p.offset(1);
                    *fresh526 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh527 = p;
                    p = p.offset(1);
                    *fresh527 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh528 = p;
                    p = p.offset(1);
                    *fresh528 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh529 = p;
                    p = p.offset(1);
                    *fresh529 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh530 = p;
                    p = p.offset(1);
                    *fresh530 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh531 = p;
                    p = p.offset(1);
                    *fresh531 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh532 = p;
                    p = p.offset(1);
                    *fresh532 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh533 = p;
                    p = p.offset(1);
                    *fresh533 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    if (*tr_1).info.track_media_kind as c_uint == e_audio as c_int as c_uint {
                        let fresh534 = p;
                        p = p.offset(1);
                        *fresh534 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh535 = p;
                        p = p.offset(1);
                        *fresh535 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh536 = p;
                        p = p.offset(1);
                        *fresh536 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh537 = p;
                        p = p.offset(1);
                        *fresh537 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh538 = p;
                        p = p.offset(1);
                        *fresh538 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh539 = p;
                        p = p.offset(1);
                        *fresh539 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh540 = p;
                        p = p.offset(1);
                        *fresh540 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh541 = p;
                        p = p.offset(1);
                        *fresh541 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh542 = p;
                        p = p.offset(1);
                        *fresh542 = ((*tr_1).info.u.a.channelcount >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh543 = p;
                        p = p.offset(1);
                        *fresh543 = ((*tr_1).info.u.a.channelcount >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh544 = p;
                        p = p.offset(1);
                        *fresh544 = (16 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh545 = p;
                        p = p.offset(1);
                        *fresh545 = (16 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh546 = p;
                        p = p.offset(1);
                        *fresh546 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh547 = p;
                        p = p.offset(1);
                        *fresh547 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh548 = p;
                        p = p.offset(1);
                        *fresh548 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh549 = p;
                        p = p.offset(1);
                        *fresh549 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh550 = p;
                        p = p.offset(1);
                        *fresh550 = ((*tr_1).info.time_scale << 16 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh551 = p;
                        p = p.offset(1);
                        *fresh551 = ((*tr_1).info.time_scale << 16 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh552 = p;
                        p = p.offset(1);
                        *fresh552 = ((*tr_1).info.time_scale << 16 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh553 = p;
                        p = p.offset(1);
                        *fresh553 = ((*tr_1).info.time_scale << 16 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    }
                    let fresh554 = stack;
                    stack = stack.offset(1);
                    *fresh554 = p;
                    p = p.offset(4 as c_int as isize);
                    let fresh555 = p;
                    p = p.offset(1);
                    *fresh555 = (BOX_esds as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh556 = p;
                    p = p.offset(1);
                    *fresh556 = (BOX_esds as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh557 = p;
                    p = p.offset(1);
                    *fresh557 = (BOX_esds as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh558 = p;
                    p = p.offset(1);
                    *fresh558 = (BOX_esds as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh559 = p;
                    p = p.offset(1);
                    *fresh559 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh560 = p;
                    p = p.offset(1);
                    *fresh560 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh561 = p;
                    p = p.offset(1);
                    *fresh561 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh562 = p;
                    p = p.offset(1);
                    *fresh562 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    if (*tr_1).vsps.bytes > 0 as c_int {
                        let mut dsi_bytes: c_int = (*tr_1).vsps.bytes - 2 as c_int;
                        let mut dsi_size_size: c_int = od_size_of_size(dsi_bytes);
                        let mut dcd_bytes: c_int = dsi_bytes
                            + dsi_size_size
                            + 1 as c_int
                            + (1 as c_int + 1 as c_int + 3 as c_int + 4 as c_int + 4 as c_int);
                        let mut dcd_size_size: c_int = od_size_of_size(dcd_bytes);
                        let mut esd_bytes: c_int = dcd_bytes + dcd_size_size + 1 as c_int + 3 as c_int;
                        let fresh563 = p;
                        p = p.offset(1);
                        *fresh563 = (3 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        if esd_bytes > 0x7f as c_int {
                            loop {
                                esd_bytes -= 0x7f as c_int;
                                let fresh564 = p;
                                p = p.offset(1);
                                *fresh564 = (0xff as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                                if esd_bytes <= 0x7f as c_int {
                                    break;
                                }
                            }
                        }
                        let fresh565 = p;
                        p = p.offset(1);
                        *fresh565 = (esd_bytes >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh566 = p;
                        p = p.offset(1);
                        *fresh566 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh567 = p;
                        p = p.offset(1);
                        *fresh567 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh568 = p;
                        p = p.offset(1);
                        *fresh568 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh569 = p;
                        p = p.offset(1);
                        *fresh569 = (4 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        if dcd_bytes > 0x7f as c_int {
                            loop {
                                dcd_bytes -= 0x7f as c_int;
                                let fresh570 = p;
                                p = p.offset(1);
                                *fresh570 = (0xff as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                                if dcd_bytes <= 0x7f as c_int {
                                    break;
                                }
                            }
                        }
                        let fresh571 = p;
                        p = p.offset(1);
                        *fresh571 = (dcd_bytes >> (8 as c_int * 0 as c_int)) as c_uchar;
                        if (*tr_1).info.track_media_kind as c_uint == e_audio as c_int as c_uint {
                            let fresh572 = p;
                            p = p.offset(1);
                            *fresh572 = (0x40 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                            let fresh573 = p;
                            p = p.offset(1);
                            *fresh573 = ((5 as c_int) << 2 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        } else {
                            let fresh574 = p;
                            p = p.offset(1);
                            *fresh574 = (208 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                            let fresh575 = p;
                            p = p.offset(1);
                            *fresh575 = ((32 as c_int) << 2 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        }
                        let fresh576 = p;
                        p = p.offset(1);
                        *fresh576 = (((*tr_1).info.u.a.channelcount)
                            .wrapping_mul(6144 as c_int as c_uint)
                            .wrapping_div(8 as c_int as c_uint)
                            >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh577 = p;
                        p = p.offset(1);
                        *fresh577 = (((*tr_1).info.u.a.channelcount)
                            .wrapping_mul(6144 as c_int as c_uint)
                            .wrapping_div(8 as c_int as c_uint)
                            >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh578 = p;
                        p = p.offset(1);
                        *fresh578 = (((*tr_1).info.u.a.channelcount)
                            .wrapping_mul(6144 as c_int as c_uint)
                            .wrapping_div(8 as c_int as c_uint)
                            >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh579 = p;
                        p = p.offset(1);
                        *fresh579 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh580 = p;
                        p = p.offset(1);
                        *fresh580 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh581 = p;
                        p = p.offset(1);
                        *fresh581 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh582 = p;
                        p = p.offset(1);
                        *fresh582 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh583 = p;
                        p = p.offset(1);
                        *fresh583 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh584 = p;
                        p = p.offset(1);
                        *fresh584 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh585 = p;
                        p = p.offset(1);
                        *fresh585 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh586 = p;
                        p = p.offset(1);
                        *fresh586 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh587 = p;
                        p = p.offset(1);
                        *fresh587 = (5 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        if dsi_bytes > 0x7f as c_int {
                            loop {
                                dsi_bytes -= 0x7f as c_int;
                                let fresh588 = p;
                                p = p.offset(1);
                                *fresh588 = (0xff as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                                if dsi_bytes <= 0x7f as c_int {
                                    break;
                                }
                            }
                        }
                        let fresh589 = p;
                        p = p.offset(1);
                        *fresh589 = (dsi_bytes >> (8 as c_int * 0 as c_int)) as c_uchar;
                        i = 0 as c_int;
                        while i < dsi_bytes {
                            let fresh590 = p;
                            p = p.offset(1);
                            *fresh590 = (*((*tr_1).vsps.data).offset((2 as c_int + i) as isize) as c_int
                                >> (8 as c_int * 0 as c_int)) as c_uchar;
                            i += 1;
                        }
                    }
                    stack = stack.offset(-1);
                    *(*stack).offset(0 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(1 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(2 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                    stack = stack.offset(-1);
                    *(*stack).offset(0 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(1 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(2 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                }
                if (*tr_1).info.track_media_kind as c_uint == e_video as c_int as c_uint
                    && (0x21 as c_int as c_uint == (*tr_1).info.object_type_indication
                        || 0x23 as c_int as c_uint == (*tr_1).info.object_type_indication)
                {
                    let mut numOfSequenceParameterSets: c_int = items_count(&mut (*tr_1).vsps);
                    let mut numOfPictureParameterSets: c_int = items_count(&mut (*tr_1).vpps);
                    if 0x21 as c_int as c_uint == (*tr_1).info.object_type_indication {
                        let fresh591 = stack;
                        stack = stack.offset(1);
                        *fresh591 = p;
                        p = p.offset(4 as c_int as isize);
                        let fresh592 = p;
                        p = p.offset(1);
                        *fresh592 = (BOX_avc1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh593 = p;
                        p = p.offset(1);
                        *fresh593 = (BOX_avc1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh594 = p;
                        p = p.offset(1);
                        *fresh594 = (BOX_avc1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh595 = p;
                        p = p.offset(1);
                        *fresh595 = (BOX_avc1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    } else {
                        let fresh596 = stack;
                        stack = stack.offset(1);
                        *fresh596 = p;
                        p = p.offset(4 as c_int as isize);
                        let fresh597 = p;
                        p = p.offset(1);
                        *fresh597 = (BOX_hvc1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh598 = p;
                        p = p.offset(1);
                        *fresh598 = (BOX_hvc1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh599 = p;
                        p = p.offset(1);
                        *fresh599 = (BOX_hvc1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh600 = p;
                        p = p.offset(1);
                        *fresh600 = (BOX_hvc1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    }
                    let fresh601 = p;
                    p = p.offset(1);
                    *fresh601 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh602 = p;
                    p = p.offset(1);
                    *fresh602 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh603 = p;
                    p = p.offset(1);
                    *fresh603 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh604 = p;
                    p = p.offset(1);
                    *fresh604 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh605 = p;
                    p = p.offset(1);
                    *fresh605 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh606 = p;
                    p = p.offset(1);
                    *fresh606 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh607 = p;
                    p = p.offset(1);
                    *fresh607 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh608 = p;
                    p = p.offset(1);
                    *fresh608 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh609 = p;
                    p = p.offset(1);
                    *fresh609 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh610 = p;
                    p = p.offset(1);
                    *fresh610 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh611 = p;
                    p = p.offset(1);
                    *fresh611 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh612 = p;
                    p = p.offset(1);
                    *fresh612 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh613 = p;
                    p = p.offset(1);
                    *fresh613 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh614 = p;
                    p = p.offset(1);
                    *fresh614 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh615 = p;
                    p = p.offset(1);
                    *fresh615 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh616 = p;
                    p = p.offset(1);
                    *fresh616 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh617 = p;
                    p = p.offset(1);
                    *fresh617 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh618 = p;
                    p = p.offset(1);
                    *fresh618 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh619 = p;
                    p = p.offset(1);
                    *fresh619 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh620 = p;
                    p = p.offset(1);
                    *fresh620 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh621 = p;
                    p = p.offset(1);
                    *fresh621 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh622 = p;
                    p = p.offset(1);
                    *fresh622 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh623 = p;
                    p = p.offset(1);
                    *fresh623 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh624 = p;
                    p = p.offset(1);
                    *fresh624 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh625 = p;
                    p = p.offset(1);
                    *fresh625 = ((*tr_1).info.u.v.width >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh626 = p;
                    p = p.offset(1);
                    *fresh626 = ((*tr_1).info.u.v.width >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh627 = p;
                    p = p.offset(1);
                    *fresh627 = ((*tr_1).info.u.v.height >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh628 = p;
                    p = p.offset(1);
                    *fresh628 = ((*tr_1).info.u.v.height >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh629 = p;
                    p = p.offset(1);
                    *fresh629 = (0x480000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh630 = p;
                    p = p.offset(1);
                    *fresh630 = (0x480000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh631 = p;
                    p = p.offset(1);
                    *fresh631 = (0x480000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh632 = p;
                    p = p.offset(1);
                    *fresh632 = (0x480000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh633 = p;
                    p = p.offset(1);
                    *fresh633 = (0x480000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh634 = p;
                    p = p.offset(1);
                    *fresh634 = (0x480000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh635 = p;
                    p = p.offset(1);
                    *fresh635 = (0x480000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh636 = p;
                    p = p.offset(1);
                    *fresh636 = (0x480000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh637 = p;
                    p = p.offset(1);
                    *fresh637 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh638 = p;
                    p = p.offset(1);
                    *fresh638 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh639 = p;
                    p = p.offset(1);
                    *fresh639 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh640 = p;
                    p = p.offset(1);
                    *fresh640 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh641 = p;
                    p = p.offset(1);
                    *fresh641 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh642 = p;
                    p = p.offset(1);
                    *fresh642 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    i = 0 as c_int;
                    while i < 32 as c_int {
                        let fresh643 = p;
                        p = p.offset(1);
                        *fresh643 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        i += 1;
                    }
                    let fresh644 = p;
                    p = p.offset(1);
                    *fresh644 = (24 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh645 = p;
                    p = p.offset(1);
                    *fresh645 = (24 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh646 = p;
                    p = p.offset(1);
                    *fresh646 = (-(1 as c_int) >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh647 = p;
                    p = p.offset(1);
                    *fresh647 = (-(1 as c_int) >> (8 as c_int * 0 as c_int)) as c_uchar;
                    if 0x21 as c_int as c_uint == (*tr_1).info.object_type_indication {
                        let fresh648 = stack;
                        stack = stack.offset(1);
                        *fresh648 = p;
                        p = p.offset(4 as c_int as isize);
                        let fresh649 = p;
                        p = p.offset(1);
                        *fresh649 = (BOX_avcC as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh650 = p;
                        p = p.offset(1);
                        *fresh650 = (BOX_avcC as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh651 = p;
                        p = p.offset(1);
                        *fresh651 = (BOX_avcC as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh652 = p;
                        p = p.offset(1);
                        *fresh652 = (BOX_avcC as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh653 = p;
                        p = p.offset(1);
                        *fresh653 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh654 = p;
                        p = p.offset(1);
                        *fresh654 = (*((*tr_1).vsps.data).offset((2 as c_int + 1 as c_int) as isize) as c_int
                            >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh655 = p;
                        p = p.offset(1);
                        *fresh655 = (*((*tr_1).vsps.data).offset((2 as c_int + 2 as c_int) as isize) as c_int
                            >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh656 = p;
                        p = p.offset(1);
                        *fresh656 = (*((*tr_1).vsps.data).offset((2 as c_int + 3 as c_int) as isize) as c_int
                            >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh657 = p;
                        p = p.offset(1);
                        *fresh657 = (255 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh658 = p;
                        p = p.offset(1);
                        *fresh658 =
                            ((0xe0 as c_int | numOfSequenceParameterSets) >> (8 as c_int * 0 as c_int)) as c_uchar;
                        i = 0 as c_int;
                        while i < (*tr_1).vsps.bytes {
                            let fresh659 = p;
                            p = p.offset(1);
                            *fresh659 = (*((*tr_1).vsps.data).offset(i as isize) as c_int >> (8 as c_int * 0 as c_int))
                                as c_uchar;
                            i += 1;
                        }
                        let fresh660 = p;
                        p = p.offset(1);
                        *fresh660 = (numOfPictureParameterSets >> (8 as c_int * 0 as c_int)) as c_uchar;
                        i = 0 as c_int;
                        while i < (*tr_1).vpps.bytes {
                            let fresh661 = p;
                            p = p.offset(1);
                            *fresh661 = (*((*tr_1).vpps.data).offset(i as isize) as c_int >> (8 as c_int * 0 as c_int))
                                as c_uchar;
                            i += 1;
                        }
                    } else {
                        let mut numOfVPS: c_int = items_count(&mut (*tr_1).vpps);
                        let fresh662 = stack;
                        stack = stack.offset(1);
                        *fresh662 = p;
                        p = p.offset(4 as c_int as isize);
                        let fresh663 = p;
                        p = p.offset(1);
                        *fresh663 = (BOX_hvcC as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh664 = p;
                        p = p.offset(1);
                        *fresh664 = (BOX_hvcC as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh665 = p;
                        p = p.offset(1);
                        *fresh665 = (BOX_hvcC as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh666 = p;
                        p = p.offset(1);
                        *fresh666 = (BOX_hvcC as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh667 = p;
                        p = p.offset(1);
                        *fresh667 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh668 = p;
                        p = p.offset(1);
                        *fresh668 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh669 = p;
                        p = p.offset(1);
                        *fresh669 = (0x60000000 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh670 = p;
                        p = p.offset(1);
                        *fresh670 = (0x60000000 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh671 = p;
                        p = p.offset(1);
                        *fresh671 = (0x60000000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh672 = p;
                        p = p.offset(1);
                        *fresh672 = (0x60000000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh673 = p;
                        p = p.offset(1);
                        *fresh673 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh674 = p;
                        p = p.offset(1);
                        *fresh674 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh675 = p;
                        p = p.offset(1);
                        *fresh675 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh676 = p;
                        p = p.offset(1);
                        *fresh676 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh677 = p;
                        p = p.offset(1);
                        *fresh677 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh678 = p;
                        p = p.offset(1);
                        *fresh678 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh679 = p;
                        p = p.offset(1);
                        *fresh679 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh680 = p;
                        p = p.offset(1);
                        *fresh680 = (0xf000 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh681 = p;
                        p = p.offset(1);
                        *fresh681 = (0xf000 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh682 = p;
                        p = p.offset(1);
                        *fresh682 = (0xfc as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh683 = p;
                        p = p.offset(1);
                        *fresh683 = (0xfc as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh684 = p;
                        p = p.offset(1);
                        *fresh684 = (0xf8 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh685 = p;
                        p = p.offset(1);
                        *fresh685 = (0xf8 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh686 = p;
                        p = p.offset(1);
                        *fresh686 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh687 = p;
                        p = p.offset(1);
                        *fresh687 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh688 = p;
                        p = p.offset(1);
                        *fresh688 = (3 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh689 = p;
                        p = p.offset(1);
                        *fresh689 = (3 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh690 = p;
                        p = p.offset(1);
                        *fresh690 = (((1 as c_int) << 7 as c_int | 32 as c_int & 0x3f as c_int)
                            >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh691 = p;
                        p = p.offset(1);
                        *fresh691 = (numOfVPS >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh692 = p;
                        p = p.offset(1);
                        *fresh692 = (numOfVPS >> (8 as c_int * 0 as c_int)) as c_uchar;
                        i = 0 as c_int;
                        while i < (*tr_1).vvps.bytes {
                            let fresh693 = p;
                            p = p.offset(1);
                            *fresh693 = (*((*tr_1).vvps.data).offset(i as isize) as c_int >> (8 as c_int * 0 as c_int))
                                as c_uchar;
                            i += 1;
                        }
                        let fresh694 = p;
                        p = p.offset(1);
                        *fresh694 = (((1 as c_int) << 7 as c_int | 33 as c_int & 0x3f as c_int)
                            >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh695 = p;
                        p = p.offset(1);
                        *fresh695 = (numOfSequenceParameterSets >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh696 = p;
                        p = p.offset(1);
                        *fresh696 = (numOfSequenceParameterSets >> (8 as c_int * 0 as c_int)) as c_uchar;
                        i = 0 as c_int;
                        while i < (*tr_1).vsps.bytes {
                            let fresh697 = p;
                            p = p.offset(1);
                            *fresh697 = (*((*tr_1).vsps.data).offset(i as isize) as c_int >> (8 as c_int * 0 as c_int))
                                as c_uchar;
                            i += 1;
                        }
                        let fresh698 = p;
                        p = p.offset(1);
                        *fresh698 = (((1 as c_int) << 7 as c_int | 34 as c_int & 0x3f as c_int)
                            >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh699 = p;
                        p = p.offset(1);
                        *fresh699 = (numOfPictureParameterSets >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh700 = p;
                        p = p.offset(1);
                        *fresh700 = (numOfPictureParameterSets >> (8 as c_int * 0 as c_int)) as c_uchar;
                        i = 0 as c_int;
                        while i < (*tr_1).vpps.bytes {
                            let fresh701 = p;
                            p = p.offset(1);
                            *fresh701 = (*((*tr_1).vpps.data).offset(i as isize) as c_int >> (8 as c_int * 0 as c_int))
                                as c_uchar;
                            i += 1;
                        }
                    }
                    stack = stack.offset(-1);
                    *(*stack).offset(0 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(1 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(2 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                    stack = stack.offset(-1);
                    *(*stack).offset(0 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(1 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(2 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                }
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                let fresh702 = stack;
                stack = stack.offset(1);
                *fresh702 = p;
                p = p.offset(4 as c_int as isize);
                let fresh703 = p;
                p = p.offset(1);
                *fresh703 = (BOX_stts as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh704 = p;
                p = p.offset(1);
                *fresh704 = (BOX_stts as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh705 = p;
                p = p.offset(1);
                *fresh705 = (BOX_stts as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh706 = p;
                p = p.offset(1);
                *fresh706 = (BOX_stts as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh707 = p;
                p = p.offset(1);
                *fresh707 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh708 = p;
                p = p.offset(1);
                *fresh708 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh709 = p;
                p = p.offset(1);
                *fresh709 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh710 = p;
                p = p.offset(1);
                *fresh710 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let mut pentry_count: *mut c_uchar = p;
                let mut cnt: c_int = 1 as c_int;
                let mut entry_count: c_int = 0 as c_int;
                let fresh711 = p;
                p = p.offset(1);
                *fresh711 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh712 = p;
                p = p.offset(1);
                *fresh712 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh713 = p;
                p = p.offset(1);
                *fresh713 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh714 = p;
                p = p.offset(1);
                *fresh714 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                i = 0 as c_int;
                while i < samples_count {
                    if i == samples_count - 1 as c_int
                        || (*sample.offset(i as isize)).duration != (*sample.offset((i + 1 as c_int) as isize)).duration
                    {
                        let fresh715 = p;
                        p = p.offset(1);
                        *fresh715 = (cnt >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh716 = p;
                        p = p.offset(1);
                        *fresh716 = (cnt >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh717 = p;
                        p = p.offset(1);
                        *fresh717 = (cnt >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh718 = p;
                        p = p.offset(1);
                        *fresh718 = (cnt >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh719 = p;
                        p = p.offset(1);
                        *fresh719 = ((*sample.offset(i as isize)).duration >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh720 = p;
                        p = p.offset(1);
                        *fresh720 = ((*sample.offset(i as isize)).duration >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh721 = p;
                        p = p.offset(1);
                        *fresh721 = ((*sample.offset(i as isize)).duration >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh722 = p;
                        p = p.offset(1);
                        *fresh722 = ((*sample.offset(i as isize)).duration >> (8 as c_int * 0 as c_int)) as c_uchar;
                        cnt = 0 as c_int;
                        entry_count += 1;
                    }
                    i += 1;
                    cnt += 1;
                }
                *pentry_count.offset(0 as c_int as isize) =
                    (entry_count >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *pentry_count.offset(1 as c_int as isize) =
                    (entry_count >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *pentry_count.offset(2 as c_int as isize) =
                    (entry_count >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *pentry_count.offset(3 as c_int as isize) = entry_count as c_char as c_uchar;
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                let fresh723 = stack;
                stack = stack.offset(1);
                *fresh723 = p;
                p = p.offset(4 as c_int as isize);
                let fresh724 = p;
                p = p.offset(1);
                *fresh724 = (BOX_stsc as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh725 = p;
                p = p.offset(1);
                *fresh725 = (BOX_stsc as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh726 = p;
                p = p.offset(1);
                *fresh726 = (BOX_stsc as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh727 = p;
                p = p.offset(1);
                *fresh727 = (BOX_stsc as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh728 = p;
                p = p.offset(1);
                *fresh728 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh729 = p;
                p = p.offset(1);
                *fresh729 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh730 = p;
                p = p.offset(1);
                *fresh730 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh731 = p;
                p = p.offset(1);
                *fresh731 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                if (*mux).enable_fragmentation != 0 {
                    let fresh732 = p;
                    p = p.offset(1);
                    *fresh732 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh733 = p;
                    p = p.offset(1);
                    *fresh733 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh734 = p;
                    p = p.offset(1);
                    *fresh734 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh735 = p;
                    p = p.offset(1);
                    *fresh735 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                } else {
                    let fresh736 = p;
                    p = p.offset(1);
                    *fresh736 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh737 = p;
                    p = p.offset(1);
                    *fresh737 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh738 = p;
                    p = p.offset(1);
                    *fresh738 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh739 = p;
                    p = p.offset(1);
                    *fresh739 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh740 = p;
                    p = p.offset(1);
                    *fresh740 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh741 = p;
                    p = p.offset(1);
                    *fresh741 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh742 = p;
                    p = p.offset(1);
                    *fresh742 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh743 = p;
                    p = p.offset(1);
                    *fresh743 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh744 = p;
                    p = p.offset(1);
                    *fresh744 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh745 = p;
                    p = p.offset(1);
                    *fresh745 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh746 = p;
                    p = p.offset(1);
                    *fresh746 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh747 = p;
                    p = p.offset(1);
                    *fresh747 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh748 = p;
                    p = p.offset(1);
                    *fresh748 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh749 = p;
                    p = p.offset(1);
                    *fresh749 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh750 = p;
                    p = p.offset(1);
                    *fresh750 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh751 = p;
                    p = p.offset(1);
                    *fresh751 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                }
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                let fresh752 = stack;
                stack = stack.offset(1);
                *fresh752 = p;
                p = p.offset(4 as c_int as isize);
                let fresh753 = p;
                p = p.offset(1);
                *fresh753 = (BOX_stsz as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh754 = p;
                p = p.offset(1);
                *fresh754 = (BOX_stsz as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh755 = p;
                p = p.offset(1);
                *fresh755 = (BOX_stsz as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh756 = p;
                p = p.offset(1);
                *fresh756 = (BOX_stsz as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh757 = p;
                p = p.offset(1);
                *fresh757 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh758 = p;
                p = p.offset(1);
                *fresh758 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh759 = p;
                p = p.offset(1);
                *fresh759 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh760 = p;
                p = p.offset(1);
                *fresh760 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh761 = p;
                p = p.offset(1);
                *fresh761 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh762 = p;
                p = p.offset(1);
                *fresh762 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh763 = p;
                p = p.offset(1);
                *fresh763 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh764 = p;
                p = p.offset(1);
                *fresh764 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                let fresh765 = p;
                p = p.offset(1);
                *fresh765 = (samples_count >> (8 as c_int * 3 as c_int)) as c_uchar;
                let fresh766 = p;
                p = p.offset(1);
                *fresh766 = (samples_count >> (8 as c_int * 2 as c_int)) as c_uchar;
                let fresh767 = p;
                p = p.offset(1);
                *fresh767 = (samples_count >> (8 as c_int * 1 as c_int)) as c_uchar;
                let fresh768 = p;
                p = p.offset(1);
                *fresh768 = (samples_count >> (8 as c_int * 0 as c_int)) as c_uchar;
                i = 0 as c_int;
                while i < samples_count {
                    let fresh769 = p;
                    p = p.offset(1);
                    *fresh769 = ((*sample.offset(i as isize)).size >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh770 = p;
                    p = p.offset(1);
                    *fresh770 = ((*sample.offset(i as isize)).size >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh771 = p;
                    p = p.offset(1);
                    *fresh771 = ((*sample.offset(i as isize)).size >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh772 = p;
                    p = p.offset(1);
                    *fresh772 = ((*sample.offset(i as isize)).size >> (8 as c_int * 0 as c_int)) as c_uchar;
                    i += 1;
                }
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                let mut is_64_bit: c_int = 0 as c_int;
                if samples_count != 0
                    && (*sample.offset((samples_count - 1 as c_int) as isize)).offset > 0xffffffff as c_uint as c_ulong
                {
                    is_64_bit = 1 as c_int;
                }
                if is_64_bit == 0 {
                    let fresh773 = stack;
                    stack = stack.offset(1);
                    *fresh773 = p;
                    p = p.offset(4 as c_int as isize);
                    let fresh774 = p;
                    p = p.offset(1);
                    *fresh774 = (BOX_stco as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh775 = p;
                    p = p.offset(1);
                    *fresh775 = (BOX_stco as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh776 = p;
                    p = p.offset(1);
                    *fresh776 = (BOX_stco as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh777 = p;
                    p = p.offset(1);
                    *fresh777 = (BOX_stco as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh778 = p;
                    p = p.offset(1);
                    *fresh778 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh779 = p;
                    p = p.offset(1);
                    *fresh779 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh780 = p;
                    p = p.offset(1);
                    *fresh780 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh781 = p;
                    p = p.offset(1);
                    *fresh781 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh782 = p;
                    p = p.offset(1);
                    *fresh782 = (samples_count >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh783 = p;
                    p = p.offset(1);
                    *fresh783 = (samples_count >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh784 = p;
                    p = p.offset(1);
                    *fresh784 = (samples_count >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh785 = p;
                    p = p.offset(1);
                    *fresh785 = (samples_count >> (8 as c_int * 0 as c_int)) as c_uchar;
                    i = 0 as c_int;
                    while i < samples_count {
                        let fresh786 = p;
                        p = p.offset(1);
                        *fresh786 = ((*sample.offset(i as isize)).offset >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh787 = p;
                        p = p.offset(1);
                        *fresh787 = ((*sample.offset(i as isize)).offset >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh788 = p;
                        p = p.offset(1);
                        *fresh788 = ((*sample.offset(i as isize)).offset >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh789 = p;
                        p = p.offset(1);
                        *fresh789 = ((*sample.offset(i as isize)).offset >> (8 as c_int * 0 as c_int)) as c_uchar;
                        i += 1;
                    }
                } else {
                    let fresh790 = stack;
                    stack = stack.offset(1);
                    *fresh790 = p;
                    p = p.offset(4 as c_int as isize);
                    let fresh791 = p;
                    p = p.offset(1);
                    *fresh791 = (BOX_co64 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh792 = p;
                    p = p.offset(1);
                    *fresh792 = (BOX_co64 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh793 = p;
                    p = p.offset(1);
                    *fresh793 = (BOX_co64 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh794 = p;
                    p = p.offset(1);
                    *fresh794 = (BOX_co64 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh795 = p;
                    p = p.offset(1);
                    *fresh795 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh796 = p;
                    p = p.offset(1);
                    *fresh796 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh797 = p;
                    p = p.offset(1);
                    *fresh797 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh798 = p;
                    p = p.offset(1);
                    *fresh798 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh799 = p;
                    p = p.offset(1);
                    *fresh799 = (samples_count >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh800 = p;
                    p = p.offset(1);
                    *fresh800 = (samples_count >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh801 = p;
                    p = p.offset(1);
                    *fresh801 = (samples_count >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh802 = p;
                    p = p.offset(1);
                    *fresh802 = (samples_count >> (8 as c_int * 0 as c_int)) as c_uchar;
                    i = 0 as c_int;
                    while i < samples_count {
                        let fresh803 = p;
                        p = p.offset(1);
                        *fresh803 = (((*sample.offset(i as isize)).offset >> 32 as c_int
                            & 0xffffffff as c_uint as c_ulong)
                            >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh804 = p;
                        p = p.offset(1);
                        *fresh804 = (((*sample.offset(i as isize)).offset >> 32 as c_int
                            & 0xffffffff as c_uint as c_ulong)
                            >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh805 = p;
                        p = p.offset(1);
                        *fresh805 = (((*sample.offset(i as isize)).offset >> 32 as c_int
                            & 0xffffffff as c_uint as c_ulong)
                            >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh806 = p;
                        p = p.offset(1);
                        *fresh806 = (((*sample.offset(i as isize)).offset >> 32 as c_int
                            & 0xffffffff as c_uint as c_ulong)
                            >> (8 as c_int * 0 as c_int)) as c_uchar;
                        let fresh807 = p;
                        p = p.offset(1);
                        *fresh807 = (((*sample.offset(i as isize)).offset & 0xffffffff as c_uint as c_ulong)
                            >> (8 as c_int * 3 as c_int)) as c_uchar;
                        let fresh808 = p;
                        p = p.offset(1);
                        *fresh808 = (((*sample.offset(i as isize)).offset & 0xffffffff as c_uint as c_ulong)
                            >> (8 as c_int * 2 as c_int)) as c_uchar;
                        let fresh809 = p;
                        p = p.offset(1);
                        *fresh809 = (((*sample.offset(i as isize)).offset & 0xffffffff as c_uint as c_ulong)
                            >> (8 as c_int * 1 as c_int)) as c_uchar;
                        let fresh810 = p;
                        p = p.offset(1);
                        *fresh810 = (((*sample.offset(i as isize)).offset & 0xffffffff as c_uint as c_ulong)
                            >> (8 as c_int * 0 as c_int)) as c_uchar;
                        i += 1;
                    }
                }
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                let mut ra_count: c_int = 0 as c_int;
                i = 0 as c_int;
                while i < samples_count {
                    ra_count += ((*sample.offset(i as isize)).flag_random_access != 0) as c_int;
                    i += 1;
                }
                if ra_count != samples_count {
                    let fresh811 = stack;
                    stack = stack.offset(1);
                    *fresh811 = p;
                    p = p.offset(4 as c_int as isize);
                    let fresh812 = p;
                    p = p.offset(1);
                    *fresh812 = (BOX_stss as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh813 = p;
                    p = p.offset(1);
                    *fresh813 = (BOX_stss as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh814 = p;
                    p = p.offset(1);
                    *fresh814 = (BOX_stss as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh815 = p;
                    p = p.offset(1);
                    *fresh815 = (BOX_stss as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh816 = p;
                    p = p.offset(1);
                    *fresh816 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh817 = p;
                    p = p.offset(1);
                    *fresh817 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh818 = p;
                    p = p.offset(1);
                    *fresh818 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh819 = p;
                    p = p.offset(1);
                    *fresh819 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
                    let fresh820 = p;
                    p = p.offset(1);
                    *fresh820 = (ra_count >> (8 as c_int * 3 as c_int)) as c_uchar;
                    let fresh821 = p;
                    p = p.offset(1);
                    *fresh821 = (ra_count >> (8 as c_int * 2 as c_int)) as c_uchar;
                    let fresh822 = p;
                    p = p.offset(1);
                    *fresh822 = (ra_count >> (8 as c_int * 1 as c_int)) as c_uchar;
                    let fresh823 = p;
                    p = p.offset(1);
                    *fresh823 = (ra_count >> (8 as c_int * 0 as c_int)) as c_uchar;
                    i = 0 as c_int;
                    while i < samples_count {
                        if (*sample.offset(i as isize)).flag_random_access != 0 {
                            let fresh824 = p;
                            p = p.offset(1);
                            *fresh824 = ((i + 1 as c_int) >> (8 as c_int * 3 as c_int)) as c_uchar;
                            let fresh825 = p;
                            p = p.offset(1);
                            *fresh825 = ((i + 1 as c_int) >> (8 as c_int * 2 as c_int)) as c_uchar;
                            let fresh826 = p;
                            p = p.offset(1);
                            *fresh826 = ((i + 1 as c_int) >> (8 as c_int * 1 as c_int)) as c_uchar;
                            let fresh827 = p;
                            p = p.offset(1);
                            *fresh827 = ((i + 1 as c_int) >> (8 as c_int * 0 as c_int)) as c_uchar;
                        }
                        i += 1;
                    }
                    stack = stack.offset(-1);
                    *(*stack).offset(0 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(1 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(2 as c_int as isize) =
                        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                }
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
                stack = stack.offset(-1);
                *(*stack).offset(0 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
                *(*stack).offset(1 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
                *(*stack).offset(2 as c_int as isize) =
                    (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
                *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
            }
            _ => {}
        }
        ntr = ntr.wrapping_add(1);
    }
    if !((*mux).text_comment).is_null() {
        let fresh828 = stack;
        stack = stack.offset(1);
        *fresh828 = p;
        p = p.offset(4 as c_int as isize);
        let fresh829 = p;
        p = p.offset(1);
        *fresh829 = (BOX_udta as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh830 = p;
        p = p.offset(1);
        *fresh830 = (BOX_udta as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh831 = p;
        p = p.offset(1);
        *fresh831 = (BOX_udta as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh832 = p;
        p = p.offset(1);
        *fresh832 = (BOX_udta as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh833 = stack;
        stack = stack.offset(1);
        *fresh833 = p;
        p = p.offset(4 as c_int as isize);
        let fresh834 = p;
        p = p.offset(1);
        *fresh834 = (BOX_meta as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh835 = p;
        p = p.offset(1);
        *fresh835 = (BOX_meta as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh836 = p;
        p = p.offset(1);
        *fresh836 = (BOX_meta as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh837 = p;
        p = p.offset(1);
        *fresh837 = (BOX_meta as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh838 = p;
        p = p.offset(1);
        *fresh838 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh839 = p;
        p = p.offset(1);
        *fresh839 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh840 = p;
        p = p.offset(1);
        *fresh840 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh841 = p;
        p = p.offset(1);
        *fresh841 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh842 = stack;
        stack = stack.offset(1);
        *fresh842 = p;
        p = p.offset(4 as c_int as isize);
        let fresh843 = p;
        p = p.offset(1);
        *fresh843 = (BOX_hdlr as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh844 = p;
        p = p.offset(1);
        *fresh844 = (BOX_hdlr as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh845 = p;
        p = p.offset(1);
        *fresh845 = (BOX_hdlr as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh846 = p;
        p = p.offset(1);
        *fresh846 = (BOX_hdlr as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh847 = p;
        p = p.offset(1);
        *fresh847 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh848 = p;
        p = p.offset(1);
        *fresh848 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh849 = p;
        p = p.offset(1);
        *fresh849 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh850 = p;
        p = p.offset(1);
        *fresh850 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh851 = p;
        p = p.offset(1);
        *fresh851 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh852 = p;
        p = p.offset(1);
        *fresh852 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh853 = p;
        p = p.offset(1);
        *fresh853 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh854 = p;
        p = p.offset(1);
        *fresh854 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh855 = p;
        p = p.offset(1);
        *fresh855 = (0x6d646972 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh856 = p;
        p = p.offset(1);
        *fresh856 = (0x6d646972 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh857 = p;
        p = p.offset(1);
        *fresh857 = (0x6d646972 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh858 = p;
        p = p.offset(1);
        *fresh858 = (0x6d646972 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh859 = p;
        p = p.offset(1);
        *fresh859 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh860 = p;
        p = p.offset(1);
        *fresh860 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh861 = p;
        p = p.offset(1);
        *fresh861 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh862 = p;
        p = p.offset(1);
        *fresh862 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh863 = p;
        p = p.offset(1);
        *fresh863 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh864 = p;
        p = p.offset(1);
        *fresh864 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh865 = p;
        p = p.offset(1);
        *fresh865 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh866 = p;
        p = p.offset(1);
        *fresh866 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh867 = p;
        p = p.offset(1);
        *fresh867 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh868 = p;
        p = p.offset(1);
        *fresh868 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh869 = p;
        p = p.offset(1);
        *fresh869 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh870 = p;
        p = p.offset(1);
        *fresh870 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh871 = p;
        p = p.offset(1);
        *fresh871 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh872 = p;
        p = p.offset(1);
        *fresh872 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh873 = p;
        p = p.offset(1);
        *fresh873 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh874 = p;
        p = p.offset(1);
        *fresh874 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
        let fresh875 = stack;
        stack = stack.offset(1);
        *fresh875 = p;
        p = p.offset(4 as c_int as isize);
        let fresh876 = p;
        p = p.offset(1);
        *fresh876 = (BOX_ilst as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh877 = p;
        p = p.offset(1);
        *fresh877 = (BOX_ilst as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh878 = p;
        p = p.offset(1);
        *fresh878 = (BOX_ilst as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh879 = p;
        p = p.offset(1);
        *fresh879 = (BOX_ilst as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh880 = stack;
        stack = stack.offset(1);
        *fresh880 = p;
        p = p.offset(4 as c_int as isize);
        let fresh881 = p;
        p = p.offset(1);
        *fresh881 = (BOX_ccmt as c_uint >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh882 = p;
        p = p.offset(1);
        *fresh882 = (BOX_ccmt as c_uint >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh883 = p;
        p = p.offset(1);
        *fresh883 = (BOX_ccmt as c_uint >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh884 = p;
        p = p.offset(1);
        *fresh884 = (BOX_ccmt as c_uint >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh885 = stack;
        stack = stack.offset(1);
        *fresh885 = p;
        p = p.offset(4 as c_int as isize);
        let fresh886 = p;
        p = p.offset(1);
        *fresh886 = (BOX_data as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh887 = p;
        p = p.offset(1);
        *fresh887 = (BOX_data as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh888 = p;
        p = p.offset(1);
        *fresh888 = (BOX_data as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh889 = p;
        p = p.offset(1);
        *fresh889 = (BOX_data as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh890 = p;
        p = p.offset(1);
        *fresh890 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh891 = p;
        p = p.offset(1);
        *fresh891 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh892 = p;
        p = p.offset(1);
        *fresh892 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh893 = p;
        p = p.offset(1);
        *fresh893 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh894 = p;
        p = p.offset(1);
        *fresh894 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh895 = p;
        p = p.offset(1);
        *fresh895 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh896 = p;
        p = p.offset(1);
        *fresh896 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh897 = p;
        p = p.offset(1);
        *fresh897 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        i = 0 as c_int;
        while i < minimp4_strlen((*mux).text_comment) as c_int + 1 as c_int {
            let fresh898 = p;
            p = p.offset(1);
            *fresh898 = (*((*mux).text_comment).offset(i as isize) as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            i += 1;
        }
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    }
    if (*mux).enable_fragmentation != 0 {
        let mut tr_2: *mut track_t = ((*mux).tracks.data as *mut track_t).offset(0 as c_int as isize);
        let mut movie_duration: uint32_t = get_duration(tr_2);
        let fresh899 = stack;
        stack = stack.offset(1);
        *fresh899 = p;
        p = p.offset(4 as c_int as isize);
        let fresh900 = p;
        p = p.offset(1);
        *fresh900 = (BOX_mvex as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh901 = p;
        p = p.offset(1);
        *fresh901 = (BOX_mvex as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh902 = p;
        p = p.offset(1);
        *fresh902 = (BOX_mvex as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh903 = p;
        p = p.offset(1);
        *fresh903 = (BOX_mvex as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh904 = stack;
        stack = stack.offset(1);
        *fresh904 = p;
        p = p.offset(4 as c_int as isize);
        let fresh905 = p;
        p = p.offset(1);
        *fresh905 = (BOX_mehd as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh906 = p;
        p = p.offset(1);
        *fresh906 = (BOX_mehd as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh907 = p;
        p = p.offset(1);
        *fresh907 = (BOX_mehd as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh908 = p;
        p = p.offset(1);
        *fresh908 = (BOX_mehd as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh909 = p;
        p = p.offset(1);
        *fresh909 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh910 = p;
        p = p.offset(1);
        *fresh910 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh911 = p;
        p = p.offset(1);
        *fresh911 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh912 = p;
        p = p.offset(1);
        *fresh912 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
        let fresh913 = p;
        p = p.offset(1);
        *fresh913 = (movie_duration >> (8 as c_int * 3 as c_int)) as c_uchar;
        let fresh914 = p;
        p = p.offset(1);
        *fresh914 = (movie_duration >> (8 as c_int * 2 as c_int)) as c_uchar;
        let fresh915 = p;
        p = p.offset(1);
        *fresh915 = (movie_duration >> (8 as c_int * 1 as c_int)) as c_uchar;
        let fresh916 = p;
        p = p.offset(1);
        *fresh916 = (movie_duration >> (8 as c_int * 0 as c_int)) as c_uchar;
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
        ntr = 0 as c_int as c_uint;
        while ntr < ntracks {
            let fresh917 = stack;
            stack = stack.offset(1);
            *fresh917 = p;
            p = p.offset(4 as c_int as isize);
            let fresh918 = p;
            p = p.offset(1);
            *fresh918 = (BOX_trex as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh919 = p;
            p = p.offset(1);
            *fresh919 = (BOX_trex as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh920 = p;
            p = p.offset(1);
            *fresh920 = (BOX_trex as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh921 = p;
            p = p.offset(1);
            *fresh921 = (BOX_trex as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh922 = p;
            p = p.offset(1);
            *fresh922 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh923 = p;
            p = p.offset(1);
            *fresh923 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh924 = p;
            p = p.offset(1);
            *fresh924 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh925 = p;
            p = p.offset(1);
            *fresh925 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh926 = p;
            p = p.offset(1);
            *fresh926 = (ntr.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh927 = p;
            p = p.offset(1);
            *fresh927 = (ntr.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh928 = p;
            p = p.offset(1);
            *fresh928 = (ntr.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh929 = p;
            p = p.offset(1);
            *fresh929 = (ntr.wrapping_add(1 as c_int as c_uint) >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh930 = p;
            p = p.offset(1);
            *fresh930 = (1 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh931 = p;
            p = p.offset(1);
            *fresh931 = (1 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh932 = p;
            p = p.offset(1);
            *fresh932 = (1 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh933 = p;
            p = p.offset(1);
            *fresh933 = (1 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh934 = p;
            p = p.offset(1);
            *fresh934 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh935 = p;
            p = p.offset(1);
            *fresh935 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh936 = p;
            p = p.offset(1);
            *fresh936 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh937 = p;
            p = p.offset(1);
            *fresh937 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh938 = p;
            p = p.offset(1);
            *fresh938 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh939 = p;
            p = p.offset(1);
            *fresh939 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh940 = p;
            p = p.offset(1);
            *fresh940 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh941 = p;
            p = p.offset(1);
            *fresh941 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            let fresh942 = p;
            p = p.offset(1);
            *fresh942 = (0 as c_int >> (8 as c_int * 3 as c_int)) as c_uchar;
            let fresh943 = p;
            p = p.offset(1);
            *fresh943 = (0 as c_int >> (8 as c_int * 2 as c_int)) as c_uchar;
            let fresh944 = p;
            p = p.offset(1);
            *fresh944 = (0 as c_int >> (8 as c_int * 1 as c_int)) as c_uchar;
            let fresh945 = p;
            p = p.offset(1);
            *fresh945 = (0 as c_int >> (8 as c_int * 0 as c_int)) as c_uchar;
            stack = stack.offset(-1);
            *(*stack).offset(0 as c_int as isize) =
                (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
            *(*stack).offset(1 as c_int as isize) =
                (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
            *(*stack).offset(2 as c_int as isize) =
                (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
            *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
            ntr = ntr.wrapping_add(1);
        }
        stack = stack.offset(-1);
        *(*stack).offset(0 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
        *(*stack).offset(1 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
        *(*stack).offset(2 as c_int as isize) =
            (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
        *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    }
    stack = stack.offset(-1);
    *(*stack).offset(0 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 3 as c_int)) as c_char as c_uchar;
    *(*stack).offset(1 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 2 as c_int)) as c_char as c_uchar;
    *(*stack).offset(2 as c_int as isize) =
        (p.offset_from(*stack) as c_long >> (8 as c_int * 1 as c_int)) as c_char as c_uchar;
    *(*stack).offset(3 as c_int as isize) = p.offset_from(*stack) as c_long as c_char as c_uchar;
    if p.offset_from(base) as c_long as c_uint <= index_bytes {
    } else {
        __assert_fail(
            b"(unsigned)(p - base) <= index_bytes\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1734 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 35], &[c_char; 35]>(b"int mp4e_flush_index(MP4E_mux_t *)\0")).as_ptr(),
        );
    }
    if p.offset_from(base) as c_long as c_uint <= index_bytes {
    } else {
        __assert_fail(
            b"(unsigned)(p - base) <= index_bytes\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1734 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 35], &[c_char; 35]>(b"int mp4e_flush_index(MP4E_mux_t *)\0")).as_ptr(),
        );
    };
    err = ((*mux).write_callback).expect("non-null function pointer")(
        (*mux).write_pos,
        base as *const c_void,
        p.offset_from(base) as c_long as size_t,
        (*mux).token,
    );
    (*mux).write_pos += p.offset_from(base) as c_long;
    minimp4_free(base as *mut c_void);
    err
}
#[no_mangle]
pub unsafe extern "C" fn MP4E_close(mut mux: *mut MP4E_mux_t) -> c_int {
    let mut err: c_int = 0 as c_int;
    let mut ntr: c_uint = 0;
    let mut ntracks: c_uint = 0;
    if mux.is_null() {
        return -(1 as c_int);
    }
    if (*mux).enable_fragmentation == 0 {
        err = mp4e_flush_index(mux);
    }
    if !((*mux).text_comment).is_null() {
        minimp4_free((*mux).text_comment as *mut c_void);
    }
    ntracks = ((*mux).tracks.bytes as c_ulong).wrapping_div(::core::mem::size_of::<track_t>() as c_ulong) as c_uint;
    ntr = 0 as c_int as c_uint;
    while ntr < ntracks {
        let mut tr: *mut track_t = ((*mux).tracks.data as *mut track_t).offset(ntr as isize);
        minimp4_vector_reset(&mut (*tr).vsps);
        minimp4_vector_reset(&mut (*tr).vpps);
        minimp4_vector_reset(&mut (*tr).smpl);
        minimp4_vector_reset(&mut (*tr).pending_sample);
        ntr = ntr.wrapping_add(1);
    }
    minimp4_vector_reset(&mut (*mux).tracks);
    minimp4_free(mux as *mut c_void);
    err
}
unsafe extern "C" fn show_bits(mut bs: *mut bit_reader_t, mut n: c_int) -> c_uint {
    let mut retval: c_uint = 0;
    if n > 0 as c_int && n <= 16 as c_int {
    } else {
        __assert_fail(
            b"n > 0 && n <= 16\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1794 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 44], &[c_char; 44]>(b"unsigned int show_bits(bit_reader_t *, int)\0"))
                .as_ptr(),
        );
    }
    if n > 0 as c_int && n <= 16 as c_int {
    } else {
        __assert_fail(
            b"n > 0 && n <= 16\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1794 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 44], &[c_char; 44]>(b"unsigned int show_bits(bit_reader_t *, int)\0"))
                .as_ptr(),
        );
    };
    retval = (*bs).cache >> (32 as c_int - n);
    retval
}
unsafe extern "C" fn flush_bits(mut bs: *mut bit_reader_t, mut n: c_int) {
    if n >= 0 as c_int && n <= 16 as c_int {
    } else {
        __assert_fail(
            b"n >= 0 && n <= 16\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1801 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 37], &[c_char; 37]>(b"void flush_bits(bit_reader_t *, int)\0")).as_ptr(),
        );
    }
    if n >= 0 as c_int && n <= 16 as c_int {
    } else {
        __assert_fail(
            b"n >= 0 && n <= 16\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1801 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 37], &[c_char; 37]>(b"void flush_bits(bit_reader_t *, int)\0")).as_ptr(),
        );
    };
    (*bs).cache <<= n;
    (*bs).cache_free_bits += n;
    if (*bs).cache_free_bits >= 0 as c_int {
        let buf = (*bs).buf.read_unaligned();
        (*bs).cache |= ((((buf as c_int) << 8 as c_int) as uint16_t as c_int | buf as c_int >> 8 as c_int) as uint32_t)
            << (*bs).cache_free_bits;
        (*bs).buf = ((*bs).buf).offset(1);
        (*bs).cache_free_bits -= 16 as c_int;
    }
}
unsafe extern "C" fn get_bits(mut bs: *mut bit_reader_t, mut n: c_int) -> c_uint {
    let mut retval: c_uint = show_bits(bs, n);
    flush_bits(bs, n);
    retval
}
unsafe extern "C" fn set_pos_bits(mut bs: *mut bit_reader_t, mut pos_bits: c_uint) {
    if pos_bits as c_int >= 0 as c_int {
    } else {
        __assert_fail(
            b"(int)pos_bits >= 0\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1821 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 48], &[c_char; 48]>(b"void set_pos_bits(bit_reader_t *, unsigned int)\0"))
                .as_ptr(),
        );
    }
    if pos_bits as c_int >= 0 as c_int {
    } else {
        __assert_fail(
            b"(int)pos_bits >= 0\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1821 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 48], &[c_char; 48]>(b"void set_pos_bits(bit_reader_t *, unsigned int)\0"))
                .as_ptr(),
        );
    };
    (*bs).buf = ((*bs).origin).offset(pos_bits.wrapping_div(16 as c_int as c_uint) as isize);
    (*bs).cache = 0 as c_int as c_uint;
    (*bs).cache_free_bits = 16 as c_int;
    flush_bits(bs, 0 as c_int);
    flush_bits(bs, (pos_bits & 15 as c_int as c_uint) as c_int);
}
unsafe extern "C" fn get_pos_bits(mut bs: *const bit_reader_t) -> c_uint {
    let mut pos_bits: c_uint =
        (((*bs).buf).offset_from((*bs).origin) as c_long as c_uint).wrapping_mul(16 as c_int as c_uint);
    pos_bits = pos_bits.wrapping_sub((16 as c_int - (*bs).cache_free_bits) as c_uint);
    if pos_bits as c_int >= 0 as c_int {
    } else {
        __assert_fail(
            b"(int)pos_bits >= 0\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1837 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 48], &[c_char; 48]>(b"unsigned int get_pos_bits(const bit_reader_t *)\0"))
                .as_ptr(),
        );
    }
    if pos_bits as c_int >= 0 as c_int {
    } else {
        __assert_fail(
            b"(int)pos_bits >= 0\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1837 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 48], &[c_char; 48]>(b"unsigned int get_pos_bits(const bit_reader_t *)\0"))
                .as_ptr(),
        );
    };
    pos_bits
}
unsafe extern "C" fn remaining_bits(mut bs: *const bit_reader_t) -> c_int {
    ((*bs).origin_bytes)
        .wrapping_mul(8 as c_int as c_uint)
        .wrapping_sub(get_pos_bits(bs)) as c_int
}
unsafe extern "C" fn init_bits(mut bs: *mut bit_reader_t, mut data: *const c_void, mut data_bytes: c_uint) {
    (*bs).origin = data as *const uint16_t;
    (*bs).origin_bytes = data_bytes;
    set_pos_bits(bs, 0 as c_int as c_uint);
}
unsafe extern "C" fn ue_bits(mut bs: *mut bit_reader_t) -> c_int {
    let mut clz: c_int = 0;
    let mut val: c_int = 0;
    clz = 0 as c_int;
    while get_bits(bs, 1 as c_int) == 0 {
        clz += 1;
    }
    val = ((((1 as c_int) << clz) - 1 as c_int) as c_uint).wrapping_add(if clz != 0 {
        get_bits(bs, clz)
    } else {
        0 as c_int as c_uint
    }) as c_int;
    val
}
unsafe extern "C" fn h264e_bs_put_bits(mut bs: *mut bs_t, mut n: c_uint, mut val: c_uint) {
    if val >> n == 0 {
    } else {
        __assert_fail(
            b"!(val >> n)\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1885 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 59], &[c_char; 59]>(
                b"void h264e_bs_put_bits(bs_t *, unsigned int, unsigned int)\0",
            ))
            .as_ptr(),
        );
    }
    if val >> n == 0 {
    } else {
        __assert_fail(
            b"!(val >> n)\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1885 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 59], &[c_char; 59]>(
                b"void h264e_bs_put_bits(bs_t *, unsigned int, unsigned int)\0",
            ))
            .as_ptr(),
        );
    };
    (*bs).shift = ((*bs).shift as c_uint).wrapping_sub(n) as c_int as c_int;
    if n <= 32 as c_int as c_uint {
    } else {
        __assert_fail(
            b"(unsigned)n <= 32\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1887 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 59], &[c_char; 59]>(
                b"void h264e_bs_put_bits(bs_t *, unsigned int, unsigned int)\0",
            ))
            .as_ptr(),
        );
    }
    if n <= 32 as c_int as c_uint {
    } else {
        __assert_fail(
            b"(unsigned)n <= 32\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1887 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 59], &[c_char; 59]>(
                b"void h264e_bs_put_bits(bs_t *, unsigned int, unsigned int)\0",
            ))
            .as_ptr(),
        );
    };
    if (*bs).shift < 0 as c_int {
        if -(*bs).shift < 32 as c_int {
        } else {
            __assert_fail(
                b"-bs->shift < 32\0" as *const u8 as *const c_char,
                b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                1890 as c_int as c_uint,
                (*::core::mem::transmute::<&[u8; 59], &[c_char; 59]>(
                    b"void h264e_bs_put_bits(bs_t *, unsigned int, unsigned int)\0",
                ))
                .as_ptr(),
            );
        }
        if -(*bs).shift < 32 as c_int {
        } else {
            __assert_fail(
                b"-bs->shift < 32\0" as *const u8 as *const c_char,
                b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                1890 as c_int as c_uint,
                (*::core::mem::transmute::<&[u8; 59], &[c_char; 59]>(
                    b"void h264e_bs_put_bits(bs_t *, unsigned int, unsigned int)\0",
                ))
                .as_ptr(),
            );
        };
        (*bs).cache |= val >> -(*bs).shift;
        let fresh946 = (*bs).buf;
        (*bs).buf = ((*bs).buf).offset(1);
        fresh946.write_unaligned(
            (*bs).cache >> 24 as c_int & 0xff as c_int as c_uint
                | (*bs).cache >> 8 as c_int & 0xff00 as c_int as c_uint
                | (*bs).cache << 8 as c_int & 0xff0000 as c_int as c_uint
                | ((*bs).cache & 0xff as c_int as c_uint) << 24 as c_int,
        );
        (*bs).shift += 32 as c_int;
        (*bs).cache = 0 as c_int as uint32_t;
    }
    (*bs).cache |= val << (*bs).shift;
}
unsafe extern "C" fn h264e_bs_flush(mut bs: *mut bs_t) {
    (*bs).buf.write_unaligned(
        (*bs).cache >> 24 as c_int & 0xff as c_int as c_uint
            | (*bs).cache >> 8 as c_int & 0xff00 as c_int as c_uint
            | (*bs).cache << 8 as c_int & 0xff0000 as c_int as c_uint
            | ((*bs).cache & 0xff as c_int as c_uint) << 24 as c_int,
    );
}
unsafe extern "C" fn h264e_bs_get_pos_bits(mut bs: *const bs_t) -> c_uint {
    let mut pos_bits: c_uint = (((*bs).buf).offset_from((*bs).origin) as c_long * 32 as c_int as c_long) as c_uint;
    pos_bits = pos_bits.wrapping_add((32 as c_int - (*bs).shift) as c_uint);
    if pos_bits as c_int >= 0 as c_int {
    } else {
        __assert_fail(
            b"(int)pos_bits >= 0\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1908 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 49], &[c_char; 49]>(
                b"unsigned int h264e_bs_get_pos_bits(const bs_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if pos_bits as c_int >= 0 as c_int {
    } else {
        __assert_fail(
            b"(int)pos_bits >= 0\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            1908 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 49], &[c_char; 49]>(
                b"unsigned int h264e_bs_get_pos_bits(const bs_t *)\0",
            ))
            .as_ptr(),
        );
    };
    pos_bits
}
unsafe extern "C" fn h264e_bs_byte_align(mut bs: *mut bs_t) -> c_uint {
    let mut pos: c_int = h264e_bs_get_pos_bits(bs) as c_int;
    h264e_bs_put_bits(bs, (-pos & 7 as c_int) as c_uint, 0 as c_int as c_uint);
    (pos + (-pos & 7 as c_int)) as c_uint
}
unsafe extern "C" fn h264e_bs_put_golomb(mut bs: *mut bs_t, mut val: c_uint) {
    let mut size: c_int = 0 as c_int;
    let mut t: c_uint = val.wrapping_add(1 as c_int as c_uint);
    loop {
        size += 1;
        t >>= 1 as c_int;
        if t == 0 {
            break;
        }
    }
    h264e_bs_put_bits(
        bs,
        (2 as c_int * size - 1 as c_int) as c_uint,
        val.wrapping_add(1 as c_int as c_uint),
    );
}
unsafe extern "C" fn h264e_bs_init_bits(mut bs: *mut bs_t, mut data: *mut c_void) {
    (*bs).origin = data as *mut bs_item_t;
    (*bs).buf = (*bs).origin;
    (*bs).shift = 32 as c_int;
    (*bs).cache = 0 as c_int as uint32_t;
}
unsafe extern "C" fn find_mem_cache(
    mut cache: *mut *mut c_void,
    mut cache_bytes: *mut c_int,
    mut cache_size: c_int,
    mut mem: *mut c_void,
    mut bytes: c_int,
) -> c_int {
    let mut i: c_int = 0;
    if bytes == 0 {
        return -(1 as c_int);
    }
    i = 0 as c_int;
    while i < cache_size {
        if *cache_bytes.offset(i as isize) == bytes && memcmp(mem, *cache.offset(i as isize), bytes as c_ulong) == 0 {
            return i;
        }
        i += 1;
    }
    i = 0 as c_int;
    while i < cache_size {
        if *cache_bytes.offset(i as isize) == 0 {
            let fresh947 = &mut (*cache.offset(i as isize));
            *fresh947 = minimp4_malloc(bytes as size_t);
            if !(*cache.offset(i as isize)).is_null() {
                minimp4_memcpy(*cache.offset(i as isize), mem, bytes as size_t);
                *cache_bytes.offset(i as isize) = bytes;
            }
            return i;
        }
        i += 1;
    }
    -(1 as c_int)
}
unsafe extern "C" fn remove_nal_escapes(
    mut dst: *mut c_uchar,
    mut src: *const c_uchar,
    mut h264_data_bytes: c_int,
) -> c_int {
    let mut i: c_int = 0 as c_int;
    let mut j: c_int = 0 as c_int;
    let mut zero_cnt: c_int = 0 as c_int;
    j = 0 as c_int;
    while j < h264_data_bytes {
        if zero_cnt == 2 as c_int && *src.offset(j as isize) as c_int <= 3 as c_int {
            if *src.offset(j as isize) as c_int == 3 as c_int {
                if j != h264_data_bytes - 1 as c_int && *src.offset((j + 1 as c_int) as isize) as c_int <= 3 as c_int {
                    j += 1;
                    zero_cnt = 0 as c_int;
                }
            } else {
                return 0 as c_int;
            }
        }
        let fresh948 = i;
        i += 1;
        *dst.offset(fresh948 as isize) = *src.offset(j as isize);
        if *src.offset(j as isize) != 0 {
            zero_cnt = 0 as c_int;
        } else {
            zero_cnt += 1;
        }
        j += 1;
    }
    i
}
unsafe extern "C" fn nal_put_esc(mut d: *mut uint8_t, mut s: *const uint8_t, mut n: c_int) -> c_int {
    let mut i: c_int = 0;
    let mut j: c_int = 4 as c_int;
    let mut cntz: c_int = 0 as c_int;
    let fresh949 = &mut (*d.offset(2 as c_int as isize));
    *fresh949 = 0 as c_int as uint8_t;
    let fresh950 = &mut (*d.offset(1 as c_int as isize));
    *fresh950 = *fresh949;
    *d.offset(0 as c_int as isize) = *fresh950;
    *d.offset(3 as c_int as isize) = 1 as c_int as uint8_t;
    i = 0 as c_int;
    while i < n {
        let fresh951 = s;
        s = s.offset(1);
        let mut byte: uint8_t = *fresh951;
        if cntz == 2 as c_int && byte as c_int <= 3 as c_int {
            let fresh952 = j;
            j += 1;
            *d.offset(fresh952 as isize) = 3 as c_int as uint8_t;
            cntz = 0 as c_int;
        }
        if byte != 0 {
            cntz = 0 as c_int;
        } else {
            cntz += 1;
        }
        let fresh953 = j;
        j += 1;
        *d.offset(fresh953 as isize) = byte;
        i += 1;
    }
    j
}
unsafe extern "C" fn copy_bits(mut bs: *mut bit_reader_t, mut bd: *mut bs_t) {
    let mut cb: c_uint = 0;
    let mut bits: c_uint = 0;
    let mut bit_count: c_int = remaining_bits(bs);
    while bit_count > 7 as c_int {
        cb = (if (bit_count - 7 as c_int) < 8 as c_int {
            bit_count - 7 as c_int
        } else {
            8 as c_int
        }) as c_uint;
        bits = get_bits(bs, cb as c_int);
        h264e_bs_put_bits(bd, cb, bits);
        bit_count = (bit_count as c_uint).wrapping_sub(cb) as c_int as c_int;
    }
    bits = get_bits(bs, bit_count);
    while bit_count != 0 && !bits & 1 as c_int as c_uint != 0 {
        bits >>= 1 as c_int;
        bit_count -= 1;
    }
    if bit_count != 0 {
        h264e_bs_put_bits(bd, bit_count as c_uint, bits);
    }
}
unsafe extern "C" fn change_sps_id(
    mut bs: *mut bit_reader_t,
    mut bd: *mut bs_t,
    mut new_id: c_int,
    mut old_id: *mut c_int,
) -> c_int {
    let mut bits: c_uint = 0;
    let mut sps_id: c_uint = 0;
    let mut i: c_uint = 0;
    let mut bytes: c_uint = 0;
    i = 0 as c_int as c_uint;
    while i < 3 as c_int as c_uint {
        bits = get_bits(bs, 8 as c_int);
        h264e_bs_put_bits(bd, 8 as c_int as c_uint, bits);
        i = i.wrapping_add(1);
    }
    sps_id = ue_bits(bs) as c_uint;
    *old_id = sps_id as c_int;
    sps_id = new_id as c_uint;
    if sps_id <= 31 as c_int as c_uint {
    } else {
        __assert_fail(
            b"sps_id <= 31\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            2076 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 54], &[c_char; 54]>(
                b"int change_sps_id(bit_reader_t *, bs_t *, int, int *)\0",
            ))
            .as_ptr(),
        );
    }
    if sps_id <= 31 as c_int as c_uint {
    } else {
        __assert_fail(
            b"sps_id <= 31\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            2076 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 54], &[c_char; 54]>(
                b"int change_sps_id(bit_reader_t *, bs_t *, int, int *)\0",
            ))
            .as_ptr(),
        );
    };
    h264e_bs_put_golomb(bd, sps_id);
    copy_bits(bs, bd);
    bytes = (h264e_bs_byte_align(bd)).wrapping_div(8 as c_int as c_uint);
    h264e_bs_flush(bd);
    bytes as c_int
}
unsafe extern "C" fn patch_pps(
    mut h: *mut h264_sps_id_patcher_t,
    mut bs: *mut bit_reader_t,
    mut bd: *mut bs_t,
    mut new_pps_id: c_int,
    mut old_id: *mut c_int,
) -> c_int {
    let mut bytes: c_int = 0;
    let mut pps_id: c_uint = ue_bits(bs) as c_uint;
    let mut sps_id: c_uint = ue_bits(bs) as c_uint;
    *old_id = pps_id as c_int;
    sps_id = (*h).map_sps[sps_id as usize] as c_uint;
    pps_id = new_pps_id as c_uint;
    if sps_id <= 31 as c_int as c_uint {
    } else {
        __assert_fail(
            b"sps_id <= 31\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            2096 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 75], &[c_char; 75]>(
                b"int patch_pps(h264_sps_id_patcher_t *, bit_reader_t *, bs_t *, int, int *)\0",
            ))
            .as_ptr(),
        );
    }
    if sps_id <= 31 as c_int as c_uint {
    } else {
        __assert_fail(
            b"sps_id <= 31\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            2096 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 75], &[c_char; 75]>(
                b"int patch_pps(h264_sps_id_patcher_t *, bit_reader_t *, bs_t *, int, int *)\0",
            ))
            .as_ptr(),
        );
    };
    if pps_id <= 255 as c_int as c_uint {
    } else {
        __assert_fail(
            b"pps_id <= 255\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            2097 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 75], &[c_char; 75]>(
                b"int patch_pps(h264_sps_id_patcher_t *, bit_reader_t *, bs_t *, int, int *)\0",
            ))
            .as_ptr(),
        );
    }
    if pps_id <= 255 as c_int as c_uint {
    } else {
        __assert_fail(
            b"pps_id <= 255\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            2097 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 75], &[c_char; 75]>(
                b"int patch_pps(h264_sps_id_patcher_t *, bit_reader_t *, bs_t *, int, int *)\0",
            ))
            .as_ptr(),
        );
    };
    h264e_bs_put_golomb(bd, pps_id);
    h264e_bs_put_golomb(bd, sps_id);
    copy_bits(bs, bd);
    bytes = (h264e_bs_byte_align(bd)).wrapping_div(8 as c_int as c_uint) as c_int;
    h264e_bs_flush(bd);
    bytes
}
unsafe extern "C" fn patch_slice_header(
    mut h: *mut h264_sps_id_patcher_t,
    mut bs: *mut bit_reader_t,
    mut bd: *mut bs_t,
) {
    let mut first_mb_in_slice: c_uint = ue_bits(bs) as c_uint;
    let mut slice_type: c_uint = ue_bits(bs) as c_uint;
    let mut pps_id: c_uint = ue_bits(bs) as c_uint;
    pps_id = (*h).map_pps[pps_id as usize] as c_uint;
    if pps_id <= 255 as c_int as c_uint {
    } else {
        __assert_fail(
            b"pps_id <= 255\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            2116 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 73], &[c_char; 73]>(
                b"void patch_slice_header(h264_sps_id_patcher_t *, bit_reader_t *, bs_t *)\0",
            ))
            .as_ptr(),
        );
    }
    if pps_id <= 255 as c_int as c_uint {
    } else {
        __assert_fail(
            b"pps_id <= 255\0" as *const u8 as *const c_char,
            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
            2116 as c_int as c_uint,
            (*::core::mem::transmute::<&[u8; 73], &[c_char; 73]>(
                b"void patch_slice_header(h264_sps_id_patcher_t *, bit_reader_t *, bs_t *)\0",
            ))
            .as_ptr(),
        );
    };
    h264e_bs_put_golomb(bd, first_mb_in_slice);
    h264e_bs_put_golomb(bd, slice_type);
    h264e_bs_put_golomb(bd, pps_id);
    copy_bits(bs, bd);
}
unsafe extern "C" fn transcode_nalu(
    mut h: *mut h264_sps_id_patcher_t,
    mut src: *const c_uchar,
    mut nalu_bytes: c_int,
    mut dst: *mut c_uchar,
) -> c_int {
    let mut old_id: c_int = 0;
    let mut bst: [bit_reader_t; 1] = [bit_reader_t {
        cache: 0,
        cache_free_bits: 0,
        buf: std::ptr::null::<uint16_t>(),
        origin: std::ptr::null::<uint16_t>(),
        origin_bytes: 0,
    }; 1];
    let mut bdt: [bs_t; 1] = [bs_t {
        shift: 0,
        cache: 0,
        buf: std::ptr::null_mut::<bs_item_t>(),
        origin: std::ptr::null_mut::<bs_item_t>(),
    }; 1];
    let mut bs: [bit_reader_t; 1] = [bit_reader_t {
        cache: 0,
        cache_free_bits: 0,
        buf: std::ptr::null::<uint16_t>(),
        origin: std::ptr::null::<uint16_t>(),
        origin_bytes: 0,
    }; 1];
    let mut bd: [bs_t; 1] = [bs_t {
        shift: 0,
        cache: 0,
        buf: std::ptr::null_mut::<bs_item_t>(),
        origin: std::ptr::null_mut::<bs_item_t>(),
    }; 1];
    let mut payload_type: c_int = *src.offset(0 as c_int as isize) as c_int & 31 as c_int;
    *dst = *src;
    h264e_bs_init_bits(bd.as_mut_ptr(), dst.offset(1 as c_int as isize) as *mut c_void);
    init_bits(
        bs.as_mut_ptr(),
        src.offset(1 as c_int as isize) as *const c_void,
        (nalu_bytes - 1 as c_int) as c_uint,
    );
    h264e_bs_init_bits(bdt.as_mut_ptr(), dst.offset(1 as c_int as isize) as *mut c_void);
    init_bits(
        bst.as_mut_ptr(),
        src.offset(1 as c_int as isize) as *const c_void,
        (nalu_bytes - 1 as c_int) as c_uint,
    );
    match payload_type {
        7 => {
            let mut cb: c_int = change_sps_id(bst.as_mut_ptr(), bdt.as_mut_ptr(), 0 as c_int, &mut old_id);
            let mut id: c_int = find_mem_cache(
                ((*h).sps_cache).as_mut_ptr(),
                ((*h).sps_bytes).as_mut_ptr(),
                32 as c_int,
                dst.offset(1 as c_int as isize) as *mut c_void,
                cb,
            );
            if id == -(1 as c_int) {
                return 0 as c_int;
            }
            (*h).map_sps[old_id as usize] = id;
            change_sps_id(bs.as_mut_ptr(), bd.as_mut_ptr(), id, &mut old_id);
        }
        8 => {
            let mut cb_0: c_int = patch_pps(h, bst.as_mut_ptr(), bdt.as_mut_ptr(), 0 as c_int, &mut old_id);
            let mut id_0: c_int = find_mem_cache(
                ((*h).pps_cache).as_mut_ptr(),
                ((*h).pps_bytes).as_mut_ptr(),
                256 as c_int,
                dst.offset(1 as c_int as isize) as *mut c_void,
                cb_0,
            );
            if id_0 == -(1 as c_int) {
                return 0 as c_int;
            }
            (*h).map_pps[old_id as usize] = id_0;
            patch_pps(h, bs.as_mut_ptr(), bd.as_mut_ptr(), id_0, &mut old_id);
        }
        1 | 2 | 5 => {
            patch_slice_header(h, bs.as_mut_ptr(), bd.as_mut_ptr());
        }
        _ => {
            minimp4_memcpy(dst as *mut c_void, src as *const c_void, nalu_bytes as size_t);
            return nalu_bytes;
        }
    }
    nalu_bytes = (1 as c_int as c_uint)
        .wrapping_add((h264e_bs_byte_align(bd.as_mut_ptr())).wrapping_div(8 as c_int as c_uint))
        as c_int;
    h264e_bs_flush(bd.as_mut_ptr());
    nalu_bytes
}
unsafe extern "C" fn find_start_code(
    mut h264_data: *const uint8_t,
    mut h264_data_bytes: c_int,
    mut zcount: *mut c_int,
) -> *const uint8_t {
    let mut eof: *const uint8_t = h264_data.offset(h264_data_bytes as isize);
    let mut p: *const uint8_t = h264_data;
    loop {
        let mut zero_cnt: c_int = 1 as c_int;
        let mut found: *const uint8_t =
            memchr(p as *const c_void, 0 as c_int, eof.offset_from(p) as c_long as c_ulong) as *mut uint8_t;
        p = if !found.is_null() { found } else { eof };
        while p.offset(zero_cnt as isize) < eof && *p.offset(zero_cnt as isize) == 0 {
            zero_cnt += 1;
        }
        if zero_cnt >= 2 as c_int && *p.offset(zero_cnt as isize) as c_int == 1 as c_int {
            *zcount = zero_cnt + 1 as c_int;
            return p.offset(zero_cnt as isize).offset(1 as c_int as isize);
        }
        p = p.offset(zero_cnt as isize);
        if p >= eof {
            break;
        }
    }
    *zcount = 0 as c_int;
    eof
}
unsafe extern "C" fn find_nal_unit(
    mut h264_data: *const uint8_t,
    mut h264_data_bytes: c_int,
    mut pnal_unit_bytes: *mut c_int,
) -> *const uint8_t {
    let mut eof: *const uint8_t = h264_data.offset(h264_data_bytes as isize);
    let mut zcount: c_int = 0;
    let mut start: *const uint8_t = find_start_code(h264_data, h264_data_bytes, &mut zcount);
    let mut stop: *const uint8_t = start;
    if !start.is_null() {
        stop = find_start_code(start, eof.offset_from(start) as c_long as c_int, &mut zcount);
        while stop > start && *stop.offset(-(1 as c_int) as isize) == 0 {
            stop = stop.offset(-1);
        }
    }
    *pnal_unit_bytes = (stop.offset_from(start) as c_long - zcount as c_long) as c_int;
    start
}
#[no_mangle]
pub unsafe extern "C" fn mp4_h26x_write_init(
    mut h: *mut mp4_h26x_writer_t,
    mut mux: *mut MP4E_mux_t,
    mut width: c_int,
    mut height: c_int,
    mut is_hevc: c_int,
) -> c_int {
    let mut tr: MP4E_track_t = MP4E_track_t {
        object_type_indication: 0,
        language: [0; 4],
        track_media_kind: e_audio,
        time_scale: 0,
        default_duration: 0,
        u: C2RustUnnamed {
            a: C2RustUnnamed_1 { channelcount: 0 },
        },
    };
    tr.track_media_kind = e_video;
    tr.language[0 as c_int as usize] = 'u' as i32 as c_uchar;
    tr.language[1 as c_int as usize] = 'n' as i32 as c_uchar;
    tr.language[2 as c_int as usize] = 'd' as i32 as c_uchar;
    tr.language[3 as c_int as usize] = 0 as c_int as c_uchar;
    tr.object_type_indication = (if is_hevc != 0 { 0x23 as c_int } else { 0x21 as c_int }) as c_uint;
    tr.time_scale = 90000 as c_int as c_uint;
    tr.default_duration = 0 as c_int as c_uint;
    tr.u.v.width = width;
    tr.u.v.height = height;
    (*h).mux_track_id = MP4E_add_track(mux, &mut tr);
    (*h).mux = mux;
    (*h).is_hevc = is_hevc;
    (*h).need_vps = is_hevc;
    (*h).need_sps = 1 as c_int;
    (*h).need_pps = 1 as c_int;
    (*h).need_idr = 1 as c_int;
    minimp4_memset(
        &mut (*h).sps_patcher as *mut h264_sps_id_patcher_t as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<h264_sps_id_patcher_t>() as c_ulong,
    );
    0 as c_int
}
#[no_mangle]
pub unsafe extern "C" fn mp4_h26x_write_close(mut h: *mut mp4_h26x_writer_t) {
    let mut p: *mut h264_sps_id_patcher_t = &mut (*h).sps_patcher;
    let mut i: c_int = 0;
    i = 0 as c_int;
    while i < 32 as c_int {
        if !((*p).sps_cache[i as usize]).is_null() {
            minimp4_free((*p).sps_cache[i as usize]);
        }
        i += 1;
    }
    i = 0 as c_int;
    while i < 256 as c_int {
        if !((*p).pps_cache[i as usize]).is_null() {
            minimp4_free((*p).pps_cache[i as usize]);
        }
        i += 1;
    }
    minimp4_memset(
        h as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<mp4_h26x_writer_t>() as c_ulong,
    );
}
unsafe extern "C" fn mp4_h265_write_nal(
    mut h: *mut mp4_h26x_writer_t,
    mut nal: *const c_uchar,
    mut sizeof_nal: c_int,
    mut timeStamp90kHz_next: c_uint,
) -> c_int {
    let mut payload_type: c_int = *nal.offset(0 as c_int as isize) as c_int >> 1 as c_int & 0x3f as c_int;
    let mut is_intra: c_int = (payload_type >= 16 as c_int && payload_type <= 21 as c_int) as c_int;
    let mut err: c_int = 0 as c_int;
    if is_intra != 0 && (*h).need_sps == 0 && (*h).need_pps == 0 && (*h).need_vps == 0 {
        (*h).need_idr = 0 as c_int;
    }
    match payload_type {
        32 => {
            MP4E_set_vps((*h).mux, (*h).mux_track_id, nal as *const c_void, sizeof_nal);
            (*h).need_vps = 0 as c_int;
        }
        33 => {
            MP4E_set_sps((*h).mux, (*h).mux_track_id, nal as *const c_void, sizeof_nal);
            (*h).need_sps = 0 as c_int;
        }
        34 => {
            MP4E_set_pps((*h).mux, (*h).mux_track_id, nal as *const c_void, sizeof_nal);
            (*h).need_pps = 0 as c_int;
        }
        _ => {
            if (*h).need_vps != 0 || (*h).need_sps != 0 || (*h).need_pps != 0 || (*h).need_idr != 0 {
                return -(1 as c_int);
            }
            let mut tmp: *mut c_uchar = minimp4_malloc((4 as c_int + sizeof_nal) as size_t) as *mut c_uchar;
            if tmp.is_null() {
                return -(2 as c_int);
            }
            let mut sample_kind: c_int = 0 as c_int;
            *tmp.offset(0 as c_int as isize) = (sizeof_nal >> 24 as c_int) as c_uchar;
            *tmp.offset(1 as c_int as isize) = (sizeof_nal >> 16 as c_int) as c_uchar;
            *tmp.offset(2 as c_int as isize) = (sizeof_nal >> 8 as c_int) as c_uchar;
            *tmp.offset(3 as c_int as isize) = sizeof_nal as c_uchar;
            minimp4_memcpy(
                tmp.offset(4 as c_int as isize) as *mut c_void,
                nal as *const c_void,
                sizeof_nal as size_t,
            );
            if is_intra != 0 {
                sample_kind = 1 as c_int;
            }
            err = MP4E_put_sample(
                (*h).mux,
                (*h).mux_track_id,
                tmp as *const c_void,
                4 as c_int + sizeof_nal,
                timeStamp90kHz_next as c_int,
                sample_kind,
            );
            minimp4_free(tmp as *mut c_void);
        }
    }
    err
}
#[no_mangle]
pub unsafe extern "C" fn mp4_h26x_write_nal(
    mut h: *mut mp4_h26x_writer_t,
    mut nal: *const c_uchar,
    mut length: c_int,
    mut timeStamp90kHz_next: c_uint,
) -> c_int {
    let mut current_block: u64;
    let mut eof: *const c_uchar = nal.offset(length as isize);
    let mut payload_type: c_int = 0;
    let mut sizeof_nal: c_int = 0;
    let mut err: c_int = 0 as c_int;
    loop {
        let mut nal1: *mut c_uchar = std::ptr::null_mut::<c_uchar>();
        let mut nal2: *mut c_uchar = std::ptr::null_mut::<c_uchar>();
        nal = find_nal_unit(nal, eof.offset_from(nal) as c_long as c_int, &mut sizeof_nal);
        if sizeof_nal == 0 {
            break;
        }
        if (*h).is_hevc != 0 {
            let mut err_0: c_int = mp4_h265_write_nal(h, nal, sizeof_nal, timeStamp90kHz_next);
            if err_0 != 0 {
                return err_0;
            }
        } else {
            payload_type = *nal.offset(0 as c_int as isize) as c_int & 31 as c_int;
            if 9 as c_int != payload_type {
                nal1 = minimp4_malloc((sizeof_nal * 17 as c_int / 16 as c_int + 32 as c_int) as size_t) as *mut c_uchar;
                if nal1.is_null() {
                    return -(2 as c_int);
                }
                nal2 = minimp4_malloc((sizeof_nal * 17 as c_int / 16 as c_int + 32 as c_int) as size_t) as *mut c_uchar;
                if nal2.is_null() {
                    minimp4_free(nal1 as *mut c_void);
                    return -(2 as c_int);
                }
                sizeof_nal = remove_nal_escapes(nal2, nal, sizeof_nal);
                if sizeof_nal == 0 {
                    current_block = 2753430684906932723;
                } else {
                    sizeof_nal = transcode_nalu(&mut (*h).sps_patcher, nal2, sizeof_nal, nal1);
                    sizeof_nal = nal_put_esc(nal2, nal1, sizeof_nal);
                    match payload_type {
                        7 => {
                            MP4E_set_sps(
                                (*h).mux,
                                (*h).mux_track_id,
                                nal2.offset(4 as c_int as isize) as *const c_void,
                                sizeof_nal - 4 as c_int,
                            );
                            (*h).need_sps = 0 as c_int;
                            current_block = 15090052786889560393;
                        }
                        8 => {
                            if (*h).need_sps != 0 {
                                current_block = 2753430684906932723;
                            } else {
                                MP4E_set_pps(
                                    (*h).mux,
                                    (*h).mux_track_id,
                                    nal2.offset(4 as c_int as isize) as *const c_void,
                                    sizeof_nal - 4 as c_int,
                                );
                                (*h).need_pps = 0 as c_int;
                                current_block = 15090052786889560393;
                            }
                        }
                        5 => {
                            if (*h).need_sps != 0 {
                                current_block = 2753430684906932723;
                            } else {
                                (*h).need_idr = 0 as c_int;
                                current_block = 14185338118334136158;
                            }
                        }
                        _ => {
                            current_block = 14185338118334136158;
                        }
                    }
                    match current_block {
                        2753430684906932723 => {}
                        _ => {
                            match current_block {
                                14185338118334136158 => {
                                    if (*h).need_sps != 0 {
                                        current_block = 2753430684906932723;
                                    } else {
                                        if (*h).need_pps == 0 && (*h).need_idr == 0 {
                                            let mut bs: [bit_reader_t; 1] = [bit_reader_t {
                                                cache: 0,
                                                cache_free_bits: 0,
                                                buf: std::ptr::null::<uint16_t>(),
                                                origin: std::ptr::null::<uint16_t>(),
                                                origin_bytes: 0,
                                            };
                                                1];
                                            init_bits(
                                                bs.as_mut_ptr(),
                                                nal.offset(1 as c_int as isize) as *const c_void,
                                                (sizeof_nal - 4 as c_int - 1 as c_int) as c_uint,
                                            );
                                            let mut first_mb_in_slice: c_uint = ue_bits(bs.as_mut_ptr()) as c_uint;
                                            let mut sample_kind: c_int = 0 as c_int;
                                            *nal2.offset(0 as c_int as isize) =
                                                ((sizeof_nal - 4 as c_int) >> 24 as c_int) as c_uchar;
                                            *nal2.offset(1 as c_int as isize) =
                                                ((sizeof_nal - 4 as c_int) >> 16 as c_int) as c_uchar;
                                            *nal2.offset(2 as c_int as isize) =
                                                ((sizeof_nal - 4 as c_int) >> 8 as c_int) as c_uchar;
                                            *nal2.offset(3 as c_int as isize) = (sizeof_nal - 4 as c_int) as c_uchar;
                                            if first_mb_in_slice != 0 {
                                                sample_kind = 2 as c_int;
                                            } else if payload_type == 5 as c_int {
                                                sample_kind = 1 as c_int;
                                            }
                                            err = MP4E_put_sample(
                                                (*h).mux,
                                                (*h).mux_track_id,
                                                nal2 as *const c_void,
                                                sizeof_nal,
                                                timeStamp90kHz_next as c_int,
                                                sample_kind,
                                            );
                                        }
                                        current_block = 15090052786889560393;
                                    }
                                }
                                _ => {}
                            }
                            match current_block {
                                2753430684906932723 => {}
                                _ => {
                                    minimp4_free(nal1 as *mut c_void);
                                    minimp4_free(nal2 as *mut c_void);
                                    if err != 0 {
                                        break;
                                    }
                                    current_block = 16668937799742929182;
                                }
                            }
                        }
                    }
                }
                match current_block {
                    16668937799742929182 => {}
                    _ => {
                        minimp4_free(nal1 as *mut c_void);
                        minimp4_free(nal2 as *mut c_void);
                        return -(1 as c_int);
                    }
                }
            }
        }
        nal = nal.offset(1);
    }
    err
}
unsafe extern "C" fn minimp4_fgets(mut mp4: *mut MP4D_demux_t) -> c_int {
    let mut c: uint8_t = 0;
    if ((*mp4).read_callback).expect("non-null function pointer")(
        (*mp4).read_pos,
        &mut c as *mut uint8_t as *mut c_void,
        1 as c_int as size_t,
        (*mp4).token,
    ) != 0
    {
        return -(1 as c_int);
    }
    (*mp4).read_pos += 1;
    c as c_int
}
unsafe extern "C" fn minimp4_read(mut mp4: *mut MP4D_demux_t, mut nb: c_int, mut eof_flag: *mut c_int) -> c_uint {
    let mut v: uint32_t = 0 as c_int as uint32_t;
    let mut last_byte: c_int = 0;
    let mut current_block_3: u64;
    match nb {
        4 => {
            v = v << 8 as c_int | minimp4_fgets(mp4) as c_uint;
            current_block_3 = 12448927583017242807;
        }
        3 => {
            current_block_3 = 12448927583017242807;
        }
        2 => {
            current_block_3 = 11245112099699409293;
        }
        1 | _ => {
            current_block_3 = 2631889515033231950;
        }
    }
    match current_block_3 {
        12448927583017242807 => {
            v = v << 8 as c_int | minimp4_fgets(mp4) as c_uint;
            current_block_3 = 11245112099699409293;
        }
        _ => {}
    }
    match current_block_3 {
        11245112099699409293 => {
            v = v << 8 as c_int | minimp4_fgets(mp4) as c_uint;
        }
        _ => {}
    }
    last_byte = minimp4_fgets(mp4);
    v = v << 8 as c_int | last_byte as c_uint;
    if last_byte < 0 as c_int {
        *eof_flag = 1 as c_int;
    }
    v
}
unsafe extern "C" fn read_payload(
    mut mp4: *mut MP4D_demux_t,
    mut nb: c_uint,
    mut payload_bytes: *mut boxsize_t,
    mut eof_flag: *mut c_int,
) -> uint32_t {
    if *payload_bytes < nb as c_ulong {
        *eof_flag = 1 as c_int;
        nb = *payload_bytes as c_int as c_uint;
    }
    *payload_bytes = (*payload_bytes as c_ulong).wrapping_sub(nb as c_ulong) as boxsize_t as boxsize_t;
    minimp4_read(mp4, nb as c_int, eof_flag)
}
unsafe extern "C" fn my_fseek(mut mp4: *mut MP4D_demux_t, mut pos: boxsize_t, mut eof_flag: *mut c_int) {
    (*mp4).read_pos = ((*mp4).read_pos as c_ulong).wrapping_add(pos) as int64_t as int64_t;
    if (*mp4).read_pos >= (*mp4).read_size {
        *eof_flag = 1 as c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn MP4D_open(
    mut mp4: *mut MP4D_demux_t,
    mut read_callback: Option<unsafe extern "C" fn(int64_t, *mut c_void, size_t, *mut c_void) -> c_int>,
    mut token: *mut c_void,
    mut file_size: int64_t,
) -> c_int {
    let mut depth: c_int = 0 as c_int;
    let mut stack: [C2RustUnnamed_7; 64] = [C2RustUnnamed_7 {
        bytes: 0,
        format: BOX_ATOM,
    }; 64];
    let mut eof_flag: c_int = 0 as c_int;
    let mut i: c_uint = 0;
    let mut tr: *mut MP4D_track_t = std::ptr::null_mut::<MP4D_track_t>();
    if mp4.is_null() || read_callback.is_none() {
        return 0 as c_int;
    }
    minimp4_memset(
        mp4 as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<MP4D_demux_t>() as c_ulong,
    );
    (*mp4).read_callback = read_callback;
    (*mp4).token = token;
    (*mp4).read_size = file_size;
    stack[0 as c_int as usize].format = BOX_ATOM;
    stack[0 as c_int as usize].bytes = 0 as c_int as boxsize_t;
    let mut current_block_407: u64;
    's_45: loop {
        static mut g_fullbox: [C2RustUnnamed_9; 13] = [
            {
                C2RustUnnamed_9 {
                    name: BOX_mdhd as c_int as uint32_t,
                    max_version: 1 as c_int as c_uint,
                    use_track_flag: 1 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_mvhd as c_int as uint32_t,
                    max_version: 1 as c_int as c_uint,
                    use_track_flag: 0 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_hdlr as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 0 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_meta as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 0 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_stts as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 0 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_ctts as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 0 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_stz2 as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 1 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_stsz as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 1 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_stsc as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 1 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_stco as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 1 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_co64 as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 1 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_stsd as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 0 as c_int as c_uint,
                }
            },
            {
                C2RustUnnamed_9 {
                    name: BOX_esds as c_int as uint32_t,
                    max_version: 0 as c_int as c_uint,
                    use_track_flag: 1 as c_int as c_uint,
                }
            },
        ];
        static mut g_envelope_box: [C2RustUnnamed_8; 20] = [
            {
                C2RustUnnamed_8 {
                    name: BOX_esds as c_int as uint32_t,
                    type_0: BOX_OD,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: OD_ESD as c_int as uint32_t,
                    type_0: BOX_OD,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: OD_DCD as c_int as uint32_t,
                    type_0: BOX_OD,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: OD_DSI as c_int as uint32_t,
                    type_0: BOX_OD,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_trak as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_moov as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_mdia as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_tref as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_minf as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_dinf as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_stbl as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_stsd as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_mp4a as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_mp4s as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_mp4v as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_avc1 as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_hvc1 as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_udta as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_meta as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
            {
                C2RustUnnamed_8 {
                    name: BOX_ilst as c_int as uint32_t,
                    type_0: BOX_ATOM,
                }
            },
        ];
        let mut FullAtomVersionAndFlags: uint32_t = 0 as c_int as uint32_t;
        let mut payload_bytes: boxsize_t = 0;
        let mut box_bytes: boxsize_t = 0;
        let mut box_name: uint32_t = 0;
        let mut ptag: *mut *mut c_uchar = std::ptr::null_mut::<*mut c_uchar>();
        let mut read_bytes: c_int = 0 as c_int;
        if stack[depth as usize].format as c_uint == BOX_ATOM as c_int as c_uint {
            box_bytes = minimp4_read(mp4, 4 as c_int, &mut eof_flag) as boxsize_t;
            '_broken_android_meta_hack: loop {
                if eof_flag != 0 {
                    break 's_45;
                }
                if box_bytes >= 2 as c_int as c_ulong && box_bytes < 8 as c_int as c_ulong {
                    if depth == 0 {
                        break 's_45;
                    }
                    MP4D_close(mp4);
                    return 0 as c_int;
                } else {
                    box_name = minimp4_read(mp4, 4 as c_int, &mut eof_flag);
                    read_bytes = 8 as c_int;
                    if box_bytes == 0 as c_int as c_ulong || box_bytes == 0xffffffff as c_uint as boxsize_t {
                        box_bytes = !(0 as c_int as boxsize_t);
                    }
                    payload_bytes = box_bytes.wrapping_sub(8 as c_int as c_ulong);
                    if box_bytes == 1 as c_int as c_ulong {
                        box_bytes = minimp4_read(mp4, 4 as c_int, &mut eof_flag) as boxsize_t;
                        if box_bytes != 0 {
                            if depth == 0 {
                                break 's_45;
                            }
                            MP4D_close(mp4);
                            return 0 as c_int;
                        } else {
                            box_bytes = minimp4_read(mp4, 4 as c_int, &mut eof_flag) as boxsize_t;
                            if box_bytes < 16 as c_int as c_ulong {
                                if depth == 0 {
                                    break 's_45;
                                }
                                MP4D_close(mp4);
                                return 0 as c_int;
                            } else {
                                payload_bytes = box_bytes.wrapping_sub(16 as c_int as c_ulong);
                            }
                        }
                    }
                    i = 0 as c_int as c_uint;
                    loop {
                        if (i as c_ulong)
                            >= (::core::mem::size_of::<[C2RustUnnamed_9; 13]>() as c_ulong)
                                .wrapping_div(::core::mem::size_of::<C2RustUnnamed_9>() as c_ulong)
                        {
                            current_block_407 = 10393716428851982524;
                            break '_broken_android_meta_hack;
                        }
                        if box_name == g_fullbox[i as usize].name {
                            FullAtomVersionAndFlags =
                                read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                            read_bytes += 4 as c_int;
                            if box_name == BOX_meta as c_int as c_uint
                                && FullAtomVersionAndFlags >= 8 as c_int as c_uint
                                && (FullAtomVersionAndFlags as c_ulong) < payload_bytes
                            {
                                if box_bytes > stack[depth as usize].bytes {
                                    current_block_407 = 2122094917359643297;
                                    break;
                                } else {
                                    current_block_407 = 10095721787123848864;
                                    break;
                                }
                            }
                            if FullAtomVersionAndFlags >> 24 as c_int > g_fullbox[i as usize].max_version {
                                if depth == 0 {
                                    current_block_407 = 10393716428851982524;
                                    break '_broken_android_meta_hack;
                                } else {
                                    current_block_407 = 9859671972921157070;
                                    break '_broken_android_meta_hack;
                                }
                            } else if g_fullbox[i as usize].use_track_flag != 0 && tr.is_null() {
                                if depth == 0 {
                                    current_block_407 = 10393716428851982524;
                                    break '_broken_android_meta_hack;
                                } else {
                                    current_block_407 = 15594839951440953787;
                                    break '_broken_android_meta_hack;
                                }
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                    match current_block_407 {
                        2122094917359643297 => {
                            if depth == 0 {
                                current_block_407 = 10393716428851982524;
                                break;
                            } else {
                                current_block_407 = 3160140712158701372;
                                break;
                            }
                        }
                        _ => {
                            stack[depth as usize].bytes = (stack[depth as usize].bytes as c_ulong)
                                .wrapping_sub(box_bytes)
                                as boxsize_t as boxsize_t;
                            depth += 1;
                            stack[depth as usize].bytes = payload_bytes.wrapping_add(4 as c_int as c_ulong);
                            stack[depth as usize].format = BOX_ATOM;
                            box_bytes = FullAtomVersionAndFlags as boxsize_t;
                        }
                    }
                }
            }
            match current_block_407 {
                10393716428851982524 => {}
                _ => match current_block_407 {
                    15594839951440953787 => {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                    3160140712158701372 => {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                    _ => {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                },
            }
        } else {
            let mut val: c_int = 0;
            box_name = (OD_BASE as c_int as c_uint).wrapping_add(minimp4_read(mp4, 1 as c_int, &mut eof_flag));
            read_bytes += 1 as c_int;
            if eof_flag != 0 {
                break;
            }
            payload_bytes = 0 as c_int as boxsize_t;
            box_bytes = 1 as c_int as boxsize_t;
            loop {
                val = minimp4_read(mp4, 1 as c_int, &mut eof_flag) as c_int;
                read_bytes += 1 as c_int;
                if eof_flag != 0 {
                    if depth == 0 {
                        break;
                    }
                    MP4D_close(mp4);
                    return 0 as c_int;
                } else {
                    payload_bytes = payload_bytes << 7 as c_int | (val & 0x7f as c_int) as c_ulong;
                    box_bytes = box_bytes.wrapping_add(1);
                    if val & 0x80 as c_int == 0 {
                        break;
                    }
                }
            }
            box_bytes = (box_bytes as c_ulong).wrapping_add(payload_bytes) as boxsize_t as boxsize_t;
        }
        if depth != 0 {
            if box_bytes > 0 as c_int as c_ulong {
            } else {
                __assert_fail(
                    b"box_bytes > 0\0" as *const u8 as *const c_char,
                    b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                    2790 as c_int as c_uint,
                    (*::core::mem::transmute::<&[u8; 89], &[c_char; 89]>(
                        b"int MP4D_open(MP4D_demux_t *, int (*)(int64_t, void *, size_t, void *), void *, int64_t)\0",
                    ))
                    .as_ptr(),
                );
            }
            if box_bytes > 0 as c_int as c_ulong {
            } else {
                __assert_fail(
                    b"box_bytes > 0\0" as *const u8 as *const c_char,
                    b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                    2790 as c_int as c_uint,
                    (*::core::mem::transmute::<&[u8; 89], &[c_char; 89]>(
                        b"int MP4D_open(MP4D_demux_t *, int (*)(int64_t, void *, size_t, void *), void *, int64_t)\0",
                    ))
                    .as_ptr(),
                );
            };
            if box_bytes > stack[depth as usize].bytes {
                box_bytes = stack[depth as usize].bytes;
                box_name = 0 as c_int as uint32_t;
                payload_bytes = box_bytes.wrapping_sub(read_bytes as c_ulong);
            }
            stack[depth as usize].bytes =
                (stack[depth as usize].bytes as c_ulong).wrapping_sub(box_bytes) as boxsize_t as boxsize_t;
        }
        match box_name {
            1937013298 | 1937011578 => {
                let mut size: c_int = 0 as c_int;
                let mut sample_size: uint32_t =
                    read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                (*tr).sample_count = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                (*tr).entry_size =
                    minimp4_malloc(((*tr).sample_count).wrapping_mul(4 as c_int as c_uint) as size_t) as *mut c_uint;
                if ((*tr).entry_size).is_null() {
                    if depth != 0 {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                } else {
                    i = 0 as c_int as c_uint;
                    while i < (*tr).sample_count {
                        if box_name == BOX_stsz as c_int as c_uint {
                            *((*tr).entry_size).offset(i as isize) = if sample_size != 0 {
                                sample_size
                            } else {
                                read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag)
                            };
                        } else {
                            match sample_size & 0xff as c_int as c_uint {
                                16 => {
                                    *((*tr).entry_size).offset(i as isize) =
                                        read_payload(mp4, 2 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                                }
                                8 => {
                                    *((*tr).entry_size).offset(i as isize) =
                                        read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                                }
                                4 => {
                                    if i & 1 as c_int as c_uint != 0 {
                                        *((*tr).entry_size).offset(i as isize) = (size & 15 as c_int) as c_uint;
                                    } else {
                                        size =
                                            read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag)
                                                as c_int;
                                        *((*tr).entry_size).offset(i as isize) = (size >> 4 as c_int) as c_uint;
                                    }
                                }
                                _ => {}
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                }
            }
            1937011555 => {
                (*tr).sample_to_chunk_count =
                    read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                (*tr).sample_to_chunk = minimp4_malloc(
                    ((*tr).sample_to_chunk_count as c_ulong)
                        .wrapping_mul(::core::mem::size_of::<MP4D_sample_to_chunk_t_tag>() as c_ulong),
                ) as *mut MP4D_sample_to_chunk_t;
                if ((*tr).sample_to_chunk).is_null() {
                    if depth != 0 {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                } else {
                    i = 0 as c_int as c_uint;
                    while i < (*tr).sample_to_chunk_count {
                        (*((*tr).sample_to_chunk).offset(i as isize)).first_chunk =
                            read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                        (*((*tr).sample_to_chunk).offset(i as isize)).samples_per_chunk =
                            read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                        let mut t: boxsize_t = if payload_bytes < 4 as c_int as c_ulong {
                            payload_bytes
                        } else {
                            4 as c_int as c_ulong
                        };
                        my_fseek(mp4, t, &mut eof_flag);
                        payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t) as boxsize_t as boxsize_t;
                        i = i.wrapping_add(1);
                    }
                }
            }
            1937011827 => {
                let mut count: c_uint = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                let mut j: c_uint = 0;
                let mut k: c_uint = 0 as c_int as c_uint;
                let mut ts: c_uint = 0 as c_int as c_uint;
                let mut ts_count: c_uint = count;
                (*tr).timestamp = minimp4_malloc(ts_count.wrapping_mul(4 as c_int as c_uint) as size_t) as *mut c_uint;
                if ((*tr).timestamp).is_null() {
                    if depth != 0 {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                } else {
                    (*tr).duration =
                        minimp4_malloc(ts_count.wrapping_mul(4 as c_int as c_uint) as size_t) as *mut c_uint;
                    if ((*tr).duration).is_null() {
                        if depth != 0 {
                            MP4D_close(mp4);
                            return 0 as c_int;
                        }
                    } else {
                        i = 0 as c_int as c_uint;
                        while i < count {
                            let mut sc: c_uint =
                                read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                            let mut d: c_int =
                                read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag) as c_int;
                            if k.wrapping_add(sc) > ts_count {
                                ts_count = k.wrapping_add(sc);
                                (*tr).timestamp = minimp4_realloc(
                                    (*tr).timestamp as *mut c_void,
                                    (ts_count as c_ulong).wrapping_mul(::core::mem::size_of::<c_uint>() as c_ulong),
                                ) as *mut c_uint;
                                (*tr).duration = minimp4_realloc(
                                    (*tr).duration as *mut c_void,
                                    (ts_count as c_ulong).wrapping_mul(::core::mem::size_of::<c_uint>() as c_ulong),
                                ) as *mut c_uint;
                            }
                            j = 0 as c_int as c_uint;
                            while j < sc {
                                *((*tr).duration).offset(k as isize) = d as c_uint;
                                let fresh954 = k;
                                k = k.wrapping_add(1);
                                *((*tr).timestamp).offset(fresh954 as isize) = ts;
                                ts = ts.wrapping_add(d as c_uint);
                                j = j.wrapping_add(1);
                            }
                            i = i.wrapping_add(1);
                        }
                    }
                }
            }
            1668576371 => {
                let mut count_0: c_uint = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                i = 0 as c_int as c_uint;
                while i < count_0 {
                    let mut _sc_0: c_int =
                        read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag) as c_int;
                    let mut _d_0: c_int =
                        read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag) as c_int;
                    i = i.wrapping_add(1);
                }
            }
            1937007471 | 1668232756 => {
                (*tr).chunk_count = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                (*tr).chunk_offset = minimp4_malloc(
                    ((*tr).chunk_count as c_ulong)
                        .wrapping_mul(::core::mem::size_of::<MP4D_file_offset_t>() as c_ulong),
                ) as *mut MP4D_file_offset_t;
                if ((*tr).chunk_offset).is_null() {
                    if depth != 0 {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                } else {
                    i = 0 as c_int as c_uint;
                    while i < (*tr).chunk_count {
                        *((*tr).chunk_offset).offset(i as isize) =
                            read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag)
                                as MP4D_file_offset_t;
                        if box_name == BOX_co64 as c_int as c_uint {
                            if *((*tr).chunk_offset).offset(i as isize) != 0 {
                                if depth == 0 {
                                    break;
                                }
                                MP4D_close(mp4);
                                return 0 as c_int;
                            } else {
                                *((*tr).chunk_offset).offset(i as isize) <<= 32 as c_int;
                                let fresh955 = &mut (*((*tr).chunk_offset).offset(i as isize));
                                *fresh955 |= read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag)
                                    as c_ulong;
                            }
                        }
                        i = i.wrapping_add(1);
                    }
                }
            }
            1836476516 => {
                let mut t_0: boxsize_t = if payload_bytes
                    < (if FullAtomVersionAndFlags >> 24 as c_int == 1 as c_int as c_uint {
                        8 as c_int + 8 as c_int
                    } else {
                        4 as c_int + 4 as c_int
                    }) as c_ulong
                {
                    payload_bytes
                } else {
                    (if FullAtomVersionAndFlags >> 24 as c_int == 1 as c_int as c_uint {
                        8 as c_int + 8 as c_int
                    } else {
                        4 as c_int + 4 as c_int
                    }) as c_ulong
                };
                my_fseek(mp4, t_0, &mut eof_flag);
                payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_0) as boxsize_t as boxsize_t;
                (*mp4).timescale = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                (*mp4).duration_hi = if FullAtomVersionAndFlags >> 24 as c_int == 1 as c_int as c_uint {
                    read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag)
                } else {
                    0 as c_int as c_uint
                };
                (*mp4).duration_lo = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                let mut t_1: boxsize_t = if payload_bytes
                    < (4 as c_int
                        + 2 as c_int
                        + 2 as c_int
                        + 4 as c_int * 2 as c_int
                        + 4 as c_int * 9 as c_int
                        + 4 as c_int * 6 as c_int
                        + 4 as c_int) as c_ulong
                {
                    payload_bytes
                } else {
                    (4 as c_int
                        + 2 as c_int
                        + 2 as c_int
                        + 4 as c_int * 2 as c_int
                        + 4 as c_int * 9 as c_int
                        + 4 as c_int * 6 as c_int
                        + 4 as c_int) as c_ulong
                };
                my_fseek(mp4, t_1, &mut eof_flag);
                payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_1) as boxsize_t as boxsize_t;
            }
            1835296868 => {
                let mut t_2: boxsize_t = if payload_bytes
                    < (if FullAtomVersionAndFlags >> 24 as c_int == 1 as c_int as c_uint {
                        8 as c_int + 8 as c_int
                    } else {
                        4 as c_int + 4 as c_int
                    }) as c_ulong
                {
                    payload_bytes
                } else {
                    (if FullAtomVersionAndFlags >> 24 as c_int == 1 as c_int as c_uint {
                        8 as c_int + 8 as c_int
                    } else {
                        4 as c_int + 4 as c_int
                    }) as c_ulong
                };
                my_fseek(mp4, t_2, &mut eof_flag);
                payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_2) as boxsize_t as boxsize_t;
                (*tr).timescale = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                (*tr).duration_hi = if FullAtomVersionAndFlags >> 24 as c_int == 1 as c_int as c_uint {
                    read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag)
                } else {
                    0 as c_int as c_uint
                };
                (*tr).duration_lo = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                let mut ISO_639_2_T: c_int =
                    read_payload(mp4, 2 as c_int as c_uint, &mut payload_bytes, &mut eof_flag) as c_int;
                (*tr).language[2 as c_int as usize] = ((ISO_639_2_T & 31 as c_int) + 0x60 as c_int) as c_uchar;
                ISO_639_2_T >>= 5 as c_int;
                (*tr).language[1 as c_int as usize] = ((ISO_639_2_T & 31 as c_int) + 0x60 as c_int) as c_uchar;
                ISO_639_2_T >>= 5 as c_int;
                (*tr).language[0 as c_int as usize] = ((ISO_639_2_T & 31 as c_int) + 0x60 as c_int) as c_uchar;
            }
            1751411826 => {
                if !tr.is_null() {
                    let mut t_3: boxsize_t = if payload_bytes < 4 as c_int as c_ulong {
                        payload_bytes
                    } else {
                        4 as c_int as c_ulong
                    };
                    my_fseek(mp4, t_3, &mut eof_flag);
                    payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_3) as boxsize_t as boxsize_t;
                    (*tr).handler_type = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                }
            }
            1651798644 => {
                if tr.is_null() {
                    if depth != 0 {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                } else {
                    let mut t_4: boxsize_t = if payload_bytes < (4 as c_int + 4 as c_int) as c_ulong {
                        payload_bytes
                    } else {
                        (4 as c_int + 4 as c_int) as c_ulong
                    };
                    my_fseek(mp4, t_4, &mut eof_flag);
                    payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_4) as boxsize_t as boxsize_t;
                    (*tr).avg_bitrate_bps = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                }
            }
            2841734242 => {
                ptag = &mut (*mp4).tag.album;
            }
            2839630420 => {
                ptag = &mut (*mp4).tag.artist;
            }
            2842583405 => {
                ptag = &mut (*mp4).tag.title;
            }
            2841928057 => {
                ptag = &mut (*mp4).tag.year;
            }
            2841865588 => {
                ptag = &mut (*mp4).tag.comment;
            }
            2842125678 => {
                ptag = &mut (*mp4).tag.genre;
            }
            1937011556 => {
                let mut t_5: boxsize_t = if payload_bytes < 4 as c_int as c_ulong {
                    payload_bytes
                } else {
                    4 as c_int as c_ulong
                };
                my_fseek(mp4, t_5, &mut eof_flag);
                payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_5) as boxsize_t as boxsize_t;
            }
            1836070003 => {
                if tr.is_null() {
                    if depth != 0 {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                } else {
                    let mut t_6: boxsize_t = if payload_bytes < (6 as c_int * 1 as c_int + 2 as c_int) as c_ulong {
                        payload_bytes
                    } else {
                        (6 as c_int * 1 as c_int + 2 as c_int) as c_ulong
                    };
                    my_fseek(mp4, t_6, &mut eof_flag);
                    payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_6) as boxsize_t as boxsize_t;
                }
            }
            1836069985 => {
                if tr.is_null() {
                    if depth != 0 {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                } else {
                    let mut t_7: boxsize_t = if payload_bytes
                        < (6 as c_int * 1 as c_int + 2 as c_int + 4 as c_int * 2 as c_int) as c_ulong
                    {
                        payload_bytes
                    } else {
                        (6 as c_int * 1 as c_int + 2 as c_int + 4 as c_int * 2 as c_int) as c_ulong
                    };
                    my_fseek(mp4, t_7, &mut eof_flag);
                    payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_7) as boxsize_t as boxsize_t;
                    (*tr).SampleDescription.audio.channelcount =
                        read_payload(mp4, 2 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                    let mut t_8: boxsize_t = if payload_bytes < (2 as c_int + 2 as c_int + 2 as c_int) as c_ulong {
                        payload_bytes
                    } else {
                        (2 as c_int + 2 as c_int + 2 as c_int) as c_ulong
                    };
                    my_fseek(mp4, t_8, &mut eof_flag);
                    payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_8) as boxsize_t as boxsize_t;
                    (*tr).SampleDescription.audio.samplerate_hz =
                        read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag) >> 16 as c_int;
                }
            }
            1635148593 | 1836070006 => {
                if tr.is_null() {
                    if depth != 0 {
                        MP4D_close(mp4);
                        return 0 as c_int;
                    }
                } else {
                    let mut t_9: boxsize_t = if payload_bytes
                        < (6 as c_int * 1 as c_int + 2 as c_int + 2 as c_int + 2 as c_int + 4 as c_int * 3 as c_int)
                            as c_ulong
                    {
                        payload_bytes
                    } else {
                        (6 as c_int * 1 as c_int + 2 as c_int + 2 as c_int + 2 as c_int + 4 as c_int * 3 as c_int)
                            as c_ulong
                    };
                    my_fseek(mp4, t_9, &mut eof_flag);
                    payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_9) as boxsize_t as boxsize_t;
                    (*tr).SampleDescription.video.width =
                        read_payload(mp4, 2 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                    (*tr).SampleDescription.video.height =
                        read_payload(mp4, 2 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                    let mut t_10: boxsize_t = if payload_bytes
                        < (4 as c_int + 4 as c_int + 4 as c_int + 2 as c_int + 32 as c_int + 2 as c_int + 2 as c_int)
                            as c_ulong
                    {
                        payload_bytes
                    } else {
                        (4 as c_int + 4 as c_int + 4 as c_int + 2 as c_int + 32 as c_int + 2 as c_int + 2 as c_int)
                            as c_ulong
                    };
                    my_fseek(mp4, t_10, &mut eof_flag);
                    payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_10) as boxsize_t as boxsize_t;
                }
            }
            1635148611 => {
                (*tr).object_type_indication = 0x21 as c_int as c_uint;
                (*tr).dsi = minimp4_malloc(box_bytes) as *mut c_uchar;
                (*tr).dsi_bytes = box_bytes as c_uint;
                let mut spspps: c_int = 0;
                let mut p: *mut c_uchar = (*tr).dsi;
                let mut _configurationVersion: c_uint =
                    read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                let mut _AVCProfileIndication: c_uint =
                    read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                let mut _profile_compatibility: c_uint =
                    read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                let mut _AVCLevelIndication: c_uint =
                    read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                let mut _lengthSizeMinusOne: c_uint =
                    read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag) & 3 as c_int as c_uint;
                spspps = 0 as c_int;
                while spspps < 2 as c_int {
                    let mut numOfSequenceParameterSets: c_uint =
                        read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                    if spspps == 0 {
                        numOfSequenceParameterSets &= 31 as c_int as c_uint;
                    }
                    let fresh956 = p;
                    p = p.offset(1);
                    *fresh956 = numOfSequenceParameterSets as c_uchar;
                    i = 0 as c_int as c_uint;
                    while i < numOfSequenceParameterSets {
                        let mut k_0: c_uint = 0;
                        let mut sequenceParameterSetLength: c_uint =
                            read_payload(mp4, 2 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                        let fresh957 = p;
                        p = p.offset(1);
                        *fresh957 = (sequenceParameterSetLength >> 8 as c_int) as c_uchar;
                        let fresh958 = p;
                        p = p.offset(1);
                        *fresh958 = sequenceParameterSetLength as c_uchar;
                        k_0 = 0 as c_int as c_uint;
                        while k_0 < sequenceParameterSetLength {
                            let fresh959 = p;
                            p = p.offset(1);
                            *fresh959 =
                                read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag) as c_uchar;
                            k_0 = k_0.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                    }
                    spspps += 1;
                }
            }
            606348339 => {
                let mut flags: c_uint = read_payload(mp4, 3 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                if flags & 0x80 as c_int as c_uint != 0 {
                    let mut t_11: boxsize_t = if payload_bytes < 2 as c_int as c_ulong {
                        payload_bytes
                    } else {
                        2 as c_int as c_ulong
                    };
                    my_fseek(mp4, t_11, &mut eof_flag);
                    payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_11) as boxsize_t as boxsize_t;
                }
                if flags & 0x40 as c_int as c_uint != 0 {
                    let mut bytecount: c_uint =
                        read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                    let mut t_12: boxsize_t = if payload_bytes < bytecount as c_ulong {
                        payload_bytes
                    } else {
                        bytecount as c_ulong
                    };
                    my_fseek(mp4, t_12, &mut eof_flag);
                    payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_12) as boxsize_t as boxsize_t;
                }
                if flags & 0x20 as c_int as c_uint != 0 {
                    let mut t_13: boxsize_t = if payload_bytes < 2 as c_int as c_ulong {
                        payload_bytes
                    } else {
                        2 as c_int as c_ulong
                    };
                    my_fseek(mp4, t_13, &mut eof_flag);
                    payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_13) as boxsize_t as boxsize_t;
                }
            }
            606348340 => {
                if !tr.is_null() {
                } else {
                    __assert_fail(
                        b"tr\0" as *const u8 as *const c_char,
                        b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                        3094 as c_int as c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 89],
                            &[c_char; 89],
                        >(
                            b"int MP4D_open(MP4D_demux_t *, int (*)(int64_t, void *, size_t, void *), void *, int64_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if !tr.is_null() {
                } else {
                    __assert_fail(
                            b"tr\0" as *const u8 as *const c_char,
                            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                            3094 as c_int as c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 89],
                                &[c_char; 89],
                            >(
                                b"int MP4D_open(MP4D_demux_t *, int (*)(int64_t, void *, size_t, void *), void *, int64_t)\0",
                            ))
                                .as_ptr(),
                        );
                };
                (*tr).object_type_indication =
                    read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
                (*tr).stream_type =
                    read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag) >> 2 as c_int;
                let mut t_14: boxsize_t = if payload_bytes < (3 as c_int + 4 as c_int) as c_ulong {
                    payload_bytes
                } else {
                    (3 as c_int + 4 as c_int) as c_ulong
                };
                my_fseek(mp4, t_14, &mut eof_flag);
                payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_14) as boxsize_t as boxsize_t;
                (*tr).avg_bitrate_bps = read_payload(mp4, 4 as c_int as c_uint, &mut payload_bytes, &mut eof_flag);
            }
            606348341 => {
                if !tr.is_null() {
                } else {
                    __assert_fail(
                        b"tr\0" as *const u8 as *const c_char,
                        b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                        3106 as c_int as c_uint,
                        (*::core::mem::transmute::<
                            &[u8; 89],
                            &[c_char; 89],
                        >(
                            b"int MP4D_open(MP4D_demux_t *, int (*)(int64_t, void *, size_t, void *), void *, int64_t)\0",
                        ))
                            .as_ptr(),
                    );
                }
                if !tr.is_null() {
                } else {
                    __assert_fail(
                            b"tr\0" as *const u8 as *const c_char,
                            b"/minimp4/minimp4.h\0" as *const u8 as *const c_char,
                            3106 as c_int as c_uint,
                            (*::core::mem::transmute::<
                                &[u8; 89],
                                &[c_char; 89],
                            >(
                                b"int MP4D_open(MP4D_demux_t *, int (*)(int64_t, void *, size_t, void *), void *, int64_t)\0",
                            ))
                                .as_ptr(),
                        );
                };
                if ((*tr).dsi).is_null() && payload_bytes != 0 {
                    (*tr).dsi = minimp4_malloc(payload_bytes as c_int as size_t) as *mut c_uchar;
                    if ((*tr).dsi).is_null() {
                        if depth != 0 {
                            MP4D_close(mp4);
                            return 0 as c_int;
                        }
                    } else {
                        i = 0 as c_int as c_uint;
                        while (i as c_ulong) < payload_bytes {
                            *((*tr).dsi).offset(i as isize) = minimp4_read(mp4, 1 as c_int, &mut eof_flag) as c_uchar;
                            i = i.wrapping_add(1);
                        }
                        (*tr).dsi_bytes = i;
                        payload_bytes = (payload_bytes as c_ulong).wrapping_sub(i as c_ulong) as boxsize_t as boxsize_t;
                    }
                }
            }
            _ => {}
        }
        if !ptag.is_null() && (*ptag).is_null() && payload_bytes > 16 as c_int as c_ulong {
            let mut t_15: boxsize_t = if payload_bytes < (4 as c_int + 4 as c_int + 4 as c_int + 4 as c_int) as c_ulong
            {
                payload_bytes
            } else {
                (4 as c_int + 4 as c_int + 4 as c_int + 4 as c_int) as c_ulong
            };
            my_fseek(mp4, t_15, &mut eof_flag);
            payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_15) as boxsize_t as boxsize_t;
            *ptag =
                minimp4_malloc((payload_bytes as c_uint).wrapping_add(1 as c_int as c_uint) as size_t) as *mut c_uchar;
            if (*ptag).is_null() {
                if depth == 0 {
                    break;
                }
                MP4D_close(mp4);
                return 0 as c_int;
            } else {
                i = 0 as c_int as c_uint;
                while payload_bytes != 0 as c_int as c_ulong {
                    *(*ptag).offset(i as isize) =
                        read_payload(mp4, 1 as c_int as c_uint, &mut payload_bytes, &mut eof_flag) as c_uchar;
                    i = i.wrapping_add(1);
                }
                *(*ptag).offset(i as isize) = 0 as c_int as c_uchar;
            }
        }
        if box_name == BOX_trak as c_int as c_uint {
            let mut mem: *mut c_void = minimp4_realloc(
                (*mp4).track as *mut c_void,
                (((*mp4).track_count).wrapping_add(1 as c_int as c_uint) as c_ulong)
                    .wrapping_mul(::core::mem::size_of::<MP4D_track_t>() as c_ulong),
            );
            if mem.is_null() {
                if depth == 0 {
                    break;
                }
                MP4D_close(mp4);
                return 0 as c_int;
            } else {
                (*mp4).track = mem as *mut MP4D_track_t;
                let fresh960 = (*mp4).track_count;
                (*mp4).track_count = ((*mp4).track_count).wrapping_add(1);
                tr = ((*mp4).track).offset(fresh960 as isize);
                minimp4_memset(
                    tr as *mut c_void,
                    0 as c_int,
                    ::core::mem::size_of::<MP4D_track_t>() as c_ulong,
                );
            }
        } else if box_name == BOX_meta as c_int as c_uint {
            tr = std::ptr::null_mut::<MP4D_track_t>();
        }
        i = 0 as c_int as c_uint;
        while (i as c_ulong)
            < (::core::mem::size_of::<[C2RustUnnamed_8; 20]>() as c_ulong)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed_8>() as c_ulong)
        {
            if box_name == g_envelope_box[i as usize].name {
                depth += 1;
                if depth >= 64 as c_int {
                    if depth == 0 {
                        break;
                    }
                    MP4D_close(mp4);
                    return 0 as c_int;
                } else {
                    stack[depth as usize].bytes = payload_bytes;
                    stack[depth as usize].format = g_envelope_box[i as usize].type_0;
                    break;
                }
            } else {
                i = i.wrapping_add(1);
            }
        }
        if i as c_ulong
            == (::core::mem::size_of::<[C2RustUnnamed_8; 20]>() as c_ulong)
                .wrapping_div(::core::mem::size_of::<C2RustUnnamed_8>() as c_ulong)
        {
            if payload_bytes > file_size as c_ulong {
                eof_flag = 1 as c_int;
            } else {
                let mut t_16: boxsize_t = if payload_bytes < payload_bytes {
                    payload_bytes
                } else {
                    payload_bytes
                };
                my_fseek(mp4, t_16, &mut eof_flag);
                payload_bytes = (payload_bytes as c_ulong).wrapping_sub(t_16) as boxsize_t as boxsize_t;
            }
        }
        while depth > 0 as c_int && stack[depth as usize].bytes == 0 {
            depth -= 1;
        }
        if eof_flag != 0 {
            break;
        }
    }
    if (*mp4).track_count == 0 {
        MP4D_close(mp4);
        return 0 as c_int;
    }
    1 as c_int
}
unsafe extern "C" fn sample_to_chunk(
    mut tr: *mut MP4D_track_t,
    mut nsample: c_uint,
    mut nfirst_sample_in_chunk: *mut c_uint,
) -> c_int {
    let mut chunk_group: c_uint = 0 as c_int as c_uint;
    let mut nc: c_uint = 0;
    let mut sum: c_uint = 0 as c_int as c_uint;
    *nfirst_sample_in_chunk = 0 as c_int as c_uint;
    if (*tr).chunk_count <= 1 as c_int as c_uint {
        return 0 as c_int;
    }
    nc = 0 as c_int as c_uint;
    while nc < (*tr).chunk_count {
        if chunk_group.wrapping_add(1 as c_int as c_uint) < (*tr).sample_to_chunk_count
            && nc.wrapping_add(1 as c_int as c_uint)
                == (*((*tr).sample_to_chunk).offset(chunk_group.wrapping_add(1 as c_int as c_uint) as isize))
                    .first_chunk
        {
            chunk_group = chunk_group.wrapping_add(1);
        }
        sum = sum.wrapping_add((*((*tr).sample_to_chunk).offset(chunk_group as isize)).samples_per_chunk);
        if nsample < sum {
            return nc as c_int;
        }
        *nfirst_sample_in_chunk = sum;
        nc = nc.wrapping_add(1);
    }
    -(1 as c_int)
}
#[no_mangle]
pub unsafe extern "C" fn MP4D_frame_offset(
    mut mp4: *const MP4D_demux_t,
    mut ntrack: c_uint,
    mut nsample: c_uint,
    mut frame_bytes: *mut c_uint,
    mut timestamp: *mut c_uint,
    mut duration: *mut c_uint,
) -> MP4D_file_offset_t {
    let mut tr: *mut MP4D_track_t = ((*mp4).track).offset(ntrack as isize);
    let mut ns: c_uint = 0;
    let mut nchunk: c_int = sample_to_chunk(tr, nsample, &mut ns);
    let mut offset: MP4D_file_offset_t = 0;
    if nchunk < 0 as c_int {
        *frame_bytes = 0 as c_int as c_uint;
        return 0 as c_int as MP4D_file_offset_t;
    }
    offset = *((*tr).chunk_offset).offset(nchunk as isize);
    while ns < nsample {
        offset = (offset as c_ulong).wrapping_add(*((*tr).entry_size).offset(ns as isize) as c_ulong)
            as MP4D_file_offset_t as MP4D_file_offset_t;
        ns = ns.wrapping_add(1);
    }
    *frame_bytes = *((*tr).entry_size).offset(ns as isize);
    if !timestamp.is_null() {
        *timestamp = *((*tr).timestamp).offset(ns as isize);
    }
    if !duration.is_null() {
        *duration = *((*tr).duration).offset(ns as isize);
    }
    offset
}
#[no_mangle]
pub unsafe extern "C" fn MP4D_close(mut mp4: *mut MP4D_demux_t) {
    while (*mp4).track_count != 0 {
        (*mp4).track_count = ((*mp4).track_count).wrapping_sub(1);
        let mut tr: *mut MP4D_track_t = ((*mp4).track).offset((*mp4).track_count as isize);
        if !((*tr).entry_size).is_null() {
            minimp4_free((*tr).entry_size as *mut c_void);
            (*tr).entry_size = std::ptr::null_mut::<c_uint>();
        }
        if !((*tr).timestamp).is_null() {
            minimp4_free((*tr).timestamp as *mut c_void);
            (*tr).timestamp = std::ptr::null_mut::<c_uint>();
        }
        if !((*tr).duration).is_null() {
            minimp4_free((*tr).duration as *mut c_void);
            (*tr).duration = std::ptr::null_mut::<c_uint>();
        }
        if !((*tr).sample_to_chunk).is_null() {
            minimp4_free((*tr).sample_to_chunk as *mut c_void);
            (*tr).sample_to_chunk = std::ptr::null_mut::<MP4D_sample_to_chunk_t_tag>();
        }
        if !((*tr).chunk_offset).is_null() {
            minimp4_free((*tr).chunk_offset as *mut c_void);
            (*tr).chunk_offset = std::ptr::null_mut::<MP4D_file_offset_t>();
        }
        if !((*tr).dsi).is_null() {
            minimp4_free((*tr).dsi as *mut c_void);
            (*tr).dsi = std::ptr::null_mut::<c_uchar>();
        }
    }
    if !((*mp4).track).is_null() {
        minimp4_free((*mp4).track as *mut c_void);
        (*mp4).track = std::ptr::null_mut::<MP4D_track_t>();
    }
    if !((*mp4).tag.title).is_null() {
        minimp4_free((*mp4).tag.title as *mut c_void);
        (*mp4).tag.title = std::ptr::null_mut::<c_uchar>();
    }
    if !((*mp4).tag.artist).is_null() {
        minimp4_free((*mp4).tag.artist as *mut c_void);
        (*mp4).tag.artist = std::ptr::null_mut::<c_uchar>();
    }
    if !((*mp4).tag.album).is_null() {
        minimp4_free((*mp4).tag.album as *mut c_void);
        (*mp4).tag.album = std::ptr::null_mut::<c_uchar>();
    }
    if !((*mp4).tag.year).is_null() {
        minimp4_free((*mp4).tag.year as *mut c_void);
        (*mp4).tag.year = std::ptr::null_mut::<c_uchar>();
    }
    if !((*mp4).tag.comment).is_null() {
        minimp4_free((*mp4).tag.comment as *mut c_void);
        (*mp4).tag.comment = std::ptr::null_mut::<c_uchar>();
    }
    if !((*mp4).tag.genre).is_null() {
        minimp4_free((*mp4).tag.genre as *mut c_void);
        (*mp4).tag.genre = std::ptr::null_mut::<c_uchar>();
    }
}
unsafe extern "C" fn skip_spspps(mut p: *const c_uchar, mut nbytes: c_int, mut nskip: c_int) -> c_int {
    let mut i: c_int = 0;
    let mut k: c_int = 0 as c_int;
    i = 0 as c_int;
    while i < nskip {
        let mut segmbytes: c_uint = 0;
        if k > nbytes - 2 as c_int {
            return -(1 as c_int);
        }
        segmbytes =
            (*p.offset(k as isize) as c_int * 256 as c_int + *p.offset((k + 1 as c_int) as isize) as c_int) as c_uint;
        k = (k as c_uint).wrapping_add((2 as c_int as c_uint).wrapping_add(segmbytes)) as c_int as c_int;
        i += 1;
    }
    k
}
unsafe extern "C" fn MP4D_read_spspps(
    mut mp4: *const MP4D_demux_t,
    mut ntrack: c_uint,
    mut pps_flag: c_int,
    mut nsps: c_int,
    mut sps_bytes: *mut c_int,
) -> *const c_void {
    let mut sps_count: c_int = 0;
    let mut skip_bytes: c_int = 0;
    let mut bytepos: c_int = 0 as c_int;
    let mut p: *mut c_uchar = (*((*mp4).track).offset(ntrack as isize)).dsi;
    if ntrack >= (*mp4).track_count {
        return std::ptr::null::<c_void>();
    }
    if (*((*mp4).track).offset(ntrack as isize)).object_type_indication != 0x21 as c_int as c_uint {
        return std::ptr::null::<c_void>();
    }
    if pps_flag != 0 {
        let fresh961 = bytepos;
        bytepos += 1;
        sps_count = *p.offset(fresh961 as isize) as c_int;
        skip_bytes = skip_spspps(
            p.offset(bytepos as isize),
            ((*((*mp4).track).offset(ntrack as isize)).dsi_bytes).wrapping_sub(bytepos as c_uint) as c_int,
            sps_count,
        );
        if skip_bytes < 0 as c_int {
            return std::ptr::null::<c_void>();
        }
        bytepos += skip_bytes;
    }
    let fresh962 = bytepos;
    bytepos += 1;
    sps_count = *p.offset(fresh962 as isize) as c_int;
    if nsps >= sps_count {
        return std::ptr::null::<c_void>();
    }
    skip_bytes = skip_spspps(
        p.offset(bytepos as isize),
        ((*((*mp4).track).offset(ntrack as isize)).dsi_bytes).wrapping_sub(bytepos as c_uint) as c_int,
        nsps,
    );
    if skip_bytes < 0 as c_int {
        return std::ptr::null::<c_void>();
    }
    bytepos += skip_bytes;
    *sps_bytes =
        *p.offset(bytepos as isize) as c_int * 256 as c_int + *p.offset((bytepos + 1 as c_int) as isize) as c_int;
    p.offset(bytepos as isize).offset(2 as c_int as isize) as *const c_void
}
#[no_mangle]
pub unsafe extern "C" fn MP4D_read_sps(
    mut mp4: *const MP4D_demux_t,
    mut ntrack: c_uint,
    mut nsps: c_int,
    mut sps_bytes: *mut c_int,
) -> *const c_void {
    MP4D_read_spspps(mp4, ntrack, 0 as c_int, nsps, sps_bytes)
}
#[no_mangle]
pub unsafe extern "C" fn MP4D_read_pps(
    mut mp4: *const MP4D_demux_t,
    mut ntrack: c_uint,
    mut npps: c_int,
    mut pps_bytes: *mut c_int,
) -> *const c_void {
    MP4D_read_spspps(mp4, ntrack, 1 as c_int, npps, pps_bytes)
}

type _IO_wide_data = u8;
type _IO_codecvt = u8;
type _IO_marker = u8;
pub use crate::c::environment::types::*;
