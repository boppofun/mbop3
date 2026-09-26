#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
// Edit from the raw c2rust output: libc mem functions implemented with core.
unsafe fn memcpy(
    dest: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    n: size_t,
) -> *mut ::core::ffi::c_void {
    ::core::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, n);
    dest
}
unsafe fn memmove(
    dest: *mut ::core::ffi::c_void,
    src: *const ::core::ffi::c_void,
    n: size_t,
) -> *mut ::core::ffi::c_void {
    ::core::ptr::copy(src as *const u8, dest as *mut u8, n);
    dest
}
unsafe fn memset(
    s: *mut ::core::ffi::c_void,
    c: ::core::ffi::c_int,
    n: size_t,
) -> *mut ::core::ffi::c_void {
    ::core::ptr::write_bytes(s as *mut u8, c as u8, n);
    s
}
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp3dec_frame_info_t {
    pub frame_bytes: ::core::ffi::c_int,
    pub frame_offset: ::core::ffi::c_int,
    pub channels: ::core::ffi::c_int,
    pub hz: ::core::ffi::c_int,
    pub layer: ::core::ffi::c_int,
    pub bitrate_kbps: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp3dec_t {
    pub mdct_overlap: [[::core::ffi::c_float; 288]; 2],
    pub qmf_state: [::core::ffi::c_float; 960],
    pub reserv: ::core::ffi::c_int,
    pub free_format_bytes: ::core::ffi::c_int,
    pub header: [::core::ffi::c_uchar; 4],
    pub reserv_buf: [::core::ffi::c_uchar; 511],
}
pub type mp3d_sample_t = int16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mp3dec_scratch_t {
    pub bs: bs_t,
    pub maindata: [uint8_t; 2815],
    pub gr_info: [L3_gr_info_t; 4],
    pub grbuf: [[::core::ffi::c_float; 576]; 2],
    pub scf: [::core::ffi::c_float; 40],
    pub syn: [[::core::ffi::c_float; 64]; 33],
    pub ist_pos: [[uint8_t; 39]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct L3_gr_info_t {
    pub sfbtab: *const uint8_t,
    pub part_23_length: uint16_t,
    pub big_values: uint16_t,
    pub scalefac_compress: uint16_t,
    pub global_gain: uint8_t,
    pub block_type: uint8_t,
    pub mixed_block_flag: uint8_t,
    pub n_long_sfb: uint8_t,
    pub n_short_sfb: uint8_t,
    pub table_select: [uint8_t; 3],
    pub region_count: [uint8_t; 3],
    pub subblock_gain: [uint8_t; 3],
    pub preflag: uint8_t,
    pub scalefac_scale: uint8_t,
    pub count1_table: uint8_t,
    pub scfsi: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bs_t {
    pub buf: *const uint8_t,
    pub pos: ::core::ffi::c_int,
    pub limit: ::core::ffi::c_int,
}
pub type size_t = usize;
pub const MAX_FREE_FORMAT_FRAME_SIZE: ::core::ffi::c_int = 2304 as ::core::ffi::c_int;
pub const MAX_FRAME_SYNC_MATCHES: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MAX_BITRESERVOIR_BYTES: ::core::ffi::c_int = 511 as ::core::ffi::c_int;
pub const SHORT_BLOCK_TYPE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STOP_BLOCK_TYPE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const HDR_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BITS_DEQUANTIZER_OUT: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const MAX_SCF: ::core::ffi::c_int = 255 as ::core::ffi::c_int
    + BITS_DEQUANTIZER_OUT * 4 as ::core::ffi::c_int - 210 as ::core::ffi::c_int;
pub const MAX_SCFI: ::core::ffi::c_int = MAX_SCF + 3 as ::core::ffi::c_int
    & !(3 as ::core::ffi::c_int);
unsafe extern "C" fn bs_init(
    mut bs: *mut bs_t,
    mut data: *const uint8_t,
    mut bytes: ::core::ffi::c_int,
) {
    (*bs).buf = data;
    (*bs).pos = 0 as ::core::ffi::c_int;
    (*bs).limit = bytes * 8 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_bits(mut bs: *mut bs_t, mut n: ::core::ffi::c_int) -> uint32_t {
    let mut next: uint32_t = 0;
    let mut cache: uint32_t = 0 as uint32_t;
    let mut s: uint32_t = ((*bs).pos & 7 as ::core::ffi::c_int) as uint32_t;
    let mut shl: ::core::ffi::c_int = (n as uint32_t).wrapping_add(s)
        as ::core::ffi::c_int;
    let mut p: *const uint8_t = (*bs)
        .buf
        .offset(((*bs).pos >> 3 as ::core::ffi::c_int) as isize);
    (*bs).pos += n;
    if (*bs).pos > (*bs).limit {
        return 0 as uint32_t;
    }
    let fresh36 = p;
    p = p.offset(1);
    next = (*fresh36 as ::core::ffi::c_int & 255 as ::core::ffi::c_int >> s) as uint32_t;
    loop {
        shl -= 8 as ::core::ffi::c_int;
        if !(shl > 0 as ::core::ffi::c_int) {
            break;
        }
        cache |= next << shl;
        let fresh37 = p;
        p = p.offset(1);
        next = *fresh37 as uint32_t;
    }
    return cache | next >> -shl;
}
unsafe extern "C" fn hdr_valid(mut h: *const uint8_t) -> ::core::ffi::c_int {
    return (*h.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 0xff as ::core::ffi::c_int
        && (*h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xf0 as ::core::ffi::c_int == 0xf0 as ::core::ffi::c_int
            || *h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xfe as ::core::ffi::c_int == 0xe2 as ::core::ffi::c_int)
        && *h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        && *h.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int != 15 as ::core::ffi::c_int
        && *h.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int
            != 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn hdr_compare(
    mut h1: *const uint8_t,
    mut h2: *const uint8_t,
) -> ::core::ffi::c_int {
    return (hdr_valid(h2) != 0
        && (*h1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            ^ *h2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            & 0xfe as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*h1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            ^ *h2.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            & 0xc as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*h1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xf0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            ^ (*h2.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf0 as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn hdr_bitrate_kbps(mut h: *const uint8_t) -> ::core::ffi::c_uint {
    static mut halfrate: [[[uint8_t; 15]; 3]; 2] = [
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                4 as ::core::ffi::c_int as uint8_t,
                8 as ::core::ffi::c_int as uint8_t,
                12 as ::core::ffi::c_int as uint8_t,
                16 as ::core::ffi::c_int as uint8_t,
                20 as ::core::ffi::c_int as uint8_t,
                24 as ::core::ffi::c_int as uint8_t,
                28 as ::core::ffi::c_int as uint8_t,
                32 as ::core::ffi::c_int as uint8_t,
                40 as ::core::ffi::c_int as uint8_t,
                48 as ::core::ffi::c_int as uint8_t,
                56 as ::core::ffi::c_int as uint8_t,
                64 as ::core::ffi::c_int as uint8_t,
                72 as ::core::ffi::c_int as uint8_t,
                80 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                4 as ::core::ffi::c_int as uint8_t,
                8 as ::core::ffi::c_int as uint8_t,
                12 as ::core::ffi::c_int as uint8_t,
                16 as ::core::ffi::c_int as uint8_t,
                20 as ::core::ffi::c_int as uint8_t,
                24 as ::core::ffi::c_int as uint8_t,
                28 as ::core::ffi::c_int as uint8_t,
                32 as ::core::ffi::c_int as uint8_t,
                40 as ::core::ffi::c_int as uint8_t,
                48 as ::core::ffi::c_int as uint8_t,
                56 as ::core::ffi::c_int as uint8_t,
                64 as ::core::ffi::c_int as uint8_t,
                72 as ::core::ffi::c_int as uint8_t,
                80 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                16 as ::core::ffi::c_int as uint8_t,
                24 as ::core::ffi::c_int as uint8_t,
                28 as ::core::ffi::c_int as uint8_t,
                32 as ::core::ffi::c_int as uint8_t,
                40 as ::core::ffi::c_int as uint8_t,
                48 as ::core::ffi::c_int as uint8_t,
                56 as ::core::ffi::c_int as uint8_t,
                64 as ::core::ffi::c_int as uint8_t,
                72 as ::core::ffi::c_int as uint8_t,
                80 as ::core::ffi::c_int as uint8_t,
                88 as ::core::ffi::c_int as uint8_t,
                96 as ::core::ffi::c_int as uint8_t,
                112 as ::core::ffi::c_int as uint8_t,
                128 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                16 as ::core::ffi::c_int as uint8_t,
                20 as ::core::ffi::c_int as uint8_t,
                24 as ::core::ffi::c_int as uint8_t,
                28 as ::core::ffi::c_int as uint8_t,
                32 as ::core::ffi::c_int as uint8_t,
                40 as ::core::ffi::c_int as uint8_t,
                48 as ::core::ffi::c_int as uint8_t,
                56 as ::core::ffi::c_int as uint8_t,
                64 as ::core::ffi::c_int as uint8_t,
                80 as ::core::ffi::c_int as uint8_t,
                96 as ::core::ffi::c_int as uint8_t,
                112 as ::core::ffi::c_int as uint8_t,
                128 as ::core::ffi::c_int as uint8_t,
                160 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                16 as ::core::ffi::c_int as uint8_t,
                24 as ::core::ffi::c_int as uint8_t,
                28 as ::core::ffi::c_int as uint8_t,
                32 as ::core::ffi::c_int as uint8_t,
                40 as ::core::ffi::c_int as uint8_t,
                48 as ::core::ffi::c_int as uint8_t,
                56 as ::core::ffi::c_int as uint8_t,
                64 as ::core::ffi::c_int as uint8_t,
                80 as ::core::ffi::c_int as uint8_t,
                96 as ::core::ffi::c_int as uint8_t,
                112 as ::core::ffi::c_int as uint8_t,
                128 as ::core::ffi::c_int as uint8_t,
                160 as ::core::ffi::c_int as uint8_t,
                192 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                16 as ::core::ffi::c_int as uint8_t,
                32 as ::core::ffi::c_int as uint8_t,
                48 as ::core::ffi::c_int as uint8_t,
                64 as ::core::ffi::c_int as uint8_t,
                80 as ::core::ffi::c_int as uint8_t,
                96 as ::core::ffi::c_int as uint8_t,
                112 as ::core::ffi::c_int as uint8_t,
                128 as ::core::ffi::c_int as uint8_t,
                144 as ::core::ffi::c_int as uint8_t,
                160 as ::core::ffi::c_int as uint8_t,
                176 as ::core::ffi::c_int as uint8_t,
                192 as ::core::ffi::c_int as uint8_t,
                208 as ::core::ffi::c_int as uint8_t,
                224 as ::core::ffi::c_int as uint8_t,
            ],
        ],
    ];
    return (2 as ::core::ffi::c_int
        * halfrate[(*h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x8 as ::core::ffi::c_int != 0) as ::core::ffi::c_int
            as usize][((*h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int)
            as usize][(*h.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
}
unsafe extern "C" fn hdr_sample_rate_hz(mut h: *const uint8_t) -> ::core::ffi::c_uint {
    static mut g_hz: [::core::ffi::c_uint; 3] = [
        44100 as ::core::ffi::c_int as ::core::ffi::c_uint,
        48000 as ::core::ffi::c_int as ::core::ffi::c_uint,
        32000 as ::core::ffi::c_int as ::core::ffi::c_uint,
    ];
    return g_hz[(*h.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int) as usize]
        >> (*h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x8 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
        >> (*h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x10 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
}
unsafe extern "C" fn hdr_frame_samples(mut h: *const uint8_t) -> ::core::ffi::c_uint {
    return (if *h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 6 as ::core::ffi::c_int == 6 as ::core::ffi::c_int
    {
        384 as ::core::ffi::c_int
    } else {
        1152 as ::core::ffi::c_int
            >> (*h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 14 as ::core::ffi::c_int == 2 as ::core::ffi::c_int)
                as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
}
unsafe extern "C" fn hdr_frame_bytes(
    mut h: *const uint8_t,
    mut free_format_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut frame_bytes: ::core::ffi::c_int = hdr_frame_samples(h)
        .wrapping_mul(hdr_bitrate_kbps(h))
        .wrapping_mul(125 as ::core::ffi::c_uint)
        .wrapping_div(hdr_sample_rate_hz(h)) as ::core::ffi::c_int;
    if *h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 6 as ::core::ffi::c_int == 6 as ::core::ffi::c_int
    {
        frame_bytes &= !(3 as ::core::ffi::c_int);
    }
    return if frame_bytes != 0 { frame_bytes } else { free_format_size };
}
unsafe extern "C" fn hdr_padding(mut h: *const uint8_t) -> ::core::ffi::c_int {
    return if *h.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 0x2 as ::core::ffi::c_int != 0
    {
        if *h.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 6 as ::core::ffi::c_int == 6 as ::core::ffi::c_int
        {
            4 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn L3_read_side_info(
    mut bs: *mut bs_t,
    mut gr: *mut L3_gr_info_t,
    mut hdr: *const uint8_t,
) -> ::core::ffi::c_int {
    static mut g_scf_long: [[uint8_t; 23]; 8] = [
        [
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            38 as ::core::ffi::c_int as uint8_t,
            46 as ::core::ffi::c_int as uint8_t,
            52 as ::core::ffi::c_int as uint8_t,
            60 as ::core::ffi::c_int as uint8_t,
            68 as ::core::ffi::c_int as uint8_t,
            58 as ::core::ffi::c_int as uint8_t,
            54 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            48 as ::core::ffi::c_int as uint8_t,
            56 as ::core::ffi::c_int as uint8_t,
            64 as ::core::ffi::c_int as uint8_t,
            76 as ::core::ffi::c_int as uint8_t,
            90 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            38 as ::core::ffi::c_int as uint8_t,
            46 as ::core::ffi::c_int as uint8_t,
            52 as ::core::ffi::c_int as uint8_t,
            60 as ::core::ffi::c_int as uint8_t,
            68 as ::core::ffi::c_int as uint8_t,
            58 as ::core::ffi::c_int as uint8_t,
            54 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            22 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            38 as ::core::ffi::c_int as uint8_t,
            46 as ::core::ffi::c_int as uint8_t,
            54 as ::core::ffi::c_int as uint8_t,
            62 as ::core::ffi::c_int as uint8_t,
            70 as ::core::ffi::c_int as uint8_t,
            76 as ::core::ffi::c_int as uint8_t,
            36 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            38 as ::core::ffi::c_int as uint8_t,
            46 as ::core::ffi::c_int as uint8_t,
            52 as ::core::ffi::c_int as uint8_t,
            60 as ::core::ffi::c_int as uint8_t,
            68 as ::core::ffi::c_int as uint8_t,
            58 as ::core::ffi::c_int as uint8_t,
            54 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            34 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            50 as ::core::ffi::c_int as uint8_t,
            54 as ::core::ffi::c_int as uint8_t,
            76 as ::core::ffi::c_int as uint8_t,
            158 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            22 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            34 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            46 as ::core::ffi::c_int as uint8_t,
            54 as ::core::ffi::c_int as uint8_t,
            54 as ::core::ffi::c_int as uint8_t,
            192 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            38 as ::core::ffi::c_int as uint8_t,
            46 as ::core::ffi::c_int as uint8_t,
            56 as ::core::ffi::c_int as uint8_t,
            68 as ::core::ffi::c_int as uint8_t,
            84 as ::core::ffi::c_int as uint8_t,
            102 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    static mut g_scf_short: [[uint8_t; 40]; 8] = [
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            36 as ::core::ffi::c_int as uint8_t,
            36 as ::core::ffi::c_int as uint8_t,
            36 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            44 as ::core::ffi::c_int as uint8_t,
            44 as ::core::ffi::c_int as uint8_t,
            44 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            22 as ::core::ffi::c_int as uint8_t,
            22 as ::core::ffi::c_int as uint8_t,
            22 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            56 as ::core::ffi::c_int as uint8_t,
            56 as ::core::ffi::c_int as uint8_t,
            56 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            66 as ::core::ffi::c_int as uint8_t,
            66 as ::core::ffi::c_int as uint8_t,
            66 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            34 as ::core::ffi::c_int as uint8_t,
            34 as ::core::ffi::c_int as uint8_t,
            34 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    static mut g_scf_mixed: [[uint8_t; 40]; 8] = [
        [
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0,
            0,
            0,
        ],
        [
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            36 as ::core::ffi::c_int as uint8_t,
            36 as ::core::ffi::c_int as uint8_t,
            36 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0,
            0,
            0,
        ],
        [
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            44 as ::core::ffi::c_int as uint8_t,
            44 as ::core::ffi::c_int as uint8_t,
            44 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0,
            0,
            0,
        ],
        [
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0,
            0,
            0,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            22 as ::core::ffi::c_int as uint8_t,
            22 as ::core::ffi::c_int as uint8_t,
            22 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            56 as ::core::ffi::c_int as uint8_t,
            56 as ::core::ffi::c_int as uint8_t,
            56 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            66 as ::core::ffi::c_int as uint8_t,
            66 as ::core::ffi::c_int as uint8_t,
            66 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            34 as ::core::ffi::c_int as uint8_t,
            34 as ::core::ffi::c_int as uint8_t,
            34 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0,
        ],
    ];
    let mut tables: ::core::ffi::c_uint = 0;
    let mut scfsi: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut main_data_begin: ::core::ffi::c_int = 0;
    let mut part_23_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sr_idx: ::core::ffi::c_int = (*hdr.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int >> 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
        + ((*hdr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
            + (*hdr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int & 1 as ::core::ffi::c_int))
            * 3 as ::core::ffi::c_int;
    sr_idx -= (sr_idx != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut gr_count: ::core::ffi::c_int = if *hdr
        .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 0xc0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    };
    if *hdr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 0x8 as ::core::ffi::c_int != 0
    {
        gr_count *= 2 as ::core::ffi::c_int;
        main_data_begin = get_bits(bs, 9 as ::core::ffi::c_int) as ::core::ffi::c_int;
        scfsi = get_bits(bs, 7 as ::core::ffi::c_int + gr_count) as ::core::ffi::c_uint;
    } else {
        main_data_begin = (get_bits(bs, 8 as ::core::ffi::c_int + gr_count) >> gr_count)
            as ::core::ffi::c_int;
    }
    loop {
        if *hdr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xc0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int
        {
            scfsi <<= 4 as ::core::ffi::c_int;
        }
        (*gr).part_23_length = get_bits(bs, 12 as ::core::ffi::c_int) as uint16_t;
        part_23_sum += (*gr).part_23_length as ::core::ffi::c_int;
        (*gr).big_values = get_bits(bs, 9 as ::core::ffi::c_int) as uint16_t;
        if (*gr).big_values as ::core::ffi::c_int > 288 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        (*gr).global_gain = get_bits(bs, 8 as ::core::ffi::c_int) as uint8_t;
        (*gr).scalefac_compress = get_bits(
            bs,
            if *hdr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x8 as ::core::ffi::c_int != 0
            {
                4 as ::core::ffi::c_int
            } else {
                9 as ::core::ffi::c_int
            },
        ) as uint16_t;
        (*gr).sfbtab = &raw const *(&raw const g_scf_long as *const [uint8_t; 23])
            .offset(sr_idx as isize) as *const uint8_t;
        (*gr).n_long_sfb = 22 as uint8_t;
        (*gr).n_short_sfb = 0 as uint8_t;
        if get_bits(bs, 1 as ::core::ffi::c_int) != 0 {
            (*gr).block_type = get_bits(bs, 2 as ::core::ffi::c_int) as uint8_t;
            if (*gr).block_type == 0 {
                return -(1 as ::core::ffi::c_int);
            }
            (*gr).mixed_block_flag = get_bits(bs, 1 as ::core::ffi::c_int) as uint8_t;
            (*gr).region_count[0 as ::core::ffi::c_int as usize] = 7 as uint8_t;
            (*gr).region_count[1 as ::core::ffi::c_int as usize] = 255 as uint8_t;
            if (*gr).block_type as ::core::ffi::c_int == SHORT_BLOCK_TYPE {
                scfsi &= 0xf0f as ::core::ffi::c_uint;
                if (*gr).mixed_block_flag == 0 {
                    (*gr).region_count[0 as ::core::ffi::c_int as usize] = 8 as uint8_t;
                    (*gr).sfbtab = &raw const *(&raw const g_scf_short
                        as *const [uint8_t; 40])
                        .offset(sr_idx as isize) as *const uint8_t;
                    (*gr).n_long_sfb = 0 as uint8_t;
                    (*gr).n_short_sfb = 39 as uint8_t;
                } else {
                    (*gr).sfbtab = &raw const *(&raw const g_scf_mixed
                        as *const [uint8_t; 40])
                        .offset(sr_idx as isize) as *const uint8_t;
                    (*gr).n_long_sfb = (if *hdr.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
                    {
                        8 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    }) as uint8_t;
                    (*gr).n_short_sfb = 30 as uint8_t;
                }
            }
            tables = get_bits(bs, 10 as ::core::ffi::c_int) as ::core::ffi::c_uint;
            tables <<= 5 as ::core::ffi::c_int;
            (*gr).subblock_gain[0 as ::core::ffi::c_int as usize] = get_bits(
                bs,
                3 as ::core::ffi::c_int,
            ) as uint8_t;
            (*gr).subblock_gain[1 as ::core::ffi::c_int as usize] = get_bits(
                bs,
                3 as ::core::ffi::c_int,
            ) as uint8_t;
            (*gr).subblock_gain[2 as ::core::ffi::c_int as usize] = get_bits(
                bs,
                3 as ::core::ffi::c_int,
            ) as uint8_t;
        } else {
            (*gr).block_type = 0 as uint8_t;
            (*gr).mixed_block_flag = 0 as uint8_t;
            tables = get_bits(bs, 15 as ::core::ffi::c_int) as ::core::ffi::c_uint;
            (*gr).region_count[0 as ::core::ffi::c_int as usize] = get_bits(
                bs,
                4 as ::core::ffi::c_int,
            ) as uint8_t;
            (*gr).region_count[1 as ::core::ffi::c_int as usize] = get_bits(
                bs,
                3 as ::core::ffi::c_int,
            ) as uint8_t;
            (*gr).region_count[2 as ::core::ffi::c_int as usize] = 255 as uint8_t;
        }
        (*gr).table_select[0 as ::core::ffi::c_int as usize] = (tables
            >> 10 as ::core::ffi::c_int) as uint8_t;
        (*gr).table_select[1 as ::core::ffi::c_int as usize] = (tables
            >> 5 as ::core::ffi::c_int & 31 as ::core::ffi::c_uint) as uint8_t;
        (*gr).table_select[2 as ::core::ffi::c_int as usize] = (tables
            & 31 as ::core::ffi::c_uint) as uint8_t;
        (*gr).preflag = (if *hdr.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int != 0
        {
            get_bits(bs, 1 as ::core::ffi::c_int)
        } else {
            ((*gr).scalefac_compress as ::core::ffi::c_int >= 500 as ::core::ffi::c_int)
                as ::core::ffi::c_int as uint32_t
        }) as uint8_t;
        (*gr).scalefac_scale = get_bits(bs, 1 as ::core::ffi::c_int) as uint8_t;
        (*gr).count1_table = get_bits(bs, 1 as ::core::ffi::c_int) as uint8_t;
        (*gr).scfsi = (scfsi >> 12 as ::core::ffi::c_int & 15 as ::core::ffi::c_uint)
            as uint8_t;
        scfsi <<= 4 as ::core::ffi::c_int;
        gr = gr.offset(1);
        gr_count -= 1;
        if !(gr_count != 0) {
            break;
        }
    }
    if part_23_sum + (*bs).pos > (*bs).limit + main_data_begin * 8 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    return main_data_begin;
}
unsafe extern "C" fn L3_read_scalefactors(
    mut scf: *mut uint8_t,
    mut ist_pos: *mut uint8_t,
    mut scf_size: *const uint8_t,
    mut scf_count: *const uint8_t,
    mut bitbuf: *mut bs_t,
    mut scfsi: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int
        && *scf_count.offset(i as isize) as ::core::ffi::c_int != 0
    {
        let mut cnt: ::core::ffi::c_int = *scf_count.offset(i as isize)
            as ::core::ffi::c_int;
        if scfsi & 8 as ::core::ffi::c_int != 0 {
            memcpy(
                scf as *mut ::core::ffi::c_void,
                ist_pos as *const ::core::ffi::c_void,
                cnt as size_t,
            );
        } else {
            let mut bits: ::core::ffi::c_int = *scf_size.offset(i as isize)
                as ::core::ffi::c_int;
            if bits == 0 {
                memset(
                    scf as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    cnt as size_t,
                );
                memset(
                    ist_pos as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    cnt as size_t,
                );
            } else {
                let mut max_scf: ::core::ffi::c_int = if scfsi < 0 as ::core::ffi::c_int
                {
                    ((1 as ::core::ffi::c_int) << bits) - 1 as ::core::ffi::c_int
                } else {
                    -(1 as ::core::ffi::c_int)
                };
                k = 0 as ::core::ffi::c_int;
                while k < cnt {
                    let mut s: ::core::ffi::c_int = get_bits(bitbuf, bits)
                        as ::core::ffi::c_int;
                    *ist_pos.offset(k as isize) = (if s == max_scf {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        s
                    }) as uint8_t;
                    *scf.offset(k as isize) = s as uint8_t;
                    k += 1;
                }
            }
        }
        ist_pos = ist_pos.offset(cnt as isize);
        scf = scf.offset(cnt as isize);
        i += 1;
        scfsi *= 2 as ::core::ffi::c_int;
    }
    let ref mut fresh34 = *scf.offset(2 as ::core::ffi::c_int as isize);
    *fresh34 = 0 as uint8_t;
    let ref mut fresh35 = *scf.offset(1 as ::core::ffi::c_int as isize);
    *fresh35 = *fresh34;
    *scf.offset(0 as ::core::ffi::c_int as isize) = *fresh35;
}
unsafe extern "C" fn L3_ldexp_q2(
    mut y: ::core::ffi::c_float,
    mut exp_q2: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    static mut g_expfrac: [::core::ffi::c_float; 4] = [
        9.31322575e-10f32,
        7.83145814e-10f32,
        6.58544508e-10f32,
        5.53767716e-10f32,
    ];
    let mut e: ::core::ffi::c_int = 0;
    loop {
        e = if 30 as ::core::ffi::c_int * 4 as ::core::ffi::c_int > exp_q2 {
            exp_q2
        } else {
            30 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
        };
        y
            *= g_expfrac[(e & 3 as ::core::ffi::c_int) as usize]
                * ((1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int
                    >> (e >> 2 as ::core::ffi::c_int)) as ::core::ffi::c_float;
        exp_q2 -= e;
        if !(exp_q2 > 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return y;
}
unsafe extern "C" fn L3_decode_scalefactors(
    mut hdr: *const uint8_t,
    mut ist_pos: *mut uint8_t,
    mut bs: *mut bs_t,
    mut gr: *const L3_gr_info_t,
    mut scf: *mut ::core::ffi::c_float,
    mut ch: ::core::ffi::c_int,
) {
    static mut g_scf_partitions: [[uint8_t; 28]; 3] = [
        [
            6 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            8 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            15 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            15 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            15 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    let mut scf_partition: *const uint8_t = &raw const *(&raw const g_scf_partitions
        as *const [uint8_t; 28])
        .offset(
            (((*gr).n_short_sfb != 0) as ::core::ffi::c_int
                + ((*gr).n_long_sfb == 0) as ::core::ffi::c_int) as isize,
        ) as *const uint8_t;
    let mut scf_size: [uint8_t; 4] = [0; 4];
    let mut iscf: [uint8_t; 40] = [0; 40];
    let mut i: ::core::ffi::c_int = 0;
    let mut scf_shift: ::core::ffi::c_int = (*gr).scalefac_scale as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int;
    let mut gain_exp: ::core::ffi::c_int = 0;
    let mut scfsi: ::core::ffi::c_int = (*gr).scfsi as ::core::ffi::c_int;
    let mut gain: ::core::ffi::c_float = 0.;
    if *hdr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 0x8 as ::core::ffi::c_int != 0
    {
        static mut g_scfc_decode: [uint8_t; 16] = [
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            15 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            19 as ::core::ffi::c_int as uint8_t,
        ];
        let mut part: ::core::ffi::c_int = g_scfc_decode[(*gr).scalefac_compress
            as usize] as ::core::ffi::c_int;
        scf_size[0 as ::core::ffi::c_int as usize] = (part >> 2 as ::core::ffi::c_int)
            as uint8_t;
        scf_size[1 as ::core::ffi::c_int as usize] = scf_size[0 as ::core::ffi::c_int
            as usize];
        scf_size[2 as ::core::ffi::c_int as usize] = (part & 3 as ::core::ffi::c_int)
            as uint8_t;
        scf_size[3 as ::core::ffi::c_int as usize] = scf_size[2 as ::core::ffi::c_int
            as usize];
    } else {
        static mut g_mod: [uint8_t; 24] = [
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
        ];
        let mut k: ::core::ffi::c_int = 0;
        let mut modprod: ::core::ffi::c_int = 0;
        let mut sfc: ::core::ffi::c_int = 0;
        let mut ist: ::core::ffi::c_int = (*hdr.offset(3 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int != 0 && ch != 0)
            as ::core::ffi::c_int;
        sfc = (*gr).scalefac_compress as ::core::ffi::c_int >> ist;
        k = ist * 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
        while sfc >= 0 as ::core::ffi::c_int {
            modprod = 1 as ::core::ffi::c_int;
            i = 3 as ::core::ffi::c_int;
            while i >= 0 as ::core::ffi::c_int {
                scf_size[i as usize] = (sfc / modprod
                    % g_mod[(k + i) as usize] as ::core::ffi::c_int) as uint8_t;
                modprod *= g_mod[(k + i) as usize] as ::core::ffi::c_int;
                i -= 1;
            }
            sfc -= modprod;
            k += 4 as ::core::ffi::c_int;
        }
        scf_partition = scf_partition.offset(k as isize);
        scfsi = -(16 as ::core::ffi::c_int);
    }
    L3_read_scalefactors(
        &raw mut iscf as *mut uint8_t,
        ist_pos,
        &raw mut scf_size as *mut uint8_t,
        scf_partition,
        bs,
        scfsi,
    );
    if (*gr).n_short_sfb != 0 {
        let mut sh: ::core::ffi::c_int = 3 as ::core::ffi::c_int - scf_shift;
        i = 0 as ::core::ffi::c_int;
        while i < (*gr).n_short_sfb as ::core::ffi::c_int {
            iscf[((*gr).n_long_sfb as ::core::ffi::c_int + i + 0 as ::core::ffi::c_int)
                as usize] = (iscf[((*gr).n_long_sfb as ::core::ffi::c_int + i
                + 0 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                + (((*gr).subblock_gain[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int) << sh)) as uint8_t;
            iscf[((*gr).n_long_sfb as ::core::ffi::c_int + i + 1 as ::core::ffi::c_int)
                as usize] = (iscf[((*gr).n_long_sfb as ::core::ffi::c_int + i
                + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                + (((*gr).subblock_gain[1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int) << sh)) as uint8_t;
            iscf[((*gr).n_long_sfb as ::core::ffi::c_int + i + 2 as ::core::ffi::c_int)
                as usize] = (iscf[((*gr).n_long_sfb as ::core::ffi::c_int + i
                + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                + (((*gr).subblock_gain[2 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int) << sh)) as uint8_t;
            i += 3 as ::core::ffi::c_int;
        }
    } else if (*gr).preflag != 0 {
        static mut g_preamp: [uint8_t; 10] = [
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
        ];
        i = 0 as ::core::ffi::c_int;
        while i < 10 as ::core::ffi::c_int {
            iscf[(11 as ::core::ffi::c_int + i) as usize] = (iscf[(11
                as ::core::ffi::c_int + i) as usize] as ::core::ffi::c_int
                + g_preamp[i as usize] as ::core::ffi::c_int) as uint8_t;
            i += 1;
        }
    }
    gain_exp = (*gr).global_gain as ::core::ffi::c_int
        + BITS_DEQUANTIZER_OUT * 4 as ::core::ffi::c_int - 210 as ::core::ffi::c_int
        - (if *hdr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xe0 as ::core::ffi::c_int == 0x60 as ::core::ffi::c_int
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        });
    gain = L3_ldexp_q2(
        ((1 as ::core::ffi::c_int) << MAX_SCFI / 4 as ::core::ffi::c_int)
            as ::core::ffi::c_float,
        MAX_SCFI - gain_exp,
    );
    i = 0 as ::core::ffi::c_int;
    while i
        < (*gr).n_long_sfb as ::core::ffi::c_int
            + (*gr).n_short_sfb as ::core::ffi::c_int
    {
        *scf.offset(i as isize) = L3_ldexp_q2(
            gain,
            (iscf[i as usize] as ::core::ffi::c_int) << scf_shift,
        );
        i += 1;
    }
}
static mut g_pow43: [::core::ffi::c_float; 145] = [
    0 as ::core::ffi::c_int as ::core::ffi::c_float,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
    -2.519842f32,
    -4.326749f32,
    -6.349604f32,
    -8.549880f32,
    -10.902724f32,
    -13.390518f32,
    -16.000000f32,
    -18.720754f32,
    -21.544347f32,
    -24.463781f32,
    -27.473142f32,
    -30.567351f32,
    -33.741992f32,
    -36.993181f32,
    0 as ::core::ffi::c_int as ::core::ffi::c_float,
    1 as ::core::ffi::c_int as ::core::ffi::c_float,
    2.519842f32,
    4.326749f32,
    6.349604f32,
    8.549880f32,
    10.902724f32,
    13.390518f32,
    16.000000f32,
    18.720754f32,
    21.544347f32,
    24.463781f32,
    27.473142f32,
    30.567351f32,
    33.741992f32,
    36.993181f32,
    40.317474f32,
    43.711787f32,
    47.173345f32,
    50.699631f32,
    54.288352f32,
    57.937408f32,
    61.644865f32,
    65.408941f32,
    69.227979f32,
    73.100443f32,
    77.024898f32,
    81.000000f32,
    85.024491f32,
    89.097188f32,
    93.216975f32,
    97.382800f32,
    101.593667f32,
    105.848633f32,
    110.146801f32,
    114.487321f32,
    118.869381f32,
    123.292209f32,
    127.755065f32,
    132.257246f32,
    136.798076f32,
    141.376907f32,
    145.993119f32,
    150.646117f32,
    155.335327f32,
    160.060199f32,
    164.820202f32,
    169.614826f32,
    174.443577f32,
    179.305980f32,
    184.201575f32,
    189.129918f32,
    194.090580f32,
    199.083145f32,
    204.107210f32,
    209.162385f32,
    214.248292f32,
    219.364564f32,
    224.510845f32,
    229.686789f32,
    234.892058f32,
    240.126328f32,
    245.389280f32,
    250.680604f32,
    256.000000f32,
    261.347174f32,
    266.721841f32,
    272.123723f32,
    277.552547f32,
    283.008049f32,
    288.489971f32,
    293.998060f32,
    299.532071f32,
    305.091761f32,
    310.676898f32,
    316.287249f32,
    321.922592f32,
    327.582707f32,
    333.267377f32,
    338.976394f32,
    344.709550f32,
    350.466646f32,
    356.247482f32,
    362.051866f32,
    367.879608f32,
    373.730522f32,
    379.604427f32,
    385.501143f32,
    391.420496f32,
    397.362314f32,
    403.326427f32,
    409.312672f32,
    415.320884f32,
    421.350905f32,
    427.402579f32,
    433.475750f32,
    439.570269f32,
    445.685987f32,
    451.822757f32,
    457.980436f32,
    464.158883f32,
    470.357960f32,
    476.577530f32,
    482.817459f32,
    489.077615f32,
    495.357868f32,
    501.658090f32,
    507.978156f32,
    514.317941f32,
    520.677324f32,
    527.056184f32,
    533.454404f32,
    539.871867f32,
    546.308458f32,
    552.764065f32,
    559.238575f32,
    565.731879f32,
    572.243870f32,
    578.774440f32,
    585.323483f32,
    591.890898f32,
    598.476581f32,
    605.080431f32,
    611.702349f32,
    618.342238f32,
    625.000000f32,
    631.675540f32,
    638.368763f32,
    645.079578f32,
];
unsafe extern "C" fn L3_pow_43(mut x: ::core::ffi::c_int) -> ::core::ffi::c_float {
    let mut frac: ::core::ffi::c_float = 0.;
    let mut sign: ::core::ffi::c_int = 0;
    let mut mult: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
    if x < 129 as ::core::ffi::c_int {
        return g_pow43[(16 as ::core::ffi::c_int + x) as usize];
    }
    if x < 1024 as ::core::ffi::c_int {
        mult = 16 as ::core::ffi::c_int;
        x <<= 3 as ::core::ffi::c_int;
    }
    sign = 2 as ::core::ffi::c_int * x & 64 as ::core::ffi::c_int;
    frac = ((x & 63 as ::core::ffi::c_int) - sign) as ::core::ffi::c_float
        / ((x & !(63 as ::core::ffi::c_int)) + sign) as ::core::ffi::c_float;
    return g_pow43[(16 as ::core::ffi::c_int + (x + sign >> 6 as ::core::ffi::c_int))
        as usize]
        * (1.0f32
            + frac
                * (4.0f32 / 3 as ::core::ffi::c_int as ::core::ffi::c_float
                    + frac * (2.0f32 / 9 as ::core::ffi::c_int as ::core::ffi::c_float)))
        * mult as ::core::ffi::c_float;
}
unsafe extern "C" fn L3_huffman(
    mut dst: *mut ::core::ffi::c_float,
    mut bs: *mut bs_t,
    mut gr_info: *const L3_gr_info_t,
    mut scf: *const ::core::ffi::c_float,
    mut layer3gr_limit: ::core::ffi::c_int,
) {
    static mut tabs: [int16_t; 2164] = [
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        513 as ::core::ffi::c_int as int16_t,
        513 as ::core::ffi::c_int as int16_t,
        513 as ::core::ffi::c_int as int16_t,
        513 as ::core::ffi::c_int as int16_t,
        513 as ::core::ffi::c_int as int16_t,
        513 as ::core::ffi::c_int as int16_t,
        513 as ::core::ffi::c_int as int16_t,
        513 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        -(255 as ::core::ffi::c_int) as int16_t,
        1313 as ::core::ffi::c_int as int16_t,
        1298 as ::core::ffi::c_int as int16_t,
        1282 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        290 as ::core::ffi::c_int as int16_t,
        288 as ::core::ffi::c_int as int16_t,
        -(255 as ::core::ffi::c_int) as int16_t,
        1313 as ::core::ffi::c_int as int16_t,
        1298 as ::core::ffi::c_int as int16_t,
        1282 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        528 as ::core::ffi::c_int as int16_t,
        528 as ::core::ffi::c_int as int16_t,
        528 as ::core::ffi::c_int as int16_t,
        528 as ::core::ffi::c_int as int16_t,
        528 as ::core::ffi::c_int as int16_t,
        528 as ::core::ffi::c_int as int16_t,
        528 as ::core::ffi::c_int as int16_t,
        528 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        290 as ::core::ffi::c_int as int16_t,
        288 as ::core::ffi::c_int as int16_t,
        -(253 as ::core::ffi::c_int) as int16_t,
        -(318 as ::core::ffi::c_int) as int16_t,
        -(351 as ::core::ffi::c_int) as int16_t,
        -(367 as ::core::ffi::c_int) as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        819 as ::core::ffi::c_int as int16_t,
        818 as ::core::ffi::c_int as int16_t,
        547 as ::core::ffi::c_int as int16_t,
        547 as ::core::ffi::c_int as int16_t,
        275 as ::core::ffi::c_int as int16_t,
        275 as ::core::ffi::c_int as int16_t,
        275 as ::core::ffi::c_int as int16_t,
        275 as ::core::ffi::c_int as int16_t,
        561 as ::core::ffi::c_int as int16_t,
        560 as ::core::ffi::c_int as int16_t,
        515 as ::core::ffi::c_int as int16_t,
        546 as ::core::ffi::c_int as int16_t,
        289 as ::core::ffi::c_int as int16_t,
        274 as ::core::ffi::c_int as int16_t,
        288 as ::core::ffi::c_int as int16_t,
        258 as ::core::ffi::c_int as int16_t,
        -(254 as ::core::ffi::c_int) as int16_t,
        -(287 as ::core::ffi::c_int) as int16_t,
        1329 as ::core::ffi::c_int as int16_t,
        1299 as ::core::ffi::c_int as int16_t,
        1314 as ::core::ffi::c_int as int16_t,
        1312 as ::core::ffi::c_int as int16_t,
        1057 as ::core::ffi::c_int as int16_t,
        1057 as ::core::ffi::c_int as int16_t,
        1042 as ::core::ffi::c_int as int16_t,
        1042 as ::core::ffi::c_int as int16_t,
        1026 as ::core::ffi::c_int as int16_t,
        1026 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        563 as ::core::ffi::c_int as int16_t,
        560 as ::core::ffi::c_int as int16_t,
        306 as ::core::ffi::c_int as int16_t,
        306 as ::core::ffi::c_int as int16_t,
        291 as ::core::ffi::c_int as int16_t,
        259 as ::core::ffi::c_int as int16_t,
        -(252 as ::core::ffi::c_int) as int16_t,
        -(413 as ::core::ffi::c_int) as int16_t,
        -(477 as ::core::ffi::c_int) as int16_t,
        -(542 as ::core::ffi::c_int) as int16_t,
        1298 as ::core::ffi::c_int as int16_t,
        -(575 as ::core::ffi::c_int) as int16_t,
        1041 as ::core::ffi::c_int as int16_t,
        1041 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        -(383 as ::core::ffi::c_int) as int16_t,
        -(399 as ::core::ffi::c_int) as int16_t,
        1107 as ::core::ffi::c_int as int16_t,
        1092 as ::core::ffi::c_int as int16_t,
        1106 as ::core::ffi::c_int as int16_t,
        1061 as ::core::ffi::c_int as int16_t,
        849 as ::core::ffi::c_int as int16_t,
        849 as ::core::ffi::c_int as int16_t,
        789 as ::core::ffi::c_int as int16_t,
        789 as ::core::ffi::c_int as int16_t,
        1104 as ::core::ffi::c_int as int16_t,
        1091 as ::core::ffi::c_int as int16_t,
        773 as ::core::ffi::c_int as int16_t,
        773 as ::core::ffi::c_int as int16_t,
        1076 as ::core::ffi::c_int as int16_t,
        1075 as ::core::ffi::c_int as int16_t,
        341 as ::core::ffi::c_int as int16_t,
        340 as ::core::ffi::c_int as int16_t,
        325 as ::core::ffi::c_int as int16_t,
        309 as ::core::ffi::c_int as int16_t,
        834 as ::core::ffi::c_int as int16_t,
        804 as ::core::ffi::c_int as int16_t,
        577 as ::core::ffi::c_int as int16_t,
        577 as ::core::ffi::c_int as int16_t,
        532 as ::core::ffi::c_int as int16_t,
        532 as ::core::ffi::c_int as int16_t,
        516 as ::core::ffi::c_int as int16_t,
        516 as ::core::ffi::c_int as int16_t,
        832 as ::core::ffi::c_int as int16_t,
        818 as ::core::ffi::c_int as int16_t,
        803 as ::core::ffi::c_int as int16_t,
        816 as ::core::ffi::c_int as int16_t,
        561 as ::core::ffi::c_int as int16_t,
        561 as ::core::ffi::c_int as int16_t,
        531 as ::core::ffi::c_int as int16_t,
        531 as ::core::ffi::c_int as int16_t,
        515 as ::core::ffi::c_int as int16_t,
        546 as ::core::ffi::c_int as int16_t,
        289 as ::core::ffi::c_int as int16_t,
        289 as ::core::ffi::c_int as int16_t,
        288 as ::core::ffi::c_int as int16_t,
        258 as ::core::ffi::c_int as int16_t,
        -(252 as ::core::ffi::c_int) as int16_t,
        -(429 as ::core::ffi::c_int) as int16_t,
        -(493 as ::core::ffi::c_int) as int16_t,
        -(559 as ::core::ffi::c_int) as int16_t,
        1057 as ::core::ffi::c_int as int16_t,
        1057 as ::core::ffi::c_int as int16_t,
        1042 as ::core::ffi::c_int as int16_t,
        1042 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        529 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        -(382 as ::core::ffi::c_int) as int16_t,
        1077 as ::core::ffi::c_int as int16_t,
        -(415 as ::core::ffi::c_int) as int16_t,
        1106 as ::core::ffi::c_int as int16_t,
        1061 as ::core::ffi::c_int as int16_t,
        1104 as ::core::ffi::c_int as int16_t,
        849 as ::core::ffi::c_int as int16_t,
        849 as ::core::ffi::c_int as int16_t,
        789 as ::core::ffi::c_int as int16_t,
        789 as ::core::ffi::c_int as int16_t,
        1091 as ::core::ffi::c_int as int16_t,
        1076 as ::core::ffi::c_int as int16_t,
        1029 as ::core::ffi::c_int as int16_t,
        1075 as ::core::ffi::c_int as int16_t,
        834 as ::core::ffi::c_int as int16_t,
        834 as ::core::ffi::c_int as int16_t,
        597 as ::core::ffi::c_int as int16_t,
        581 as ::core::ffi::c_int as int16_t,
        340 as ::core::ffi::c_int as int16_t,
        340 as ::core::ffi::c_int as int16_t,
        339 as ::core::ffi::c_int as int16_t,
        324 as ::core::ffi::c_int as int16_t,
        804 as ::core::ffi::c_int as int16_t,
        833 as ::core::ffi::c_int as int16_t,
        532 as ::core::ffi::c_int as int16_t,
        532 as ::core::ffi::c_int as int16_t,
        832 as ::core::ffi::c_int as int16_t,
        772 as ::core::ffi::c_int as int16_t,
        818 as ::core::ffi::c_int as int16_t,
        803 as ::core::ffi::c_int as int16_t,
        817 as ::core::ffi::c_int as int16_t,
        787 as ::core::ffi::c_int as int16_t,
        816 as ::core::ffi::c_int as int16_t,
        771 as ::core::ffi::c_int as int16_t,
        290 as ::core::ffi::c_int as int16_t,
        290 as ::core::ffi::c_int as int16_t,
        290 as ::core::ffi::c_int as int16_t,
        290 as ::core::ffi::c_int as int16_t,
        288 as ::core::ffi::c_int as int16_t,
        258 as ::core::ffi::c_int as int16_t,
        -(253 as ::core::ffi::c_int) as int16_t,
        -(349 as ::core::ffi::c_int) as int16_t,
        -(414 as ::core::ffi::c_int) as int16_t,
        -(447 as ::core::ffi::c_int) as int16_t,
        -(463 as ::core::ffi::c_int) as int16_t,
        1329 as ::core::ffi::c_int as int16_t,
        1299 as ::core::ffi::c_int as int16_t,
        -(479 as ::core::ffi::c_int) as int16_t,
        1314 as ::core::ffi::c_int as int16_t,
        1312 as ::core::ffi::c_int as int16_t,
        1057 as ::core::ffi::c_int as int16_t,
        1057 as ::core::ffi::c_int as int16_t,
        1042 as ::core::ffi::c_int as int16_t,
        1042 as ::core::ffi::c_int as int16_t,
        1026 as ::core::ffi::c_int as int16_t,
        1026 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        -(319 as ::core::ffi::c_int) as int16_t,
        851 as ::core::ffi::c_int as int16_t,
        821 as ::core::ffi::c_int as int16_t,
        -(335 as ::core::ffi::c_int) as int16_t,
        836 as ::core::ffi::c_int as int16_t,
        850 as ::core::ffi::c_int as int16_t,
        805 as ::core::ffi::c_int as int16_t,
        849 as ::core::ffi::c_int as int16_t,
        341 as ::core::ffi::c_int as int16_t,
        340 as ::core::ffi::c_int as int16_t,
        325 as ::core::ffi::c_int as int16_t,
        336 as ::core::ffi::c_int as int16_t,
        533 as ::core::ffi::c_int as int16_t,
        533 as ::core::ffi::c_int as int16_t,
        579 as ::core::ffi::c_int as int16_t,
        579 as ::core::ffi::c_int as int16_t,
        564 as ::core::ffi::c_int as int16_t,
        564 as ::core::ffi::c_int as int16_t,
        773 as ::core::ffi::c_int as int16_t,
        832 as ::core::ffi::c_int as int16_t,
        578 as ::core::ffi::c_int as int16_t,
        548 as ::core::ffi::c_int as int16_t,
        563 as ::core::ffi::c_int as int16_t,
        516 as ::core::ffi::c_int as int16_t,
        321 as ::core::ffi::c_int as int16_t,
        276 as ::core::ffi::c_int as int16_t,
        306 as ::core::ffi::c_int as int16_t,
        291 as ::core::ffi::c_int as int16_t,
        304 as ::core::ffi::c_int as int16_t,
        259 as ::core::ffi::c_int as int16_t,
        -(251 as ::core::ffi::c_int) as int16_t,
        -(572 as ::core::ffi::c_int) as int16_t,
        -(733 as ::core::ffi::c_int) as int16_t,
        -(830 as ::core::ffi::c_int) as int16_t,
        -(863 as ::core::ffi::c_int) as int16_t,
        -(879 as ::core::ffi::c_int) as int16_t,
        1041 as ::core::ffi::c_int as int16_t,
        1041 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        -(511 as ::core::ffi::c_int) as int16_t,
        -(527 as ::core::ffi::c_int) as int16_t,
        -(543 as ::core::ffi::c_int) as int16_t,
        1396 as ::core::ffi::c_int as int16_t,
        1351 as ::core::ffi::c_int as int16_t,
        1381 as ::core::ffi::c_int as int16_t,
        1366 as ::core::ffi::c_int as int16_t,
        1395 as ::core::ffi::c_int as int16_t,
        1335 as ::core::ffi::c_int as int16_t,
        1380 as ::core::ffi::c_int as int16_t,
        -(559 as ::core::ffi::c_int) as int16_t,
        1334 as ::core::ffi::c_int as int16_t,
        1138 as ::core::ffi::c_int as int16_t,
        1138 as ::core::ffi::c_int as int16_t,
        1063 as ::core::ffi::c_int as int16_t,
        1063 as ::core::ffi::c_int as int16_t,
        1350 as ::core::ffi::c_int as int16_t,
        1392 as ::core::ffi::c_int as int16_t,
        1031 as ::core::ffi::c_int as int16_t,
        1031 as ::core::ffi::c_int as int16_t,
        1062 as ::core::ffi::c_int as int16_t,
        1062 as ::core::ffi::c_int as int16_t,
        1364 as ::core::ffi::c_int as int16_t,
        1363 as ::core::ffi::c_int as int16_t,
        1120 as ::core::ffi::c_int as int16_t,
        1120 as ::core::ffi::c_int as int16_t,
        1333 as ::core::ffi::c_int as int16_t,
        1348 as ::core::ffi::c_int as int16_t,
        881 as ::core::ffi::c_int as int16_t,
        881 as ::core::ffi::c_int as int16_t,
        881 as ::core::ffi::c_int as int16_t,
        881 as ::core::ffi::c_int as int16_t,
        375 as ::core::ffi::c_int as int16_t,
        374 as ::core::ffi::c_int as int16_t,
        359 as ::core::ffi::c_int as int16_t,
        373 as ::core::ffi::c_int as int16_t,
        343 as ::core::ffi::c_int as int16_t,
        358 as ::core::ffi::c_int as int16_t,
        341 as ::core::ffi::c_int as int16_t,
        325 as ::core::ffi::c_int as int16_t,
        791 as ::core::ffi::c_int as int16_t,
        791 as ::core::ffi::c_int as int16_t,
        1123 as ::core::ffi::c_int as int16_t,
        1122 as ::core::ffi::c_int as int16_t,
        -(703 as ::core::ffi::c_int) as int16_t,
        1105 as ::core::ffi::c_int as int16_t,
        1045 as ::core::ffi::c_int as int16_t,
        -(719 as ::core::ffi::c_int) as int16_t,
        865 as ::core::ffi::c_int as int16_t,
        865 as ::core::ffi::c_int as int16_t,
        790 as ::core::ffi::c_int as int16_t,
        790 as ::core::ffi::c_int as int16_t,
        774 as ::core::ffi::c_int as int16_t,
        774 as ::core::ffi::c_int as int16_t,
        1104 as ::core::ffi::c_int as int16_t,
        1029 as ::core::ffi::c_int as int16_t,
        338 as ::core::ffi::c_int as int16_t,
        293 as ::core::ffi::c_int as int16_t,
        323 as ::core::ffi::c_int as int16_t,
        308 as ::core::ffi::c_int as int16_t,
        -(799 as ::core::ffi::c_int) as int16_t,
        -(815 as ::core::ffi::c_int) as int16_t,
        833 as ::core::ffi::c_int as int16_t,
        788 as ::core::ffi::c_int as int16_t,
        772 as ::core::ffi::c_int as int16_t,
        818 as ::core::ffi::c_int as int16_t,
        803 as ::core::ffi::c_int as int16_t,
        816 as ::core::ffi::c_int as int16_t,
        322 as ::core::ffi::c_int as int16_t,
        292 as ::core::ffi::c_int as int16_t,
        307 as ::core::ffi::c_int as int16_t,
        320 as ::core::ffi::c_int as int16_t,
        561 as ::core::ffi::c_int as int16_t,
        531 as ::core::ffi::c_int as int16_t,
        515 as ::core::ffi::c_int as int16_t,
        546 as ::core::ffi::c_int as int16_t,
        289 as ::core::ffi::c_int as int16_t,
        274 as ::core::ffi::c_int as int16_t,
        288 as ::core::ffi::c_int as int16_t,
        258 as ::core::ffi::c_int as int16_t,
        -(251 as ::core::ffi::c_int) as int16_t,
        -(525 as ::core::ffi::c_int) as int16_t,
        -(605 as ::core::ffi::c_int) as int16_t,
        -(685 as ::core::ffi::c_int) as int16_t,
        -(765 as ::core::ffi::c_int) as int16_t,
        -(831 as ::core::ffi::c_int) as int16_t,
        -(846 as ::core::ffi::c_int) as int16_t,
        1298 as ::core::ffi::c_int as int16_t,
        1057 as ::core::ffi::c_int as int16_t,
        1057 as ::core::ffi::c_int as int16_t,
        1312 as ::core::ffi::c_int as int16_t,
        1282 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        512 as ::core::ffi::c_int as int16_t,
        1399 as ::core::ffi::c_int as int16_t,
        1398 as ::core::ffi::c_int as int16_t,
        1383 as ::core::ffi::c_int as int16_t,
        1367 as ::core::ffi::c_int as int16_t,
        1382 as ::core::ffi::c_int as int16_t,
        1396 as ::core::ffi::c_int as int16_t,
        1351 as ::core::ffi::c_int as int16_t,
        -(511 as ::core::ffi::c_int) as int16_t,
        1381 as ::core::ffi::c_int as int16_t,
        1366 as ::core::ffi::c_int as int16_t,
        1139 as ::core::ffi::c_int as int16_t,
        1139 as ::core::ffi::c_int as int16_t,
        1079 as ::core::ffi::c_int as int16_t,
        1079 as ::core::ffi::c_int as int16_t,
        1124 as ::core::ffi::c_int as int16_t,
        1124 as ::core::ffi::c_int as int16_t,
        1364 as ::core::ffi::c_int as int16_t,
        1349 as ::core::ffi::c_int as int16_t,
        1363 as ::core::ffi::c_int as int16_t,
        1333 as ::core::ffi::c_int as int16_t,
        882 as ::core::ffi::c_int as int16_t,
        882 as ::core::ffi::c_int as int16_t,
        882 as ::core::ffi::c_int as int16_t,
        882 as ::core::ffi::c_int as int16_t,
        807 as ::core::ffi::c_int as int16_t,
        807 as ::core::ffi::c_int as int16_t,
        807 as ::core::ffi::c_int as int16_t,
        807 as ::core::ffi::c_int as int16_t,
        1094 as ::core::ffi::c_int as int16_t,
        1094 as ::core::ffi::c_int as int16_t,
        1136 as ::core::ffi::c_int as int16_t,
        1136 as ::core::ffi::c_int as int16_t,
        373 as ::core::ffi::c_int as int16_t,
        341 as ::core::ffi::c_int as int16_t,
        535 as ::core::ffi::c_int as int16_t,
        535 as ::core::ffi::c_int as int16_t,
        881 as ::core::ffi::c_int as int16_t,
        775 as ::core::ffi::c_int as int16_t,
        867 as ::core::ffi::c_int as int16_t,
        822 as ::core::ffi::c_int as int16_t,
        774 as ::core::ffi::c_int as int16_t,
        -(591 as ::core::ffi::c_int) as int16_t,
        324 as ::core::ffi::c_int as int16_t,
        338 as ::core::ffi::c_int as int16_t,
        -(671 as ::core::ffi::c_int) as int16_t,
        849 as ::core::ffi::c_int as int16_t,
        550 as ::core::ffi::c_int as int16_t,
        550 as ::core::ffi::c_int as int16_t,
        866 as ::core::ffi::c_int as int16_t,
        864 as ::core::ffi::c_int as int16_t,
        609 as ::core::ffi::c_int as int16_t,
        609 as ::core::ffi::c_int as int16_t,
        293 as ::core::ffi::c_int as int16_t,
        336 as ::core::ffi::c_int as int16_t,
        534 as ::core::ffi::c_int as int16_t,
        534 as ::core::ffi::c_int as int16_t,
        789 as ::core::ffi::c_int as int16_t,
        835 as ::core::ffi::c_int as int16_t,
        773 as ::core::ffi::c_int as int16_t,
        -(751 as ::core::ffi::c_int) as int16_t,
        834 as ::core::ffi::c_int as int16_t,
        804 as ::core::ffi::c_int as int16_t,
        308 as ::core::ffi::c_int as int16_t,
        307 as ::core::ffi::c_int as int16_t,
        833 as ::core::ffi::c_int as int16_t,
        788 as ::core::ffi::c_int as int16_t,
        832 as ::core::ffi::c_int as int16_t,
        772 as ::core::ffi::c_int as int16_t,
        562 as ::core::ffi::c_int as int16_t,
        562 as ::core::ffi::c_int as int16_t,
        547 as ::core::ffi::c_int as int16_t,
        547 as ::core::ffi::c_int as int16_t,
        305 as ::core::ffi::c_int as int16_t,
        275 as ::core::ffi::c_int as int16_t,
        560 as ::core::ffi::c_int as int16_t,
        515 as ::core::ffi::c_int as int16_t,
        290 as ::core::ffi::c_int as int16_t,
        290 as ::core::ffi::c_int as int16_t,
        -(252 as ::core::ffi::c_int) as int16_t,
        -(397 as ::core::ffi::c_int) as int16_t,
        -(477 as ::core::ffi::c_int) as int16_t,
        -(557 as ::core::ffi::c_int) as int16_t,
        -(622 as ::core::ffi::c_int) as int16_t,
        -(653 as ::core::ffi::c_int) as int16_t,
        -(719 as ::core::ffi::c_int) as int16_t,
        -(735 as ::core::ffi::c_int) as int16_t,
        -(750 as ::core::ffi::c_int) as int16_t,
        1329 as ::core::ffi::c_int as int16_t,
        1299 as ::core::ffi::c_int as int16_t,
        1314 as ::core::ffi::c_int as int16_t,
        1057 as ::core::ffi::c_int as int16_t,
        1057 as ::core::ffi::c_int as int16_t,
        1042 as ::core::ffi::c_int as int16_t,
        1042 as ::core::ffi::c_int as int16_t,
        1312 as ::core::ffi::c_int as int16_t,
        1282 as ::core::ffi::c_int as int16_t,
        1024 as ::core::ffi::c_int as int16_t,
        1024 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        784 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        -(383 as ::core::ffi::c_int) as int16_t,
        1127 as ::core::ffi::c_int as int16_t,
        1141 as ::core::ffi::c_int as int16_t,
        1111 as ::core::ffi::c_int as int16_t,
        1126 as ::core::ffi::c_int as int16_t,
        1140 as ::core::ffi::c_int as int16_t,
        1095 as ::core::ffi::c_int as int16_t,
        1110 as ::core::ffi::c_int as int16_t,
        869 as ::core::ffi::c_int as int16_t,
        869 as ::core::ffi::c_int as int16_t,
        883 as ::core::ffi::c_int as int16_t,
        883 as ::core::ffi::c_int as int16_t,
        1079 as ::core::ffi::c_int as int16_t,
        1109 as ::core::ffi::c_int as int16_t,
        882 as ::core::ffi::c_int as int16_t,
        882 as ::core::ffi::c_int as int16_t,
        375 as ::core::ffi::c_int as int16_t,
        374 as ::core::ffi::c_int as int16_t,
        807 as ::core::ffi::c_int as int16_t,
        868 as ::core::ffi::c_int as int16_t,
        838 as ::core::ffi::c_int as int16_t,
        881 as ::core::ffi::c_int as int16_t,
        791 as ::core::ffi::c_int as int16_t,
        -(463 as ::core::ffi::c_int) as int16_t,
        867 as ::core::ffi::c_int as int16_t,
        822 as ::core::ffi::c_int as int16_t,
        368 as ::core::ffi::c_int as int16_t,
        263 as ::core::ffi::c_int as int16_t,
        852 as ::core::ffi::c_int as int16_t,
        837 as ::core::ffi::c_int as int16_t,
        836 as ::core::ffi::c_int as int16_t,
        -(543 as ::core::ffi::c_int) as int16_t,
        610 as ::core::ffi::c_int as int16_t,
        610 as ::core::ffi::c_int as int16_t,
        550 as ::core::ffi::c_int as int16_t,
        550 as ::core::ffi::c_int as int16_t,
        352 as ::core::ffi::c_int as int16_t,
        336 as ::core::ffi::c_int as int16_t,
        534 as ::core::ffi::c_int as int16_t,
        534 as ::core::ffi::c_int as int16_t,
        865 as ::core::ffi::c_int as int16_t,
        774 as ::core::ffi::c_int as int16_t,
        851 as ::core::ffi::c_int as int16_t,
        821 as ::core::ffi::c_int as int16_t,
        850 as ::core::ffi::c_int as int16_t,
        805 as ::core::ffi::c_int as int16_t,
        593 as ::core::ffi::c_int as int16_t,
        533 as ::core::ffi::c_int as int16_t,
        579 as ::core::ffi::c_int as int16_t,
        564 as ::core::ffi::c_int as int16_t,
        773 as ::core::ffi::c_int as int16_t,
        832 as ::core::ffi::c_int as int16_t,
        578 as ::core::ffi::c_int as int16_t,
        578 as ::core::ffi::c_int as int16_t,
        548 as ::core::ffi::c_int as int16_t,
        548 as ::core::ffi::c_int as int16_t,
        577 as ::core::ffi::c_int as int16_t,
        577 as ::core::ffi::c_int as int16_t,
        307 as ::core::ffi::c_int as int16_t,
        276 as ::core::ffi::c_int as int16_t,
        306 as ::core::ffi::c_int as int16_t,
        291 as ::core::ffi::c_int as int16_t,
        516 as ::core::ffi::c_int as int16_t,
        560 as ::core::ffi::c_int as int16_t,
        259 as ::core::ffi::c_int as int16_t,
        259 as ::core::ffi::c_int as int16_t,
        -(250 as ::core::ffi::c_int) as int16_t,
        -(2107 as ::core::ffi::c_int) as int16_t,
        -(2507 as ::core::ffi::c_int) as int16_t,
        -(2764 as ::core::ffi::c_int) as int16_t,
        -(2909 as ::core::ffi::c_int) as int16_t,
        -(2974 as ::core::ffi::c_int) as int16_t,
        -(3007 as ::core::ffi::c_int) as int16_t,
        -(3023 as ::core::ffi::c_int) as int16_t,
        1041 as ::core::ffi::c_int as int16_t,
        1041 as ::core::ffi::c_int as int16_t,
        1040 as ::core::ffi::c_int as int16_t,
        1040 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        -(767 as ::core::ffi::c_int) as int16_t,
        -(1052 as ::core::ffi::c_int) as int16_t,
        -(1213 as ::core::ffi::c_int) as int16_t,
        -(1277 as ::core::ffi::c_int) as int16_t,
        -(1358 as ::core::ffi::c_int) as int16_t,
        -(1405 as ::core::ffi::c_int) as int16_t,
        -(1469 as ::core::ffi::c_int) as int16_t,
        -(1535 as ::core::ffi::c_int) as int16_t,
        -(1550 as ::core::ffi::c_int) as int16_t,
        -(1582 as ::core::ffi::c_int) as int16_t,
        -(1614 as ::core::ffi::c_int) as int16_t,
        -(1647 as ::core::ffi::c_int) as int16_t,
        -(1662 as ::core::ffi::c_int) as int16_t,
        -(1694 as ::core::ffi::c_int) as int16_t,
        -(1726 as ::core::ffi::c_int) as int16_t,
        -(1759 as ::core::ffi::c_int) as int16_t,
        -(1774 as ::core::ffi::c_int) as int16_t,
        -(1807 as ::core::ffi::c_int) as int16_t,
        -(1822 as ::core::ffi::c_int) as int16_t,
        -(1854 as ::core::ffi::c_int) as int16_t,
        -(1886 as ::core::ffi::c_int) as int16_t,
        1565 as ::core::ffi::c_int as int16_t,
        -(1919 as ::core::ffi::c_int) as int16_t,
        -(1935 as ::core::ffi::c_int) as int16_t,
        -(1951 as ::core::ffi::c_int) as int16_t,
        -(1967 as ::core::ffi::c_int) as int16_t,
        1731 as ::core::ffi::c_int as int16_t,
        1730 as ::core::ffi::c_int as int16_t,
        1580 as ::core::ffi::c_int as int16_t,
        1717 as ::core::ffi::c_int as int16_t,
        -(1983 as ::core::ffi::c_int) as int16_t,
        1729 as ::core::ffi::c_int as int16_t,
        1564 as ::core::ffi::c_int as int16_t,
        -(1999 as ::core::ffi::c_int) as int16_t,
        1548 as ::core::ffi::c_int as int16_t,
        -(2015 as ::core::ffi::c_int) as int16_t,
        -(2031 as ::core::ffi::c_int) as int16_t,
        1715 as ::core::ffi::c_int as int16_t,
        1595 as ::core::ffi::c_int as int16_t,
        -(2047 as ::core::ffi::c_int) as int16_t,
        1714 as ::core::ffi::c_int as int16_t,
        -(2063 as ::core::ffi::c_int) as int16_t,
        1610 as ::core::ffi::c_int as int16_t,
        -(2079 as ::core::ffi::c_int) as int16_t,
        1609 as ::core::ffi::c_int as int16_t,
        -(2095 as ::core::ffi::c_int) as int16_t,
        1323 as ::core::ffi::c_int as int16_t,
        1323 as ::core::ffi::c_int as int16_t,
        1457 as ::core::ffi::c_int as int16_t,
        1457 as ::core::ffi::c_int as int16_t,
        1307 as ::core::ffi::c_int as int16_t,
        1307 as ::core::ffi::c_int as int16_t,
        1712 as ::core::ffi::c_int as int16_t,
        1547 as ::core::ffi::c_int as int16_t,
        1641 as ::core::ffi::c_int as int16_t,
        1700 as ::core::ffi::c_int as int16_t,
        1699 as ::core::ffi::c_int as int16_t,
        1594 as ::core::ffi::c_int as int16_t,
        1685 as ::core::ffi::c_int as int16_t,
        1625 as ::core::ffi::c_int as int16_t,
        1442 as ::core::ffi::c_int as int16_t,
        1442 as ::core::ffi::c_int as int16_t,
        1322 as ::core::ffi::c_int as int16_t,
        1322 as ::core::ffi::c_int as int16_t,
        -(780 as ::core::ffi::c_int) as int16_t,
        -(973 as ::core::ffi::c_int) as int16_t,
        -(910 as ::core::ffi::c_int) as int16_t,
        1279 as ::core::ffi::c_int as int16_t,
        1278 as ::core::ffi::c_int as int16_t,
        1277 as ::core::ffi::c_int as int16_t,
        1262 as ::core::ffi::c_int as int16_t,
        1276 as ::core::ffi::c_int as int16_t,
        1261 as ::core::ffi::c_int as int16_t,
        1275 as ::core::ffi::c_int as int16_t,
        1215 as ::core::ffi::c_int as int16_t,
        1260 as ::core::ffi::c_int as int16_t,
        1229 as ::core::ffi::c_int as int16_t,
        -(959 as ::core::ffi::c_int) as int16_t,
        974 as ::core::ffi::c_int as int16_t,
        974 as ::core::ffi::c_int as int16_t,
        989 as ::core::ffi::c_int as int16_t,
        989 as ::core::ffi::c_int as int16_t,
        -(943 as ::core::ffi::c_int) as int16_t,
        735 as ::core::ffi::c_int as int16_t,
        478 as ::core::ffi::c_int as int16_t,
        478 as ::core::ffi::c_int as int16_t,
        495 as ::core::ffi::c_int as int16_t,
        463 as ::core::ffi::c_int as int16_t,
        506 as ::core::ffi::c_int as int16_t,
        414 as ::core::ffi::c_int as int16_t,
        -(1039 as ::core::ffi::c_int) as int16_t,
        1003 as ::core::ffi::c_int as int16_t,
        958 as ::core::ffi::c_int as int16_t,
        1017 as ::core::ffi::c_int as int16_t,
        927 as ::core::ffi::c_int as int16_t,
        942 as ::core::ffi::c_int as int16_t,
        987 as ::core::ffi::c_int as int16_t,
        957 as ::core::ffi::c_int as int16_t,
        431 as ::core::ffi::c_int as int16_t,
        476 as ::core::ffi::c_int as int16_t,
        1272 as ::core::ffi::c_int as int16_t,
        1167 as ::core::ffi::c_int as int16_t,
        1228 as ::core::ffi::c_int as int16_t,
        -(1183 as ::core::ffi::c_int) as int16_t,
        1256 as ::core::ffi::c_int as int16_t,
        -(1199 as ::core::ffi::c_int) as int16_t,
        895 as ::core::ffi::c_int as int16_t,
        895 as ::core::ffi::c_int as int16_t,
        941 as ::core::ffi::c_int as int16_t,
        941 as ::core::ffi::c_int as int16_t,
        1242 as ::core::ffi::c_int as int16_t,
        1227 as ::core::ffi::c_int as int16_t,
        1212 as ::core::ffi::c_int as int16_t,
        1135 as ::core::ffi::c_int as int16_t,
        1014 as ::core::ffi::c_int as int16_t,
        1014 as ::core::ffi::c_int as int16_t,
        490 as ::core::ffi::c_int as int16_t,
        489 as ::core::ffi::c_int as int16_t,
        503 as ::core::ffi::c_int as int16_t,
        487 as ::core::ffi::c_int as int16_t,
        910 as ::core::ffi::c_int as int16_t,
        1013 as ::core::ffi::c_int as int16_t,
        985 as ::core::ffi::c_int as int16_t,
        925 as ::core::ffi::c_int as int16_t,
        863 as ::core::ffi::c_int as int16_t,
        894 as ::core::ffi::c_int as int16_t,
        970 as ::core::ffi::c_int as int16_t,
        955 as ::core::ffi::c_int as int16_t,
        1012 as ::core::ffi::c_int as int16_t,
        847 as ::core::ffi::c_int as int16_t,
        -(1343 as ::core::ffi::c_int) as int16_t,
        831 as ::core::ffi::c_int as int16_t,
        755 as ::core::ffi::c_int as int16_t,
        755 as ::core::ffi::c_int as int16_t,
        984 as ::core::ffi::c_int as int16_t,
        909 as ::core::ffi::c_int as int16_t,
        428 as ::core::ffi::c_int as int16_t,
        366 as ::core::ffi::c_int as int16_t,
        754 as ::core::ffi::c_int as int16_t,
        559 as ::core::ffi::c_int as int16_t,
        -(1391 as ::core::ffi::c_int) as int16_t,
        752 as ::core::ffi::c_int as int16_t,
        486 as ::core::ffi::c_int as int16_t,
        457 as ::core::ffi::c_int as int16_t,
        924 as ::core::ffi::c_int as int16_t,
        997 as ::core::ffi::c_int as int16_t,
        698 as ::core::ffi::c_int as int16_t,
        698 as ::core::ffi::c_int as int16_t,
        983 as ::core::ffi::c_int as int16_t,
        893 as ::core::ffi::c_int as int16_t,
        740 as ::core::ffi::c_int as int16_t,
        740 as ::core::ffi::c_int as int16_t,
        908 as ::core::ffi::c_int as int16_t,
        877 as ::core::ffi::c_int as int16_t,
        739 as ::core::ffi::c_int as int16_t,
        739 as ::core::ffi::c_int as int16_t,
        667 as ::core::ffi::c_int as int16_t,
        667 as ::core::ffi::c_int as int16_t,
        953 as ::core::ffi::c_int as int16_t,
        938 as ::core::ffi::c_int as int16_t,
        497 as ::core::ffi::c_int as int16_t,
        287 as ::core::ffi::c_int as int16_t,
        271 as ::core::ffi::c_int as int16_t,
        271 as ::core::ffi::c_int as int16_t,
        683 as ::core::ffi::c_int as int16_t,
        606 as ::core::ffi::c_int as int16_t,
        590 as ::core::ffi::c_int as int16_t,
        712 as ::core::ffi::c_int as int16_t,
        726 as ::core::ffi::c_int as int16_t,
        574 as ::core::ffi::c_int as int16_t,
        302 as ::core::ffi::c_int as int16_t,
        302 as ::core::ffi::c_int as int16_t,
        738 as ::core::ffi::c_int as int16_t,
        736 as ::core::ffi::c_int as int16_t,
        481 as ::core::ffi::c_int as int16_t,
        286 as ::core::ffi::c_int as int16_t,
        526 as ::core::ffi::c_int as int16_t,
        725 as ::core::ffi::c_int as int16_t,
        605 as ::core::ffi::c_int as int16_t,
        711 as ::core::ffi::c_int as int16_t,
        636 as ::core::ffi::c_int as int16_t,
        724 as ::core::ffi::c_int as int16_t,
        696 as ::core::ffi::c_int as int16_t,
        651 as ::core::ffi::c_int as int16_t,
        589 as ::core::ffi::c_int as int16_t,
        681 as ::core::ffi::c_int as int16_t,
        666 as ::core::ffi::c_int as int16_t,
        710 as ::core::ffi::c_int as int16_t,
        364 as ::core::ffi::c_int as int16_t,
        467 as ::core::ffi::c_int as int16_t,
        573 as ::core::ffi::c_int as int16_t,
        695 as ::core::ffi::c_int as int16_t,
        466 as ::core::ffi::c_int as int16_t,
        466 as ::core::ffi::c_int as int16_t,
        301 as ::core::ffi::c_int as int16_t,
        465 as ::core::ffi::c_int as int16_t,
        379 as ::core::ffi::c_int as int16_t,
        379 as ::core::ffi::c_int as int16_t,
        709 as ::core::ffi::c_int as int16_t,
        604 as ::core::ffi::c_int as int16_t,
        665 as ::core::ffi::c_int as int16_t,
        679 as ::core::ffi::c_int as int16_t,
        316 as ::core::ffi::c_int as int16_t,
        316 as ::core::ffi::c_int as int16_t,
        634 as ::core::ffi::c_int as int16_t,
        633 as ::core::ffi::c_int as int16_t,
        436 as ::core::ffi::c_int as int16_t,
        436 as ::core::ffi::c_int as int16_t,
        464 as ::core::ffi::c_int as int16_t,
        269 as ::core::ffi::c_int as int16_t,
        424 as ::core::ffi::c_int as int16_t,
        394 as ::core::ffi::c_int as int16_t,
        452 as ::core::ffi::c_int as int16_t,
        332 as ::core::ffi::c_int as int16_t,
        438 as ::core::ffi::c_int as int16_t,
        363 as ::core::ffi::c_int as int16_t,
        347 as ::core::ffi::c_int as int16_t,
        408 as ::core::ffi::c_int as int16_t,
        393 as ::core::ffi::c_int as int16_t,
        448 as ::core::ffi::c_int as int16_t,
        331 as ::core::ffi::c_int as int16_t,
        422 as ::core::ffi::c_int as int16_t,
        362 as ::core::ffi::c_int as int16_t,
        407 as ::core::ffi::c_int as int16_t,
        392 as ::core::ffi::c_int as int16_t,
        421 as ::core::ffi::c_int as int16_t,
        346 as ::core::ffi::c_int as int16_t,
        406 as ::core::ffi::c_int as int16_t,
        391 as ::core::ffi::c_int as int16_t,
        376 as ::core::ffi::c_int as int16_t,
        375 as ::core::ffi::c_int as int16_t,
        359 as ::core::ffi::c_int as int16_t,
        1441 as ::core::ffi::c_int as int16_t,
        1306 as ::core::ffi::c_int as int16_t,
        -(2367 as ::core::ffi::c_int) as int16_t,
        1290 as ::core::ffi::c_int as int16_t,
        -(2383 as ::core::ffi::c_int) as int16_t,
        1337 as ::core::ffi::c_int as int16_t,
        -(2399 as ::core::ffi::c_int) as int16_t,
        -(2415 as ::core::ffi::c_int) as int16_t,
        1426 as ::core::ffi::c_int as int16_t,
        1321 as ::core::ffi::c_int as int16_t,
        -(2431 as ::core::ffi::c_int) as int16_t,
        1411 as ::core::ffi::c_int as int16_t,
        1336 as ::core::ffi::c_int as int16_t,
        -(2447 as ::core::ffi::c_int) as int16_t,
        -(2463 as ::core::ffi::c_int) as int16_t,
        -(2479 as ::core::ffi::c_int) as int16_t,
        1169 as ::core::ffi::c_int as int16_t,
        1169 as ::core::ffi::c_int as int16_t,
        1049 as ::core::ffi::c_int as int16_t,
        1049 as ::core::ffi::c_int as int16_t,
        1424 as ::core::ffi::c_int as int16_t,
        1289 as ::core::ffi::c_int as int16_t,
        1412 as ::core::ffi::c_int as int16_t,
        1352 as ::core::ffi::c_int as int16_t,
        1319 as ::core::ffi::c_int as int16_t,
        -(2495 as ::core::ffi::c_int) as int16_t,
        1154 as ::core::ffi::c_int as int16_t,
        1154 as ::core::ffi::c_int as int16_t,
        1064 as ::core::ffi::c_int as int16_t,
        1064 as ::core::ffi::c_int as int16_t,
        1153 as ::core::ffi::c_int as int16_t,
        1153 as ::core::ffi::c_int as int16_t,
        416 as ::core::ffi::c_int as int16_t,
        390 as ::core::ffi::c_int as int16_t,
        360 as ::core::ffi::c_int as int16_t,
        404 as ::core::ffi::c_int as int16_t,
        403 as ::core::ffi::c_int as int16_t,
        389 as ::core::ffi::c_int as int16_t,
        344 as ::core::ffi::c_int as int16_t,
        374 as ::core::ffi::c_int as int16_t,
        373 as ::core::ffi::c_int as int16_t,
        343 as ::core::ffi::c_int as int16_t,
        358 as ::core::ffi::c_int as int16_t,
        372 as ::core::ffi::c_int as int16_t,
        327 as ::core::ffi::c_int as int16_t,
        357 as ::core::ffi::c_int as int16_t,
        342 as ::core::ffi::c_int as int16_t,
        311 as ::core::ffi::c_int as int16_t,
        356 as ::core::ffi::c_int as int16_t,
        326 as ::core::ffi::c_int as int16_t,
        1395 as ::core::ffi::c_int as int16_t,
        1394 as ::core::ffi::c_int as int16_t,
        1137 as ::core::ffi::c_int as int16_t,
        1137 as ::core::ffi::c_int as int16_t,
        1047 as ::core::ffi::c_int as int16_t,
        1047 as ::core::ffi::c_int as int16_t,
        1365 as ::core::ffi::c_int as int16_t,
        1392 as ::core::ffi::c_int as int16_t,
        1287 as ::core::ffi::c_int as int16_t,
        1379 as ::core::ffi::c_int as int16_t,
        1334 as ::core::ffi::c_int as int16_t,
        1364 as ::core::ffi::c_int as int16_t,
        1349 as ::core::ffi::c_int as int16_t,
        1378 as ::core::ffi::c_int as int16_t,
        1318 as ::core::ffi::c_int as int16_t,
        1363 as ::core::ffi::c_int as int16_t,
        792 as ::core::ffi::c_int as int16_t,
        792 as ::core::ffi::c_int as int16_t,
        792 as ::core::ffi::c_int as int16_t,
        792 as ::core::ffi::c_int as int16_t,
        1152 as ::core::ffi::c_int as int16_t,
        1152 as ::core::ffi::c_int as int16_t,
        1032 as ::core::ffi::c_int as int16_t,
        1032 as ::core::ffi::c_int as int16_t,
        1121 as ::core::ffi::c_int as int16_t,
        1121 as ::core::ffi::c_int as int16_t,
        1046 as ::core::ffi::c_int as int16_t,
        1046 as ::core::ffi::c_int as int16_t,
        1120 as ::core::ffi::c_int as int16_t,
        1120 as ::core::ffi::c_int as int16_t,
        1030 as ::core::ffi::c_int as int16_t,
        1030 as ::core::ffi::c_int as int16_t,
        -(2895 as ::core::ffi::c_int) as int16_t,
        1106 as ::core::ffi::c_int as int16_t,
        1061 as ::core::ffi::c_int as int16_t,
        1104 as ::core::ffi::c_int as int16_t,
        849 as ::core::ffi::c_int as int16_t,
        849 as ::core::ffi::c_int as int16_t,
        789 as ::core::ffi::c_int as int16_t,
        789 as ::core::ffi::c_int as int16_t,
        1091 as ::core::ffi::c_int as int16_t,
        1076 as ::core::ffi::c_int as int16_t,
        1029 as ::core::ffi::c_int as int16_t,
        1090 as ::core::ffi::c_int as int16_t,
        1060 as ::core::ffi::c_int as int16_t,
        1075 as ::core::ffi::c_int as int16_t,
        833 as ::core::ffi::c_int as int16_t,
        833 as ::core::ffi::c_int as int16_t,
        309 as ::core::ffi::c_int as int16_t,
        324 as ::core::ffi::c_int as int16_t,
        532 as ::core::ffi::c_int as int16_t,
        532 as ::core::ffi::c_int as int16_t,
        832 as ::core::ffi::c_int as int16_t,
        772 as ::core::ffi::c_int as int16_t,
        818 as ::core::ffi::c_int as int16_t,
        803 as ::core::ffi::c_int as int16_t,
        561 as ::core::ffi::c_int as int16_t,
        561 as ::core::ffi::c_int as int16_t,
        531 as ::core::ffi::c_int as int16_t,
        560 as ::core::ffi::c_int as int16_t,
        515 as ::core::ffi::c_int as int16_t,
        546 as ::core::ffi::c_int as int16_t,
        289 as ::core::ffi::c_int as int16_t,
        274 as ::core::ffi::c_int as int16_t,
        288 as ::core::ffi::c_int as int16_t,
        258 as ::core::ffi::c_int as int16_t,
        -(250 as ::core::ffi::c_int) as int16_t,
        -(1179 as ::core::ffi::c_int) as int16_t,
        -(1579 as ::core::ffi::c_int) as int16_t,
        -(1836 as ::core::ffi::c_int) as int16_t,
        -(1996 as ::core::ffi::c_int) as int16_t,
        -(2124 as ::core::ffi::c_int) as int16_t,
        -(2253 as ::core::ffi::c_int) as int16_t,
        -(2333 as ::core::ffi::c_int) as int16_t,
        -(2413 as ::core::ffi::c_int) as int16_t,
        -(2477 as ::core::ffi::c_int) as int16_t,
        -(2542 as ::core::ffi::c_int) as int16_t,
        -(2574 as ::core::ffi::c_int) as int16_t,
        -(2607 as ::core::ffi::c_int) as int16_t,
        -(2622 as ::core::ffi::c_int) as int16_t,
        -(2655 as ::core::ffi::c_int) as int16_t,
        1314 as ::core::ffi::c_int as int16_t,
        1313 as ::core::ffi::c_int as int16_t,
        1298 as ::core::ffi::c_int as int16_t,
        1312 as ::core::ffi::c_int as int16_t,
        1282 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        785 as ::core::ffi::c_int as int16_t,
        1040 as ::core::ffi::c_int as int16_t,
        1040 as ::core::ffi::c_int as int16_t,
        1025 as ::core::ffi::c_int as int16_t,
        1025 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        768 as ::core::ffi::c_int as int16_t,
        -(766 as ::core::ffi::c_int) as int16_t,
        -(798 as ::core::ffi::c_int) as int16_t,
        -(830 as ::core::ffi::c_int) as int16_t,
        -(862 as ::core::ffi::c_int) as int16_t,
        -(895 as ::core::ffi::c_int) as int16_t,
        -(911 as ::core::ffi::c_int) as int16_t,
        -(927 as ::core::ffi::c_int) as int16_t,
        -(943 as ::core::ffi::c_int) as int16_t,
        -(959 as ::core::ffi::c_int) as int16_t,
        -(975 as ::core::ffi::c_int) as int16_t,
        -(991 as ::core::ffi::c_int) as int16_t,
        -(1007 as ::core::ffi::c_int) as int16_t,
        -(1023 as ::core::ffi::c_int) as int16_t,
        -(1039 as ::core::ffi::c_int) as int16_t,
        -(1055 as ::core::ffi::c_int) as int16_t,
        -(1070 as ::core::ffi::c_int) as int16_t,
        1724 as ::core::ffi::c_int as int16_t,
        1647 as ::core::ffi::c_int as int16_t,
        -(1103 as ::core::ffi::c_int) as int16_t,
        -(1119 as ::core::ffi::c_int) as int16_t,
        1631 as ::core::ffi::c_int as int16_t,
        1767 as ::core::ffi::c_int as int16_t,
        1662 as ::core::ffi::c_int as int16_t,
        1738 as ::core::ffi::c_int as int16_t,
        1708 as ::core::ffi::c_int as int16_t,
        1723 as ::core::ffi::c_int as int16_t,
        -(1135 as ::core::ffi::c_int) as int16_t,
        1780 as ::core::ffi::c_int as int16_t,
        1615 as ::core::ffi::c_int as int16_t,
        1779 as ::core::ffi::c_int as int16_t,
        1599 as ::core::ffi::c_int as int16_t,
        1677 as ::core::ffi::c_int as int16_t,
        1646 as ::core::ffi::c_int as int16_t,
        1778 as ::core::ffi::c_int as int16_t,
        1583 as ::core::ffi::c_int as int16_t,
        -(1151 as ::core::ffi::c_int) as int16_t,
        1777 as ::core::ffi::c_int as int16_t,
        1567 as ::core::ffi::c_int as int16_t,
        1737 as ::core::ffi::c_int as int16_t,
        1692 as ::core::ffi::c_int as int16_t,
        1765 as ::core::ffi::c_int as int16_t,
        1722 as ::core::ffi::c_int as int16_t,
        1707 as ::core::ffi::c_int as int16_t,
        1630 as ::core::ffi::c_int as int16_t,
        1751 as ::core::ffi::c_int as int16_t,
        1661 as ::core::ffi::c_int as int16_t,
        1764 as ::core::ffi::c_int as int16_t,
        1614 as ::core::ffi::c_int as int16_t,
        1736 as ::core::ffi::c_int as int16_t,
        1676 as ::core::ffi::c_int as int16_t,
        1763 as ::core::ffi::c_int as int16_t,
        1750 as ::core::ffi::c_int as int16_t,
        1645 as ::core::ffi::c_int as int16_t,
        1598 as ::core::ffi::c_int as int16_t,
        1721 as ::core::ffi::c_int as int16_t,
        1691 as ::core::ffi::c_int as int16_t,
        1762 as ::core::ffi::c_int as int16_t,
        1706 as ::core::ffi::c_int as int16_t,
        1582 as ::core::ffi::c_int as int16_t,
        1761 as ::core::ffi::c_int as int16_t,
        1566 as ::core::ffi::c_int as int16_t,
        -(1167 as ::core::ffi::c_int) as int16_t,
        1749 as ::core::ffi::c_int as int16_t,
        1629 as ::core::ffi::c_int as int16_t,
        767 as ::core::ffi::c_int as int16_t,
        766 as ::core::ffi::c_int as int16_t,
        751 as ::core::ffi::c_int as int16_t,
        765 as ::core::ffi::c_int as int16_t,
        494 as ::core::ffi::c_int as int16_t,
        494 as ::core::ffi::c_int as int16_t,
        735 as ::core::ffi::c_int as int16_t,
        764 as ::core::ffi::c_int as int16_t,
        719 as ::core::ffi::c_int as int16_t,
        749 as ::core::ffi::c_int as int16_t,
        734 as ::core::ffi::c_int as int16_t,
        763 as ::core::ffi::c_int as int16_t,
        447 as ::core::ffi::c_int as int16_t,
        447 as ::core::ffi::c_int as int16_t,
        748 as ::core::ffi::c_int as int16_t,
        718 as ::core::ffi::c_int as int16_t,
        477 as ::core::ffi::c_int as int16_t,
        506 as ::core::ffi::c_int as int16_t,
        431 as ::core::ffi::c_int as int16_t,
        491 as ::core::ffi::c_int as int16_t,
        446 as ::core::ffi::c_int as int16_t,
        476 as ::core::ffi::c_int as int16_t,
        461 as ::core::ffi::c_int as int16_t,
        505 as ::core::ffi::c_int as int16_t,
        415 as ::core::ffi::c_int as int16_t,
        430 as ::core::ffi::c_int as int16_t,
        475 as ::core::ffi::c_int as int16_t,
        445 as ::core::ffi::c_int as int16_t,
        504 as ::core::ffi::c_int as int16_t,
        399 as ::core::ffi::c_int as int16_t,
        460 as ::core::ffi::c_int as int16_t,
        489 as ::core::ffi::c_int as int16_t,
        414 as ::core::ffi::c_int as int16_t,
        503 as ::core::ffi::c_int as int16_t,
        383 as ::core::ffi::c_int as int16_t,
        474 as ::core::ffi::c_int as int16_t,
        429 as ::core::ffi::c_int as int16_t,
        459 as ::core::ffi::c_int as int16_t,
        502 as ::core::ffi::c_int as int16_t,
        502 as ::core::ffi::c_int as int16_t,
        746 as ::core::ffi::c_int as int16_t,
        752 as ::core::ffi::c_int as int16_t,
        488 as ::core::ffi::c_int as int16_t,
        398 as ::core::ffi::c_int as int16_t,
        501 as ::core::ffi::c_int as int16_t,
        473 as ::core::ffi::c_int as int16_t,
        413 as ::core::ffi::c_int as int16_t,
        472 as ::core::ffi::c_int as int16_t,
        486 as ::core::ffi::c_int as int16_t,
        271 as ::core::ffi::c_int as int16_t,
        480 as ::core::ffi::c_int as int16_t,
        270 as ::core::ffi::c_int as int16_t,
        -(1439 as ::core::ffi::c_int) as int16_t,
        -(1455 as ::core::ffi::c_int) as int16_t,
        1357 as ::core::ffi::c_int as int16_t,
        -(1471 as ::core::ffi::c_int) as int16_t,
        -(1487 as ::core::ffi::c_int) as int16_t,
        -(1503 as ::core::ffi::c_int) as int16_t,
        1341 as ::core::ffi::c_int as int16_t,
        1325 as ::core::ffi::c_int as int16_t,
        -(1519 as ::core::ffi::c_int) as int16_t,
        1489 as ::core::ffi::c_int as int16_t,
        1463 as ::core::ffi::c_int as int16_t,
        1403 as ::core::ffi::c_int as int16_t,
        1309 as ::core::ffi::c_int as int16_t,
        -(1535 as ::core::ffi::c_int) as int16_t,
        1372 as ::core::ffi::c_int as int16_t,
        1448 as ::core::ffi::c_int as int16_t,
        1418 as ::core::ffi::c_int as int16_t,
        1476 as ::core::ffi::c_int as int16_t,
        1356 as ::core::ffi::c_int as int16_t,
        1462 as ::core::ffi::c_int as int16_t,
        1387 as ::core::ffi::c_int as int16_t,
        -(1551 as ::core::ffi::c_int) as int16_t,
        1475 as ::core::ffi::c_int as int16_t,
        1340 as ::core::ffi::c_int as int16_t,
        1447 as ::core::ffi::c_int as int16_t,
        1402 as ::core::ffi::c_int as int16_t,
        1386 as ::core::ffi::c_int as int16_t,
        -(1567 as ::core::ffi::c_int) as int16_t,
        1068 as ::core::ffi::c_int as int16_t,
        1068 as ::core::ffi::c_int as int16_t,
        1474 as ::core::ffi::c_int as int16_t,
        1461 as ::core::ffi::c_int as int16_t,
        455 as ::core::ffi::c_int as int16_t,
        380 as ::core::ffi::c_int as int16_t,
        468 as ::core::ffi::c_int as int16_t,
        440 as ::core::ffi::c_int as int16_t,
        395 as ::core::ffi::c_int as int16_t,
        425 as ::core::ffi::c_int as int16_t,
        410 as ::core::ffi::c_int as int16_t,
        454 as ::core::ffi::c_int as int16_t,
        364 as ::core::ffi::c_int as int16_t,
        467 as ::core::ffi::c_int as int16_t,
        466 as ::core::ffi::c_int as int16_t,
        464 as ::core::ffi::c_int as int16_t,
        453 as ::core::ffi::c_int as int16_t,
        269 as ::core::ffi::c_int as int16_t,
        409 as ::core::ffi::c_int as int16_t,
        448 as ::core::ffi::c_int as int16_t,
        268 as ::core::ffi::c_int as int16_t,
        432 as ::core::ffi::c_int as int16_t,
        1371 as ::core::ffi::c_int as int16_t,
        1473 as ::core::ffi::c_int as int16_t,
        1432 as ::core::ffi::c_int as int16_t,
        1417 as ::core::ffi::c_int as int16_t,
        1308 as ::core::ffi::c_int as int16_t,
        1460 as ::core::ffi::c_int as int16_t,
        1355 as ::core::ffi::c_int as int16_t,
        1446 as ::core::ffi::c_int as int16_t,
        1459 as ::core::ffi::c_int as int16_t,
        1431 as ::core::ffi::c_int as int16_t,
        1083 as ::core::ffi::c_int as int16_t,
        1083 as ::core::ffi::c_int as int16_t,
        1401 as ::core::ffi::c_int as int16_t,
        1416 as ::core::ffi::c_int as int16_t,
        1458 as ::core::ffi::c_int as int16_t,
        1445 as ::core::ffi::c_int as int16_t,
        1067 as ::core::ffi::c_int as int16_t,
        1067 as ::core::ffi::c_int as int16_t,
        1370 as ::core::ffi::c_int as int16_t,
        1457 as ::core::ffi::c_int as int16_t,
        1051 as ::core::ffi::c_int as int16_t,
        1051 as ::core::ffi::c_int as int16_t,
        1291 as ::core::ffi::c_int as int16_t,
        1430 as ::core::ffi::c_int as int16_t,
        1385 as ::core::ffi::c_int as int16_t,
        1444 as ::core::ffi::c_int as int16_t,
        1354 as ::core::ffi::c_int as int16_t,
        1415 as ::core::ffi::c_int as int16_t,
        1400 as ::core::ffi::c_int as int16_t,
        1443 as ::core::ffi::c_int as int16_t,
        1082 as ::core::ffi::c_int as int16_t,
        1082 as ::core::ffi::c_int as int16_t,
        1173 as ::core::ffi::c_int as int16_t,
        1113 as ::core::ffi::c_int as int16_t,
        1186 as ::core::ffi::c_int as int16_t,
        1066 as ::core::ffi::c_int as int16_t,
        1185 as ::core::ffi::c_int as int16_t,
        1050 as ::core::ffi::c_int as int16_t,
        -(1967 as ::core::ffi::c_int) as int16_t,
        1158 as ::core::ffi::c_int as int16_t,
        1128 as ::core::ffi::c_int as int16_t,
        1172 as ::core::ffi::c_int as int16_t,
        1097 as ::core::ffi::c_int as int16_t,
        1171 as ::core::ffi::c_int as int16_t,
        1081 as ::core::ffi::c_int as int16_t,
        -(1983 as ::core::ffi::c_int) as int16_t,
        1157 as ::core::ffi::c_int as int16_t,
        1112 as ::core::ffi::c_int as int16_t,
        416 as ::core::ffi::c_int as int16_t,
        266 as ::core::ffi::c_int as int16_t,
        375 as ::core::ffi::c_int as int16_t,
        400 as ::core::ffi::c_int as int16_t,
        1170 as ::core::ffi::c_int as int16_t,
        1142 as ::core::ffi::c_int as int16_t,
        1127 as ::core::ffi::c_int as int16_t,
        1065 as ::core::ffi::c_int as int16_t,
        793 as ::core::ffi::c_int as int16_t,
        793 as ::core::ffi::c_int as int16_t,
        1169 as ::core::ffi::c_int as int16_t,
        1033 as ::core::ffi::c_int as int16_t,
        1156 as ::core::ffi::c_int as int16_t,
        1096 as ::core::ffi::c_int as int16_t,
        1141 as ::core::ffi::c_int as int16_t,
        1111 as ::core::ffi::c_int as int16_t,
        1155 as ::core::ffi::c_int as int16_t,
        1080 as ::core::ffi::c_int as int16_t,
        1126 as ::core::ffi::c_int as int16_t,
        1140 as ::core::ffi::c_int as int16_t,
        898 as ::core::ffi::c_int as int16_t,
        898 as ::core::ffi::c_int as int16_t,
        808 as ::core::ffi::c_int as int16_t,
        808 as ::core::ffi::c_int as int16_t,
        897 as ::core::ffi::c_int as int16_t,
        897 as ::core::ffi::c_int as int16_t,
        792 as ::core::ffi::c_int as int16_t,
        792 as ::core::ffi::c_int as int16_t,
        1095 as ::core::ffi::c_int as int16_t,
        1152 as ::core::ffi::c_int as int16_t,
        1032 as ::core::ffi::c_int as int16_t,
        1125 as ::core::ffi::c_int as int16_t,
        1110 as ::core::ffi::c_int as int16_t,
        1139 as ::core::ffi::c_int as int16_t,
        1079 as ::core::ffi::c_int as int16_t,
        1124 as ::core::ffi::c_int as int16_t,
        882 as ::core::ffi::c_int as int16_t,
        807 as ::core::ffi::c_int as int16_t,
        838 as ::core::ffi::c_int as int16_t,
        881 as ::core::ffi::c_int as int16_t,
        853 as ::core::ffi::c_int as int16_t,
        791 as ::core::ffi::c_int as int16_t,
        -(2319 as ::core::ffi::c_int) as int16_t,
        867 as ::core::ffi::c_int as int16_t,
        368 as ::core::ffi::c_int as int16_t,
        263 as ::core::ffi::c_int as int16_t,
        822 as ::core::ffi::c_int as int16_t,
        852 as ::core::ffi::c_int as int16_t,
        837 as ::core::ffi::c_int as int16_t,
        866 as ::core::ffi::c_int as int16_t,
        806 as ::core::ffi::c_int as int16_t,
        865 as ::core::ffi::c_int as int16_t,
        -(2399 as ::core::ffi::c_int) as int16_t,
        851 as ::core::ffi::c_int as int16_t,
        352 as ::core::ffi::c_int as int16_t,
        262 as ::core::ffi::c_int as int16_t,
        534 as ::core::ffi::c_int as int16_t,
        534 as ::core::ffi::c_int as int16_t,
        821 as ::core::ffi::c_int as int16_t,
        836 as ::core::ffi::c_int as int16_t,
        594 as ::core::ffi::c_int as int16_t,
        594 as ::core::ffi::c_int as int16_t,
        549 as ::core::ffi::c_int as int16_t,
        549 as ::core::ffi::c_int as int16_t,
        593 as ::core::ffi::c_int as int16_t,
        593 as ::core::ffi::c_int as int16_t,
        533 as ::core::ffi::c_int as int16_t,
        533 as ::core::ffi::c_int as int16_t,
        848 as ::core::ffi::c_int as int16_t,
        773 as ::core::ffi::c_int as int16_t,
        579 as ::core::ffi::c_int as int16_t,
        579 as ::core::ffi::c_int as int16_t,
        564 as ::core::ffi::c_int as int16_t,
        578 as ::core::ffi::c_int as int16_t,
        548 as ::core::ffi::c_int as int16_t,
        563 as ::core::ffi::c_int as int16_t,
        276 as ::core::ffi::c_int as int16_t,
        276 as ::core::ffi::c_int as int16_t,
        577 as ::core::ffi::c_int as int16_t,
        576 as ::core::ffi::c_int as int16_t,
        306 as ::core::ffi::c_int as int16_t,
        291 as ::core::ffi::c_int as int16_t,
        516 as ::core::ffi::c_int as int16_t,
        560 as ::core::ffi::c_int as int16_t,
        305 as ::core::ffi::c_int as int16_t,
        305 as ::core::ffi::c_int as int16_t,
        275 as ::core::ffi::c_int as int16_t,
        259 as ::core::ffi::c_int as int16_t,
        -(251 as ::core::ffi::c_int) as int16_t,
        -(892 as ::core::ffi::c_int) as int16_t,
        -(2058 as ::core::ffi::c_int) as int16_t,
        -(2620 as ::core::ffi::c_int) as int16_t,
        -(2828 as ::core::ffi::c_int) as int16_t,
        -(2957 as ::core::ffi::c_int) as int16_t,
        -(3023 as ::core::ffi::c_int) as int16_t,
        -(3039 as ::core::ffi::c_int) as int16_t,
        1041 as ::core::ffi::c_int as int16_t,
        1041 as ::core::ffi::c_int as int16_t,
        1040 as ::core::ffi::c_int as int16_t,
        1040 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        769 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        256 as ::core::ffi::c_int as int16_t,
        -(511 as ::core::ffi::c_int) as int16_t,
        -(527 as ::core::ffi::c_int) as int16_t,
        -(543 as ::core::ffi::c_int) as int16_t,
        -(559 as ::core::ffi::c_int) as int16_t,
        1530 as ::core::ffi::c_int as int16_t,
        -(575 as ::core::ffi::c_int) as int16_t,
        -(591 as ::core::ffi::c_int) as int16_t,
        1528 as ::core::ffi::c_int as int16_t,
        1527 as ::core::ffi::c_int as int16_t,
        1407 as ::core::ffi::c_int as int16_t,
        1526 as ::core::ffi::c_int as int16_t,
        1391 as ::core::ffi::c_int as int16_t,
        1023 as ::core::ffi::c_int as int16_t,
        1023 as ::core::ffi::c_int as int16_t,
        1023 as ::core::ffi::c_int as int16_t,
        1023 as ::core::ffi::c_int as int16_t,
        1525 as ::core::ffi::c_int as int16_t,
        1375 as ::core::ffi::c_int as int16_t,
        1268 as ::core::ffi::c_int as int16_t,
        1268 as ::core::ffi::c_int as int16_t,
        1103 as ::core::ffi::c_int as int16_t,
        1103 as ::core::ffi::c_int as int16_t,
        1087 as ::core::ffi::c_int as int16_t,
        1087 as ::core::ffi::c_int as int16_t,
        1039 as ::core::ffi::c_int as int16_t,
        1039 as ::core::ffi::c_int as int16_t,
        1523 as ::core::ffi::c_int as int16_t,
        -(604 as ::core::ffi::c_int) as int16_t,
        815 as ::core::ffi::c_int as int16_t,
        815 as ::core::ffi::c_int as int16_t,
        815 as ::core::ffi::c_int as int16_t,
        815 as ::core::ffi::c_int as int16_t,
        510 as ::core::ffi::c_int as int16_t,
        495 as ::core::ffi::c_int as int16_t,
        509 as ::core::ffi::c_int as int16_t,
        479 as ::core::ffi::c_int as int16_t,
        508 as ::core::ffi::c_int as int16_t,
        463 as ::core::ffi::c_int as int16_t,
        507 as ::core::ffi::c_int as int16_t,
        447 as ::core::ffi::c_int as int16_t,
        431 as ::core::ffi::c_int as int16_t,
        505 as ::core::ffi::c_int as int16_t,
        415 as ::core::ffi::c_int as int16_t,
        399 as ::core::ffi::c_int as int16_t,
        -(734 as ::core::ffi::c_int) as int16_t,
        -(782 as ::core::ffi::c_int) as int16_t,
        1262 as ::core::ffi::c_int as int16_t,
        -(815 as ::core::ffi::c_int) as int16_t,
        1259 as ::core::ffi::c_int as int16_t,
        1244 as ::core::ffi::c_int as int16_t,
        -(831 as ::core::ffi::c_int) as int16_t,
        1258 as ::core::ffi::c_int as int16_t,
        1228 as ::core::ffi::c_int as int16_t,
        -(847 as ::core::ffi::c_int) as int16_t,
        -(863 as ::core::ffi::c_int) as int16_t,
        1196 as ::core::ffi::c_int as int16_t,
        -(879 as ::core::ffi::c_int) as int16_t,
        1253 as ::core::ffi::c_int as int16_t,
        987 as ::core::ffi::c_int as int16_t,
        987 as ::core::ffi::c_int as int16_t,
        748 as ::core::ffi::c_int as int16_t,
        -(767 as ::core::ffi::c_int) as int16_t,
        493 as ::core::ffi::c_int as int16_t,
        493 as ::core::ffi::c_int as int16_t,
        462 as ::core::ffi::c_int as int16_t,
        477 as ::core::ffi::c_int as int16_t,
        414 as ::core::ffi::c_int as int16_t,
        414 as ::core::ffi::c_int as int16_t,
        686 as ::core::ffi::c_int as int16_t,
        669 as ::core::ffi::c_int as int16_t,
        478 as ::core::ffi::c_int as int16_t,
        446 as ::core::ffi::c_int as int16_t,
        461 as ::core::ffi::c_int as int16_t,
        445 as ::core::ffi::c_int as int16_t,
        474 as ::core::ffi::c_int as int16_t,
        429 as ::core::ffi::c_int as int16_t,
        487 as ::core::ffi::c_int as int16_t,
        458 as ::core::ffi::c_int as int16_t,
        412 as ::core::ffi::c_int as int16_t,
        471 as ::core::ffi::c_int as int16_t,
        1266 as ::core::ffi::c_int as int16_t,
        1264 as ::core::ffi::c_int as int16_t,
        1009 as ::core::ffi::c_int as int16_t,
        1009 as ::core::ffi::c_int as int16_t,
        799 as ::core::ffi::c_int as int16_t,
        799 as ::core::ffi::c_int as int16_t,
        -(1019 as ::core::ffi::c_int) as int16_t,
        -(1276 as ::core::ffi::c_int) as int16_t,
        -(1452 as ::core::ffi::c_int) as int16_t,
        -(1581 as ::core::ffi::c_int) as int16_t,
        -(1677 as ::core::ffi::c_int) as int16_t,
        -(1757 as ::core::ffi::c_int) as int16_t,
        -(1821 as ::core::ffi::c_int) as int16_t,
        -(1886 as ::core::ffi::c_int) as int16_t,
        -(1933 as ::core::ffi::c_int) as int16_t,
        -(1997 as ::core::ffi::c_int) as int16_t,
        1257 as ::core::ffi::c_int as int16_t,
        1257 as ::core::ffi::c_int as int16_t,
        1483 as ::core::ffi::c_int as int16_t,
        1468 as ::core::ffi::c_int as int16_t,
        1512 as ::core::ffi::c_int as int16_t,
        1422 as ::core::ffi::c_int as int16_t,
        1497 as ::core::ffi::c_int as int16_t,
        1406 as ::core::ffi::c_int as int16_t,
        1467 as ::core::ffi::c_int as int16_t,
        1496 as ::core::ffi::c_int as int16_t,
        1421 as ::core::ffi::c_int as int16_t,
        1510 as ::core::ffi::c_int as int16_t,
        1134 as ::core::ffi::c_int as int16_t,
        1134 as ::core::ffi::c_int as int16_t,
        1225 as ::core::ffi::c_int as int16_t,
        1225 as ::core::ffi::c_int as int16_t,
        1466 as ::core::ffi::c_int as int16_t,
        1451 as ::core::ffi::c_int as int16_t,
        1374 as ::core::ffi::c_int as int16_t,
        1405 as ::core::ffi::c_int as int16_t,
        1252 as ::core::ffi::c_int as int16_t,
        1252 as ::core::ffi::c_int as int16_t,
        1358 as ::core::ffi::c_int as int16_t,
        1480 as ::core::ffi::c_int as int16_t,
        1164 as ::core::ffi::c_int as int16_t,
        1164 as ::core::ffi::c_int as int16_t,
        1251 as ::core::ffi::c_int as int16_t,
        1251 as ::core::ffi::c_int as int16_t,
        1238 as ::core::ffi::c_int as int16_t,
        1238 as ::core::ffi::c_int as int16_t,
        1389 as ::core::ffi::c_int as int16_t,
        1465 as ::core::ffi::c_int as int16_t,
        -(1407 as ::core::ffi::c_int) as int16_t,
        1054 as ::core::ffi::c_int as int16_t,
        1101 as ::core::ffi::c_int as int16_t,
        -(1423 as ::core::ffi::c_int) as int16_t,
        1207 as ::core::ffi::c_int as int16_t,
        -(1439 as ::core::ffi::c_int) as int16_t,
        830 as ::core::ffi::c_int as int16_t,
        830 as ::core::ffi::c_int as int16_t,
        1248 as ::core::ffi::c_int as int16_t,
        1038 as ::core::ffi::c_int as int16_t,
        1237 as ::core::ffi::c_int as int16_t,
        1117 as ::core::ffi::c_int as int16_t,
        1223 as ::core::ffi::c_int as int16_t,
        1148 as ::core::ffi::c_int as int16_t,
        1236 as ::core::ffi::c_int as int16_t,
        1208 as ::core::ffi::c_int as int16_t,
        411 as ::core::ffi::c_int as int16_t,
        426 as ::core::ffi::c_int as int16_t,
        395 as ::core::ffi::c_int as int16_t,
        410 as ::core::ffi::c_int as int16_t,
        379 as ::core::ffi::c_int as int16_t,
        269 as ::core::ffi::c_int as int16_t,
        1193 as ::core::ffi::c_int as int16_t,
        1222 as ::core::ffi::c_int as int16_t,
        1132 as ::core::ffi::c_int as int16_t,
        1235 as ::core::ffi::c_int as int16_t,
        1221 as ::core::ffi::c_int as int16_t,
        1116 as ::core::ffi::c_int as int16_t,
        976 as ::core::ffi::c_int as int16_t,
        976 as ::core::ffi::c_int as int16_t,
        1192 as ::core::ffi::c_int as int16_t,
        1162 as ::core::ffi::c_int as int16_t,
        1177 as ::core::ffi::c_int as int16_t,
        1220 as ::core::ffi::c_int as int16_t,
        1131 as ::core::ffi::c_int as int16_t,
        1191 as ::core::ffi::c_int as int16_t,
        963 as ::core::ffi::c_int as int16_t,
        963 as ::core::ffi::c_int as int16_t,
        -(1647 as ::core::ffi::c_int) as int16_t,
        961 as ::core::ffi::c_int as int16_t,
        780 as ::core::ffi::c_int as int16_t,
        -(1663 as ::core::ffi::c_int) as int16_t,
        558 as ::core::ffi::c_int as int16_t,
        558 as ::core::ffi::c_int as int16_t,
        994 as ::core::ffi::c_int as int16_t,
        993 as ::core::ffi::c_int as int16_t,
        437 as ::core::ffi::c_int as int16_t,
        408 as ::core::ffi::c_int as int16_t,
        393 as ::core::ffi::c_int as int16_t,
        407 as ::core::ffi::c_int as int16_t,
        829 as ::core::ffi::c_int as int16_t,
        978 as ::core::ffi::c_int as int16_t,
        813 as ::core::ffi::c_int as int16_t,
        797 as ::core::ffi::c_int as int16_t,
        947 as ::core::ffi::c_int as int16_t,
        -(1743 as ::core::ffi::c_int) as int16_t,
        721 as ::core::ffi::c_int as int16_t,
        721 as ::core::ffi::c_int as int16_t,
        377 as ::core::ffi::c_int as int16_t,
        392 as ::core::ffi::c_int as int16_t,
        844 as ::core::ffi::c_int as int16_t,
        950 as ::core::ffi::c_int as int16_t,
        828 as ::core::ffi::c_int as int16_t,
        890 as ::core::ffi::c_int as int16_t,
        706 as ::core::ffi::c_int as int16_t,
        706 as ::core::ffi::c_int as int16_t,
        812 as ::core::ffi::c_int as int16_t,
        859 as ::core::ffi::c_int as int16_t,
        796 as ::core::ffi::c_int as int16_t,
        960 as ::core::ffi::c_int as int16_t,
        948 as ::core::ffi::c_int as int16_t,
        843 as ::core::ffi::c_int as int16_t,
        934 as ::core::ffi::c_int as int16_t,
        874 as ::core::ffi::c_int as int16_t,
        571 as ::core::ffi::c_int as int16_t,
        571 as ::core::ffi::c_int as int16_t,
        -(1919 as ::core::ffi::c_int) as int16_t,
        690 as ::core::ffi::c_int as int16_t,
        555 as ::core::ffi::c_int as int16_t,
        689 as ::core::ffi::c_int as int16_t,
        421 as ::core::ffi::c_int as int16_t,
        346 as ::core::ffi::c_int as int16_t,
        539 as ::core::ffi::c_int as int16_t,
        539 as ::core::ffi::c_int as int16_t,
        944 as ::core::ffi::c_int as int16_t,
        779 as ::core::ffi::c_int as int16_t,
        918 as ::core::ffi::c_int as int16_t,
        873 as ::core::ffi::c_int as int16_t,
        932 as ::core::ffi::c_int as int16_t,
        842 as ::core::ffi::c_int as int16_t,
        903 as ::core::ffi::c_int as int16_t,
        888 as ::core::ffi::c_int as int16_t,
        570 as ::core::ffi::c_int as int16_t,
        570 as ::core::ffi::c_int as int16_t,
        931 as ::core::ffi::c_int as int16_t,
        917 as ::core::ffi::c_int as int16_t,
        674 as ::core::ffi::c_int as int16_t,
        674 as ::core::ffi::c_int as int16_t,
        -(2575 as ::core::ffi::c_int) as int16_t,
        1562 as ::core::ffi::c_int as int16_t,
        -(2591 as ::core::ffi::c_int) as int16_t,
        1609 as ::core::ffi::c_int as int16_t,
        -(2607 as ::core::ffi::c_int) as int16_t,
        1654 as ::core::ffi::c_int as int16_t,
        1322 as ::core::ffi::c_int as int16_t,
        1322 as ::core::ffi::c_int as int16_t,
        1441 as ::core::ffi::c_int as int16_t,
        1441 as ::core::ffi::c_int as int16_t,
        1696 as ::core::ffi::c_int as int16_t,
        1546 as ::core::ffi::c_int as int16_t,
        1683 as ::core::ffi::c_int as int16_t,
        1593 as ::core::ffi::c_int as int16_t,
        1669 as ::core::ffi::c_int as int16_t,
        1624 as ::core::ffi::c_int as int16_t,
        1426 as ::core::ffi::c_int as int16_t,
        1426 as ::core::ffi::c_int as int16_t,
        1321 as ::core::ffi::c_int as int16_t,
        1321 as ::core::ffi::c_int as int16_t,
        1639 as ::core::ffi::c_int as int16_t,
        1680 as ::core::ffi::c_int as int16_t,
        1425 as ::core::ffi::c_int as int16_t,
        1425 as ::core::ffi::c_int as int16_t,
        1305 as ::core::ffi::c_int as int16_t,
        1305 as ::core::ffi::c_int as int16_t,
        1545 as ::core::ffi::c_int as int16_t,
        1668 as ::core::ffi::c_int as int16_t,
        1608 as ::core::ffi::c_int as int16_t,
        1623 as ::core::ffi::c_int as int16_t,
        1667 as ::core::ffi::c_int as int16_t,
        1592 as ::core::ffi::c_int as int16_t,
        1638 as ::core::ffi::c_int as int16_t,
        1666 as ::core::ffi::c_int as int16_t,
        1320 as ::core::ffi::c_int as int16_t,
        1320 as ::core::ffi::c_int as int16_t,
        1652 as ::core::ffi::c_int as int16_t,
        1607 as ::core::ffi::c_int as int16_t,
        1409 as ::core::ffi::c_int as int16_t,
        1409 as ::core::ffi::c_int as int16_t,
        1304 as ::core::ffi::c_int as int16_t,
        1304 as ::core::ffi::c_int as int16_t,
        1288 as ::core::ffi::c_int as int16_t,
        1288 as ::core::ffi::c_int as int16_t,
        1664 as ::core::ffi::c_int as int16_t,
        1637 as ::core::ffi::c_int as int16_t,
        1395 as ::core::ffi::c_int as int16_t,
        1395 as ::core::ffi::c_int as int16_t,
        1335 as ::core::ffi::c_int as int16_t,
        1335 as ::core::ffi::c_int as int16_t,
        1622 as ::core::ffi::c_int as int16_t,
        1636 as ::core::ffi::c_int as int16_t,
        1394 as ::core::ffi::c_int as int16_t,
        1394 as ::core::ffi::c_int as int16_t,
        1319 as ::core::ffi::c_int as int16_t,
        1319 as ::core::ffi::c_int as int16_t,
        1606 as ::core::ffi::c_int as int16_t,
        1621 as ::core::ffi::c_int as int16_t,
        1392 as ::core::ffi::c_int as int16_t,
        1392 as ::core::ffi::c_int as int16_t,
        1137 as ::core::ffi::c_int as int16_t,
        1137 as ::core::ffi::c_int as int16_t,
        1137 as ::core::ffi::c_int as int16_t,
        1137 as ::core::ffi::c_int as int16_t,
        345 as ::core::ffi::c_int as int16_t,
        390 as ::core::ffi::c_int as int16_t,
        360 as ::core::ffi::c_int as int16_t,
        375 as ::core::ffi::c_int as int16_t,
        404 as ::core::ffi::c_int as int16_t,
        373 as ::core::ffi::c_int as int16_t,
        1047 as ::core::ffi::c_int as int16_t,
        -(2751 as ::core::ffi::c_int) as int16_t,
        -(2767 as ::core::ffi::c_int) as int16_t,
        -(2783 as ::core::ffi::c_int) as int16_t,
        1062 as ::core::ffi::c_int as int16_t,
        1121 as ::core::ffi::c_int as int16_t,
        1046 as ::core::ffi::c_int as int16_t,
        -(2799 as ::core::ffi::c_int) as int16_t,
        1077 as ::core::ffi::c_int as int16_t,
        -(2815 as ::core::ffi::c_int) as int16_t,
        1106 as ::core::ffi::c_int as int16_t,
        1061 as ::core::ffi::c_int as int16_t,
        789 as ::core::ffi::c_int as int16_t,
        789 as ::core::ffi::c_int as int16_t,
        1105 as ::core::ffi::c_int as int16_t,
        1104 as ::core::ffi::c_int as int16_t,
        263 as ::core::ffi::c_int as int16_t,
        355 as ::core::ffi::c_int as int16_t,
        310 as ::core::ffi::c_int as int16_t,
        340 as ::core::ffi::c_int as int16_t,
        325 as ::core::ffi::c_int as int16_t,
        354 as ::core::ffi::c_int as int16_t,
        352 as ::core::ffi::c_int as int16_t,
        262 as ::core::ffi::c_int as int16_t,
        339 as ::core::ffi::c_int as int16_t,
        324 as ::core::ffi::c_int as int16_t,
        1091 as ::core::ffi::c_int as int16_t,
        1076 as ::core::ffi::c_int as int16_t,
        1029 as ::core::ffi::c_int as int16_t,
        1090 as ::core::ffi::c_int as int16_t,
        1060 as ::core::ffi::c_int as int16_t,
        1075 as ::core::ffi::c_int as int16_t,
        833 as ::core::ffi::c_int as int16_t,
        833 as ::core::ffi::c_int as int16_t,
        788 as ::core::ffi::c_int as int16_t,
        788 as ::core::ffi::c_int as int16_t,
        1088 as ::core::ffi::c_int as int16_t,
        1028 as ::core::ffi::c_int as int16_t,
        818 as ::core::ffi::c_int as int16_t,
        818 as ::core::ffi::c_int as int16_t,
        803 as ::core::ffi::c_int as int16_t,
        803 as ::core::ffi::c_int as int16_t,
        561 as ::core::ffi::c_int as int16_t,
        561 as ::core::ffi::c_int as int16_t,
        531 as ::core::ffi::c_int as int16_t,
        531 as ::core::ffi::c_int as int16_t,
        816 as ::core::ffi::c_int as int16_t,
        771 as ::core::ffi::c_int as int16_t,
        546 as ::core::ffi::c_int as int16_t,
        546 as ::core::ffi::c_int as int16_t,
        289 as ::core::ffi::c_int as int16_t,
        274 as ::core::ffi::c_int as int16_t,
        288 as ::core::ffi::c_int as int16_t,
        258 as ::core::ffi::c_int as int16_t,
        -(253 as ::core::ffi::c_int) as int16_t,
        -(317 as ::core::ffi::c_int) as int16_t,
        -(381 as ::core::ffi::c_int) as int16_t,
        -(446 as ::core::ffi::c_int) as int16_t,
        -(478 as ::core::ffi::c_int) as int16_t,
        -(509 as ::core::ffi::c_int) as int16_t,
        1279 as ::core::ffi::c_int as int16_t,
        1279 as ::core::ffi::c_int as int16_t,
        -(811 as ::core::ffi::c_int) as int16_t,
        -(1179 as ::core::ffi::c_int) as int16_t,
        -(1451 as ::core::ffi::c_int) as int16_t,
        -(1756 as ::core::ffi::c_int) as int16_t,
        -(1900 as ::core::ffi::c_int) as int16_t,
        -(2028 as ::core::ffi::c_int) as int16_t,
        -(2189 as ::core::ffi::c_int) as int16_t,
        -(2253 as ::core::ffi::c_int) as int16_t,
        -(2333 as ::core::ffi::c_int) as int16_t,
        -(2414 as ::core::ffi::c_int) as int16_t,
        -(2445 as ::core::ffi::c_int) as int16_t,
        -(2511 as ::core::ffi::c_int) as int16_t,
        -(2526 as ::core::ffi::c_int) as int16_t,
        1313 as ::core::ffi::c_int as int16_t,
        1298 as ::core::ffi::c_int as int16_t,
        -(2559 as ::core::ffi::c_int) as int16_t,
        1041 as ::core::ffi::c_int as int16_t,
        1041 as ::core::ffi::c_int as int16_t,
        1040 as ::core::ffi::c_int as int16_t,
        1040 as ::core::ffi::c_int as int16_t,
        1025 as ::core::ffi::c_int as int16_t,
        1025 as ::core::ffi::c_int as int16_t,
        1024 as ::core::ffi::c_int as int16_t,
        1024 as ::core::ffi::c_int as int16_t,
        1022 as ::core::ffi::c_int as int16_t,
        1007 as ::core::ffi::c_int as int16_t,
        1021 as ::core::ffi::c_int as int16_t,
        991 as ::core::ffi::c_int as int16_t,
        1020 as ::core::ffi::c_int as int16_t,
        975 as ::core::ffi::c_int as int16_t,
        1019 as ::core::ffi::c_int as int16_t,
        959 as ::core::ffi::c_int as int16_t,
        687 as ::core::ffi::c_int as int16_t,
        687 as ::core::ffi::c_int as int16_t,
        1018 as ::core::ffi::c_int as int16_t,
        1017 as ::core::ffi::c_int as int16_t,
        671 as ::core::ffi::c_int as int16_t,
        671 as ::core::ffi::c_int as int16_t,
        655 as ::core::ffi::c_int as int16_t,
        655 as ::core::ffi::c_int as int16_t,
        1016 as ::core::ffi::c_int as int16_t,
        1015 as ::core::ffi::c_int as int16_t,
        639 as ::core::ffi::c_int as int16_t,
        639 as ::core::ffi::c_int as int16_t,
        758 as ::core::ffi::c_int as int16_t,
        758 as ::core::ffi::c_int as int16_t,
        623 as ::core::ffi::c_int as int16_t,
        623 as ::core::ffi::c_int as int16_t,
        757 as ::core::ffi::c_int as int16_t,
        607 as ::core::ffi::c_int as int16_t,
        756 as ::core::ffi::c_int as int16_t,
        591 as ::core::ffi::c_int as int16_t,
        755 as ::core::ffi::c_int as int16_t,
        575 as ::core::ffi::c_int as int16_t,
        754 as ::core::ffi::c_int as int16_t,
        559 as ::core::ffi::c_int as int16_t,
        543 as ::core::ffi::c_int as int16_t,
        543 as ::core::ffi::c_int as int16_t,
        1009 as ::core::ffi::c_int as int16_t,
        783 as ::core::ffi::c_int as int16_t,
        -(575 as ::core::ffi::c_int) as int16_t,
        -(621 as ::core::ffi::c_int) as int16_t,
        -(685 as ::core::ffi::c_int) as int16_t,
        -(749 as ::core::ffi::c_int) as int16_t,
        496 as ::core::ffi::c_int as int16_t,
        -(590 as ::core::ffi::c_int) as int16_t,
        750 as ::core::ffi::c_int as int16_t,
        749 as ::core::ffi::c_int as int16_t,
        734 as ::core::ffi::c_int as int16_t,
        748 as ::core::ffi::c_int as int16_t,
        974 as ::core::ffi::c_int as int16_t,
        989 as ::core::ffi::c_int as int16_t,
        1003 as ::core::ffi::c_int as int16_t,
        958 as ::core::ffi::c_int as int16_t,
        988 as ::core::ffi::c_int as int16_t,
        973 as ::core::ffi::c_int as int16_t,
        1002 as ::core::ffi::c_int as int16_t,
        942 as ::core::ffi::c_int as int16_t,
        987 as ::core::ffi::c_int as int16_t,
        957 as ::core::ffi::c_int as int16_t,
        972 as ::core::ffi::c_int as int16_t,
        1001 as ::core::ffi::c_int as int16_t,
        926 as ::core::ffi::c_int as int16_t,
        986 as ::core::ffi::c_int as int16_t,
        941 as ::core::ffi::c_int as int16_t,
        971 as ::core::ffi::c_int as int16_t,
        956 as ::core::ffi::c_int as int16_t,
        1000 as ::core::ffi::c_int as int16_t,
        910 as ::core::ffi::c_int as int16_t,
        985 as ::core::ffi::c_int as int16_t,
        925 as ::core::ffi::c_int as int16_t,
        999 as ::core::ffi::c_int as int16_t,
        894 as ::core::ffi::c_int as int16_t,
        970 as ::core::ffi::c_int as int16_t,
        -(1071 as ::core::ffi::c_int) as int16_t,
        -(1087 as ::core::ffi::c_int) as int16_t,
        -(1102 as ::core::ffi::c_int) as int16_t,
        1390 as ::core::ffi::c_int as int16_t,
        -(1135 as ::core::ffi::c_int) as int16_t,
        1436 as ::core::ffi::c_int as int16_t,
        1509 as ::core::ffi::c_int as int16_t,
        1451 as ::core::ffi::c_int as int16_t,
        1374 as ::core::ffi::c_int as int16_t,
        -(1151 as ::core::ffi::c_int) as int16_t,
        1405 as ::core::ffi::c_int as int16_t,
        1358 as ::core::ffi::c_int as int16_t,
        1480 as ::core::ffi::c_int as int16_t,
        1420 as ::core::ffi::c_int as int16_t,
        -(1167 as ::core::ffi::c_int) as int16_t,
        1507 as ::core::ffi::c_int as int16_t,
        1494 as ::core::ffi::c_int as int16_t,
        1389 as ::core::ffi::c_int as int16_t,
        1342 as ::core::ffi::c_int as int16_t,
        1465 as ::core::ffi::c_int as int16_t,
        1435 as ::core::ffi::c_int as int16_t,
        1450 as ::core::ffi::c_int as int16_t,
        1326 as ::core::ffi::c_int as int16_t,
        1505 as ::core::ffi::c_int as int16_t,
        1310 as ::core::ffi::c_int as int16_t,
        1493 as ::core::ffi::c_int as int16_t,
        1373 as ::core::ffi::c_int as int16_t,
        1479 as ::core::ffi::c_int as int16_t,
        1404 as ::core::ffi::c_int as int16_t,
        1492 as ::core::ffi::c_int as int16_t,
        1464 as ::core::ffi::c_int as int16_t,
        1419 as ::core::ffi::c_int as int16_t,
        428 as ::core::ffi::c_int as int16_t,
        443 as ::core::ffi::c_int as int16_t,
        472 as ::core::ffi::c_int as int16_t,
        397 as ::core::ffi::c_int as int16_t,
        736 as ::core::ffi::c_int as int16_t,
        526 as ::core::ffi::c_int as int16_t,
        464 as ::core::ffi::c_int as int16_t,
        464 as ::core::ffi::c_int as int16_t,
        486 as ::core::ffi::c_int as int16_t,
        457 as ::core::ffi::c_int as int16_t,
        442 as ::core::ffi::c_int as int16_t,
        471 as ::core::ffi::c_int as int16_t,
        484 as ::core::ffi::c_int as int16_t,
        482 as ::core::ffi::c_int as int16_t,
        1357 as ::core::ffi::c_int as int16_t,
        1449 as ::core::ffi::c_int as int16_t,
        1434 as ::core::ffi::c_int as int16_t,
        1478 as ::core::ffi::c_int as int16_t,
        1388 as ::core::ffi::c_int as int16_t,
        1491 as ::core::ffi::c_int as int16_t,
        1341 as ::core::ffi::c_int as int16_t,
        1490 as ::core::ffi::c_int as int16_t,
        1325 as ::core::ffi::c_int as int16_t,
        1489 as ::core::ffi::c_int as int16_t,
        1463 as ::core::ffi::c_int as int16_t,
        1403 as ::core::ffi::c_int as int16_t,
        1309 as ::core::ffi::c_int as int16_t,
        1477 as ::core::ffi::c_int as int16_t,
        1372 as ::core::ffi::c_int as int16_t,
        1448 as ::core::ffi::c_int as int16_t,
        1418 as ::core::ffi::c_int as int16_t,
        1433 as ::core::ffi::c_int as int16_t,
        1476 as ::core::ffi::c_int as int16_t,
        1356 as ::core::ffi::c_int as int16_t,
        1462 as ::core::ffi::c_int as int16_t,
        1387 as ::core::ffi::c_int as int16_t,
        -(1439 as ::core::ffi::c_int) as int16_t,
        1475 as ::core::ffi::c_int as int16_t,
        1340 as ::core::ffi::c_int as int16_t,
        1447 as ::core::ffi::c_int as int16_t,
        1402 as ::core::ffi::c_int as int16_t,
        1474 as ::core::ffi::c_int as int16_t,
        1324 as ::core::ffi::c_int as int16_t,
        1461 as ::core::ffi::c_int as int16_t,
        1371 as ::core::ffi::c_int as int16_t,
        1473 as ::core::ffi::c_int as int16_t,
        269 as ::core::ffi::c_int as int16_t,
        448 as ::core::ffi::c_int as int16_t,
        1432 as ::core::ffi::c_int as int16_t,
        1417 as ::core::ffi::c_int as int16_t,
        1308 as ::core::ffi::c_int as int16_t,
        1460 as ::core::ffi::c_int as int16_t,
        -(1711 as ::core::ffi::c_int) as int16_t,
        1459 as ::core::ffi::c_int as int16_t,
        -(1727 as ::core::ffi::c_int) as int16_t,
        1441 as ::core::ffi::c_int as int16_t,
        1099 as ::core::ffi::c_int as int16_t,
        1099 as ::core::ffi::c_int as int16_t,
        1446 as ::core::ffi::c_int as int16_t,
        1386 as ::core::ffi::c_int as int16_t,
        1431 as ::core::ffi::c_int as int16_t,
        1401 as ::core::ffi::c_int as int16_t,
        -(1743 as ::core::ffi::c_int) as int16_t,
        1289 as ::core::ffi::c_int as int16_t,
        1083 as ::core::ffi::c_int as int16_t,
        1083 as ::core::ffi::c_int as int16_t,
        1160 as ::core::ffi::c_int as int16_t,
        1160 as ::core::ffi::c_int as int16_t,
        1458 as ::core::ffi::c_int as int16_t,
        1445 as ::core::ffi::c_int as int16_t,
        1067 as ::core::ffi::c_int as int16_t,
        1067 as ::core::ffi::c_int as int16_t,
        1370 as ::core::ffi::c_int as int16_t,
        1457 as ::core::ffi::c_int as int16_t,
        1307 as ::core::ffi::c_int as int16_t,
        1430 as ::core::ffi::c_int as int16_t,
        1129 as ::core::ffi::c_int as int16_t,
        1129 as ::core::ffi::c_int as int16_t,
        1098 as ::core::ffi::c_int as int16_t,
        1098 as ::core::ffi::c_int as int16_t,
        268 as ::core::ffi::c_int as int16_t,
        432 as ::core::ffi::c_int as int16_t,
        267 as ::core::ffi::c_int as int16_t,
        416 as ::core::ffi::c_int as int16_t,
        266 as ::core::ffi::c_int as int16_t,
        400 as ::core::ffi::c_int as int16_t,
        -(1887 as ::core::ffi::c_int) as int16_t,
        1144 as ::core::ffi::c_int as int16_t,
        1187 as ::core::ffi::c_int as int16_t,
        1082 as ::core::ffi::c_int as int16_t,
        1173 as ::core::ffi::c_int as int16_t,
        1113 as ::core::ffi::c_int as int16_t,
        1186 as ::core::ffi::c_int as int16_t,
        1066 as ::core::ffi::c_int as int16_t,
        1050 as ::core::ffi::c_int as int16_t,
        1158 as ::core::ffi::c_int as int16_t,
        1128 as ::core::ffi::c_int as int16_t,
        1143 as ::core::ffi::c_int as int16_t,
        1172 as ::core::ffi::c_int as int16_t,
        1097 as ::core::ffi::c_int as int16_t,
        1171 as ::core::ffi::c_int as int16_t,
        1081 as ::core::ffi::c_int as int16_t,
        420 as ::core::ffi::c_int as int16_t,
        391 as ::core::ffi::c_int as int16_t,
        1157 as ::core::ffi::c_int as int16_t,
        1112 as ::core::ffi::c_int as int16_t,
        1170 as ::core::ffi::c_int as int16_t,
        1142 as ::core::ffi::c_int as int16_t,
        1127 as ::core::ffi::c_int as int16_t,
        1065 as ::core::ffi::c_int as int16_t,
        1169 as ::core::ffi::c_int as int16_t,
        1049 as ::core::ffi::c_int as int16_t,
        1156 as ::core::ffi::c_int as int16_t,
        1096 as ::core::ffi::c_int as int16_t,
        1141 as ::core::ffi::c_int as int16_t,
        1111 as ::core::ffi::c_int as int16_t,
        1155 as ::core::ffi::c_int as int16_t,
        1080 as ::core::ffi::c_int as int16_t,
        1126 as ::core::ffi::c_int as int16_t,
        1154 as ::core::ffi::c_int as int16_t,
        1064 as ::core::ffi::c_int as int16_t,
        1153 as ::core::ffi::c_int as int16_t,
        1140 as ::core::ffi::c_int as int16_t,
        1095 as ::core::ffi::c_int as int16_t,
        1048 as ::core::ffi::c_int as int16_t,
        -(2159 as ::core::ffi::c_int) as int16_t,
        1125 as ::core::ffi::c_int as int16_t,
        1110 as ::core::ffi::c_int as int16_t,
        1137 as ::core::ffi::c_int as int16_t,
        -(2175 as ::core::ffi::c_int) as int16_t,
        823 as ::core::ffi::c_int as int16_t,
        823 as ::core::ffi::c_int as int16_t,
        1139 as ::core::ffi::c_int as int16_t,
        1138 as ::core::ffi::c_int as int16_t,
        807 as ::core::ffi::c_int as int16_t,
        807 as ::core::ffi::c_int as int16_t,
        384 as ::core::ffi::c_int as int16_t,
        264 as ::core::ffi::c_int as int16_t,
        368 as ::core::ffi::c_int as int16_t,
        263 as ::core::ffi::c_int as int16_t,
        868 as ::core::ffi::c_int as int16_t,
        838 as ::core::ffi::c_int as int16_t,
        853 as ::core::ffi::c_int as int16_t,
        791 as ::core::ffi::c_int as int16_t,
        867 as ::core::ffi::c_int as int16_t,
        822 as ::core::ffi::c_int as int16_t,
        852 as ::core::ffi::c_int as int16_t,
        837 as ::core::ffi::c_int as int16_t,
        866 as ::core::ffi::c_int as int16_t,
        806 as ::core::ffi::c_int as int16_t,
        865 as ::core::ffi::c_int as int16_t,
        790 as ::core::ffi::c_int as int16_t,
        -(2319 as ::core::ffi::c_int) as int16_t,
        851 as ::core::ffi::c_int as int16_t,
        821 as ::core::ffi::c_int as int16_t,
        836 as ::core::ffi::c_int as int16_t,
        352 as ::core::ffi::c_int as int16_t,
        262 as ::core::ffi::c_int as int16_t,
        850 as ::core::ffi::c_int as int16_t,
        805 as ::core::ffi::c_int as int16_t,
        849 as ::core::ffi::c_int as int16_t,
        -(2399 as ::core::ffi::c_int) as int16_t,
        533 as ::core::ffi::c_int as int16_t,
        533 as ::core::ffi::c_int as int16_t,
        835 as ::core::ffi::c_int as int16_t,
        820 as ::core::ffi::c_int as int16_t,
        336 as ::core::ffi::c_int as int16_t,
        261 as ::core::ffi::c_int as int16_t,
        578 as ::core::ffi::c_int as int16_t,
        548 as ::core::ffi::c_int as int16_t,
        563 as ::core::ffi::c_int as int16_t,
        577 as ::core::ffi::c_int as int16_t,
        532 as ::core::ffi::c_int as int16_t,
        532 as ::core::ffi::c_int as int16_t,
        832 as ::core::ffi::c_int as int16_t,
        772 as ::core::ffi::c_int as int16_t,
        562 as ::core::ffi::c_int as int16_t,
        562 as ::core::ffi::c_int as int16_t,
        547 as ::core::ffi::c_int as int16_t,
        547 as ::core::ffi::c_int as int16_t,
        305 as ::core::ffi::c_int as int16_t,
        275 as ::core::ffi::c_int as int16_t,
        560 as ::core::ffi::c_int as int16_t,
        515 as ::core::ffi::c_int as int16_t,
        290 as ::core::ffi::c_int as int16_t,
        290 as ::core::ffi::c_int as int16_t,
        288 as ::core::ffi::c_int as int16_t,
        258 as ::core::ffi::c_int as int16_t,
    ];
    static mut tab32: [uint8_t; 28] = [
        130 as ::core::ffi::c_int as uint8_t,
        162 as ::core::ffi::c_int as uint8_t,
        193 as ::core::ffi::c_int as uint8_t,
        209 as ::core::ffi::c_int as uint8_t,
        44 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        76 as ::core::ffi::c_int as uint8_t,
        140 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        190 as ::core::ffi::c_int as uint8_t,
        254 as ::core::ffi::c_int as uint8_t,
        222 as ::core::ffi::c_int as uint8_t,
        238 as ::core::ffi::c_int as uint8_t,
        126 as ::core::ffi::c_int as uint8_t,
        94 as ::core::ffi::c_int as uint8_t,
        157 as ::core::ffi::c_int as uint8_t,
        157 as ::core::ffi::c_int as uint8_t,
        109 as ::core::ffi::c_int as uint8_t,
        61 as ::core::ffi::c_int as uint8_t,
        173 as ::core::ffi::c_int as uint8_t,
        205 as ::core::ffi::c_int as uint8_t,
    ];
    static mut tab33: [uint8_t; 16] = [
        252 as ::core::ffi::c_int as uint8_t,
        236 as ::core::ffi::c_int as uint8_t,
        220 as ::core::ffi::c_int as uint8_t,
        204 as ::core::ffi::c_int as uint8_t,
        188 as ::core::ffi::c_int as uint8_t,
        172 as ::core::ffi::c_int as uint8_t,
        156 as ::core::ffi::c_int as uint8_t,
        140 as ::core::ffi::c_int as uint8_t,
        124 as ::core::ffi::c_int as uint8_t,
        108 as ::core::ffi::c_int as uint8_t,
        92 as ::core::ffi::c_int as uint8_t,
        76 as ::core::ffi::c_int as uint8_t,
        60 as ::core::ffi::c_int as uint8_t,
        44 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
    ];
    static mut tabindex: [int16_t; 32] = [
        0 as ::core::ffi::c_int as int16_t,
        32 as ::core::ffi::c_int as int16_t,
        64 as ::core::ffi::c_int as int16_t,
        98 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        132 as ::core::ffi::c_int as int16_t,
        180 as ::core::ffi::c_int as int16_t,
        218 as ::core::ffi::c_int as int16_t,
        292 as ::core::ffi::c_int as int16_t,
        364 as ::core::ffi::c_int as int16_t,
        426 as ::core::ffi::c_int as int16_t,
        538 as ::core::ffi::c_int as int16_t,
        648 as ::core::ffi::c_int as int16_t,
        746 as ::core::ffi::c_int as int16_t,
        0 as ::core::ffi::c_int as int16_t,
        1126 as ::core::ffi::c_int as int16_t,
        1460 as ::core::ffi::c_int as int16_t,
        1460 as ::core::ffi::c_int as int16_t,
        1460 as ::core::ffi::c_int as int16_t,
        1460 as ::core::ffi::c_int as int16_t,
        1460 as ::core::ffi::c_int as int16_t,
        1460 as ::core::ffi::c_int as int16_t,
        1460 as ::core::ffi::c_int as int16_t,
        1460 as ::core::ffi::c_int as int16_t,
        1842 as ::core::ffi::c_int as int16_t,
        1842 as ::core::ffi::c_int as int16_t,
        1842 as ::core::ffi::c_int as int16_t,
        1842 as ::core::ffi::c_int as int16_t,
        1842 as ::core::ffi::c_int as int16_t,
        1842 as ::core::ffi::c_int as int16_t,
        1842 as ::core::ffi::c_int as int16_t,
        1842 as ::core::ffi::c_int as int16_t,
    ];
    static mut g_linbits: [uint8_t; 32] = [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
    ];
    let mut one: ::core::ffi::c_float = 0.0f32;
    let mut ireg: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut big_val_cnt: ::core::ffi::c_int = (*gr_info).big_values
        as ::core::ffi::c_int;
    let mut sfb: *const uint8_t = (*gr_info).sfbtab;
    let mut bs_next_ptr: *const uint8_t = (*bs)
        .buf
        .offset(((*bs).pos / 8 as ::core::ffi::c_int) as isize);
    let mut bs_cache: uint32_t = (*bs_next_ptr.offset(0 as ::core::ffi::c_int as isize)
        as uint32_t)
        .wrapping_mul(256 as uint32_t)
        .wrapping_add(*bs_next_ptr.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
        .wrapping_mul(256 as uint32_t)
        .wrapping_add(*bs_next_ptr.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
        .wrapping_mul(256 as uint32_t)
        .wrapping_add(*bs_next_ptr.offset(3 as ::core::ffi::c_int as isize) as uint32_t)
        << ((*bs).pos & 7 as ::core::ffi::c_int);
    let mut pairs_to_decode: ::core::ffi::c_int = 0;
    let mut np: ::core::ffi::c_int = 0;
    let mut bs_sh: ::core::ffi::c_int = ((*bs).pos & 7 as ::core::ffi::c_int)
        - 8 as ::core::ffi::c_int;
    bs_next_ptr = bs_next_ptr.offset(4 as ::core::ffi::c_int as isize);
    while big_val_cnt > 0 as ::core::ffi::c_int {
        let mut tab_num: ::core::ffi::c_int = (*gr_info).table_select[ireg as usize]
            as ::core::ffi::c_int;
        let fresh21 = ireg;
        ireg = ireg + 1;
        let mut sfb_cnt: ::core::ffi::c_int = (*gr_info).region_count[fresh21 as usize]
            as ::core::ffi::c_int;
        let mut codebook: *const int16_t = (&raw const tabs as *const int16_t)
            .offset(tabindex[tab_num as usize] as ::core::ffi::c_int as isize);
        let mut linbits: ::core::ffi::c_int = g_linbits[tab_num as usize]
            as ::core::ffi::c_int;
        if linbits != 0 {
            loop {
                let fresh22 = sfb;
                sfb = sfb.offset(1);
                np = *fresh22 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
                pairs_to_decode = if big_val_cnt > np { np } else { big_val_cnt };
                let fresh23 = scf;
                scf = scf.offset(1);
                one = *fresh23;
                loop {
                    let mut j: ::core::ffi::c_int = 0;
                    let mut w: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
                    let mut leaf: ::core::ffi::c_int = *codebook
                        .offset((bs_cache >> 32 as ::core::ffi::c_int - w) as isize)
                        as ::core::ffi::c_int;
                    while leaf < 0 as ::core::ffi::c_int {
                        bs_cache <<= w;
                        bs_sh += w;
                        w = leaf & 7 as ::core::ffi::c_int;
                        leaf = *codebook
                            .offset(
                                (bs_cache >> 32 as ::core::ffi::c_int - w)
                                    .wrapping_sub((leaf >> 3 as ::core::ffi::c_int) as uint32_t)
                                    as isize,
                            ) as ::core::ffi::c_int;
                    }
                    bs_cache <<= leaf >> 8 as ::core::ffi::c_int;
                    bs_sh += leaf >> 8 as ::core::ffi::c_int;
                    j = 0 as ::core::ffi::c_int;
                    while j < 2 as ::core::ffi::c_int {
                        let mut lsb: ::core::ffi::c_int = leaf
                            & 0xf as ::core::ffi::c_int;
                        if lsb == 15 as ::core::ffi::c_int {
                            lsb = (lsb as uint32_t)
                                .wrapping_add(
                                    bs_cache >> 32 as ::core::ffi::c_int - linbits,
                                ) as ::core::ffi::c_int as ::core::ffi::c_int;
                            bs_cache <<= linbits;
                            bs_sh += linbits;
                            while bs_sh >= 0 as ::core::ffi::c_int {
                                let fresh24 = bs_next_ptr;
                                bs_next_ptr = bs_next_ptr.offset(1);
                                bs_cache |= (*fresh24 as uint32_t) << bs_sh;
                                bs_sh -= 8 as ::core::ffi::c_int;
                            }
                            *dst = one * L3_pow_43(lsb)
                                * (if (bs_cache as int32_t) < 0 as int32_t {
                                    -(1 as ::core::ffi::c_int)
                                } else {
                                    1 as ::core::ffi::c_int
                                }) as ::core::ffi::c_float;
                        } else {
                            *dst = g_pow43[((16 as ::core::ffi::c_int + lsb) as uint32_t)
                                .wrapping_sub(
                                    (16 as uint32_t)
                                        .wrapping_mul(bs_cache >> 31 as ::core::ffi::c_int),
                                ) as usize] * one;
                        }
                        bs_cache
                            <<= if lsb != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            };
                        bs_sh
                            += if lsb != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            };
                        j += 1;
                        dst = dst.offset(1);
                        leaf >>= 4 as ::core::ffi::c_int;
                    }
                    while bs_sh >= 0 as ::core::ffi::c_int {
                        let fresh25 = bs_next_ptr;
                        bs_next_ptr = bs_next_ptr.offset(1);
                        bs_cache |= (*fresh25 as uint32_t) << bs_sh;
                        bs_sh -= 8 as ::core::ffi::c_int;
                    }
                    pairs_to_decode -= 1;
                    if !(pairs_to_decode != 0) {
                        break;
                    }
                }
                big_val_cnt -= np;
                if !(big_val_cnt > 0 as ::core::ffi::c_int
                    && {
                        sfb_cnt -= 1;
                        sfb_cnt >= 0 as ::core::ffi::c_int
                    })
                {
                    break;
                }
            }
        } else {
            loop {
                let fresh26 = sfb;
                sfb = sfb.offset(1);
                np = *fresh26 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
                pairs_to_decode = if big_val_cnt > np { np } else { big_val_cnt };
                let fresh27 = scf;
                scf = scf.offset(1);
                one = *fresh27;
                loop {
                    let mut j_0: ::core::ffi::c_int = 0;
                    let mut w_0: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
                    let mut leaf_0: ::core::ffi::c_int = *codebook
                        .offset((bs_cache >> 32 as ::core::ffi::c_int - w_0) as isize)
                        as ::core::ffi::c_int;
                    while leaf_0 < 0 as ::core::ffi::c_int {
                        bs_cache <<= w_0;
                        bs_sh += w_0;
                        w_0 = leaf_0 & 7 as ::core::ffi::c_int;
                        leaf_0 = *codebook
                            .offset(
                                (bs_cache >> 32 as ::core::ffi::c_int - w_0)
                                    .wrapping_sub(
                                        (leaf_0 >> 3 as ::core::ffi::c_int) as uint32_t,
                                    ) as isize,
                            ) as ::core::ffi::c_int;
                    }
                    bs_cache <<= leaf_0 >> 8 as ::core::ffi::c_int;
                    bs_sh += leaf_0 >> 8 as ::core::ffi::c_int;
                    j_0 = 0 as ::core::ffi::c_int;
                    while j_0 < 2 as ::core::ffi::c_int {
                        let mut lsb_0: ::core::ffi::c_int = leaf_0
                            & 0xf as ::core::ffi::c_int;
                        *dst = g_pow43[((16 as ::core::ffi::c_int + lsb_0) as uint32_t)
                            .wrapping_sub(
                                (16 as uint32_t)
                                    .wrapping_mul(bs_cache >> 31 as ::core::ffi::c_int),
                            ) as usize] * one;
                        bs_cache
                            <<= if lsb_0 != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            };
                        bs_sh
                            += if lsb_0 != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            };
                        j_0 += 1;
                        dst = dst.offset(1);
                        leaf_0 >>= 4 as ::core::ffi::c_int;
                    }
                    while bs_sh >= 0 as ::core::ffi::c_int {
                        let fresh28 = bs_next_ptr;
                        bs_next_ptr = bs_next_ptr.offset(1);
                        bs_cache |= (*fresh28 as uint32_t) << bs_sh;
                        bs_sh -= 8 as ::core::ffi::c_int;
                    }
                    pairs_to_decode -= 1;
                    if !(pairs_to_decode != 0) {
                        break;
                    }
                }
                big_val_cnt -= np;
                if !(big_val_cnt > 0 as ::core::ffi::c_int
                    && {
                        sfb_cnt -= 1;
                        sfb_cnt >= 0 as ::core::ffi::c_int
                    })
                {
                    break;
                }
            }
        }
    }
    np = 1 as ::core::ffi::c_int - big_val_cnt;
    loop {
        let mut codebook_count1: *const uint8_t = if (*gr_info).count1_table
            as ::core::ffi::c_int != 0
        {
            &raw const tab33 as *const uint8_t
        } else {
            &raw const tab32 as *const uint8_t
        };
        let mut leaf_1: ::core::ffi::c_int = *codebook_count1
            .offset(
                (bs_cache >> 32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int;
        if leaf_1 & 8 as ::core::ffi::c_int == 0 {
            leaf_1 = *codebook_count1
                .offset(
                    ((leaf_1 >> 3 as ::core::ffi::c_int) as uint32_t)
                        .wrapping_add(
                            bs_cache << 4 as ::core::ffi::c_int
                                >> 32 as ::core::ffi::c_int
                                    - (leaf_1 & 3 as ::core::ffi::c_int),
                        ) as isize,
                ) as ::core::ffi::c_int;
        }
        bs_cache <<= leaf_1 & 7 as ::core::ffi::c_int;
        bs_sh += leaf_1 & 7 as ::core::ffi::c_int;
        if bs_next_ptr.offset_from((*bs).buf) as ::core::ffi::c_long
            * 8 as ::core::ffi::c_long - 24 as ::core::ffi::c_long
            + bs_sh as ::core::ffi::c_long > layer3gr_limit as ::core::ffi::c_long
        {
            break;
        }
        np -= 1;
        if np == 0 {
            let fresh29 = sfb;
            sfb = sfb.offset(1);
            np = *fresh29 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
            if np == 0 {
                break;
            }
            let fresh30 = scf;
            scf = scf.offset(1);
            one = *fresh30;
        }
        if leaf_1 & 128 as ::core::ffi::c_int >> 0 as ::core::ffi::c_int != 0 {
            *dst.offset(0 as ::core::ffi::c_int as isize) = if (bs_cache as int32_t)
                < 0 as int32_t
            {
                -one
            } else {
                one
            };
            bs_cache <<= 1 as ::core::ffi::c_int;
            bs_sh += 1 as ::core::ffi::c_int;
        }
        if leaf_1 & 128 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int != 0 {
            *dst.offset(1 as ::core::ffi::c_int as isize) = if (bs_cache as int32_t)
                < 0 as int32_t
            {
                -one
            } else {
                one
            };
            bs_cache <<= 1 as ::core::ffi::c_int;
            bs_sh += 1 as ::core::ffi::c_int;
        }
        np -= 1;
        if np == 0 {
            let fresh31 = sfb;
            sfb = sfb.offset(1);
            np = *fresh31 as ::core::ffi::c_int / 2 as ::core::ffi::c_int;
            if np == 0 {
                break;
            }
            let fresh32 = scf;
            scf = scf.offset(1);
            one = *fresh32;
        }
        if leaf_1 & 128 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int != 0 {
            *dst.offset(2 as ::core::ffi::c_int as isize) = if (bs_cache as int32_t)
                < 0 as int32_t
            {
                -one
            } else {
                one
            };
            bs_cache <<= 1 as ::core::ffi::c_int;
            bs_sh += 1 as ::core::ffi::c_int;
        }
        if leaf_1 & 128 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int != 0 {
            *dst.offset(3 as ::core::ffi::c_int as isize) = if (bs_cache as int32_t)
                < 0 as int32_t
            {
                -one
            } else {
                one
            };
            bs_cache <<= 1 as ::core::ffi::c_int;
            bs_sh += 1 as ::core::ffi::c_int;
        }
        while bs_sh >= 0 as ::core::ffi::c_int {
            let fresh33 = bs_next_ptr;
            bs_next_ptr = bs_next_ptr.offset(1);
            bs_cache |= (*fresh33 as uint32_t) << bs_sh;
            bs_sh -= 8 as ::core::ffi::c_int;
        }
        dst = dst.offset(4 as ::core::ffi::c_int as isize);
    }
    (*bs).pos = layer3gr_limit;
}
unsafe extern "C" fn L3_midside_stereo(
    mut left: *mut ::core::ffi::c_float,
    mut n: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut right: *mut ::core::ffi::c_float = left
        .offset(576 as ::core::ffi::c_int as isize);
    while i < n {
        let mut a: ::core::ffi::c_float = *left.offset(i as isize);
        let mut b: ::core::ffi::c_float = *right.offset(i as isize);
        *left.offset(i as isize) = a + b;
        *right.offset(i as isize) = a - b;
        i += 1;
    }
}
unsafe extern "C" fn L3_intensity_stereo_band(
    mut left: *mut ::core::ffi::c_float,
    mut n: ::core::ffi::c_int,
    mut kl: ::core::ffi::c_float,
    mut kr: ::core::ffi::c_float,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < n {
        *left.offset((i + 576 as ::core::ffi::c_int) as isize) = *left.offset(i as isize)
            * kr;
        *left.offset(i as isize) = *left.offset(i as isize) * kl;
        i += 1;
    }
}
unsafe extern "C" fn L3_stereo_top_band(
    mut right: *const ::core::ffi::c_float,
    mut sfb: *const uint8_t,
    mut nbands: ::core::ffi::c_int,
    mut max_band: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    let ref mut fresh19 = *max_band.offset(2 as ::core::ffi::c_int as isize);
    *fresh19 = -(1 as ::core::ffi::c_int);
    let ref mut fresh20 = *max_band.offset(1 as ::core::ffi::c_int as isize);
    *fresh20 = *fresh19;
    *max_band.offset(0 as ::core::ffi::c_int as isize) = *fresh20;
    i = 0 as ::core::ffi::c_int;
    while i < nbands {
        k = 0 as ::core::ffi::c_int;
        while k < *sfb.offset(i as isize) as ::core::ffi::c_int {
            if *right.offset(k as isize)
                != 0 as ::core::ffi::c_int as ::core::ffi::c_float
                || *right.offset((k + 1 as ::core::ffi::c_int) as isize)
                    != 0 as ::core::ffi::c_int as ::core::ffi::c_float
            {
                *max_band.offset((i % 3 as ::core::ffi::c_int) as isize) = i;
                break;
            } else {
                k += 2 as ::core::ffi::c_int;
            }
        }
        right = right.offset(*sfb.offset(i as isize) as ::core::ffi::c_int as isize);
        i += 1;
    }
}
unsafe extern "C" fn L3_stereo_process(
    mut left: *mut ::core::ffi::c_float,
    mut ist_pos: *const uint8_t,
    mut sfb: *const uint8_t,
    mut hdr: *const uint8_t,
    mut max_band: *mut ::core::ffi::c_int,
    mut mpeg2_sh: ::core::ffi::c_int,
) {
    static mut g_pan: [::core::ffi::c_float; 14] = [
        0 as ::core::ffi::c_int as ::core::ffi::c_float,
        1 as ::core::ffi::c_int as ::core::ffi::c_float,
        0.21132487f32,
        0.78867513f32,
        0.36602540f32,
        0.63397460f32,
        0.5f32,
        0.5f32,
        0.63397460f32,
        0.36602540f32,
        0.78867513f32,
        0.21132487f32,
        1 as ::core::ffi::c_int as ::core::ffi::c_float,
        0 as ::core::ffi::c_int as ::core::ffi::c_float,
    ];
    let mut i: ::core::ffi::c_uint = 0;
    let mut max_pos: ::core::ffi::c_uint = (if *hdr
        .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 0x8 as ::core::ffi::c_int != 0
    {
        7 as ::core::ffi::c_int
    } else {
        64 as ::core::ffi::c_int
    }) as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_uint;
    while *sfb.offset(i as isize) != 0 {
        let mut ipos: ::core::ffi::c_uint = *ist_pos.offset(i as isize)
            as ::core::ffi::c_uint;
        if i as ::core::ffi::c_int
            > *max_band.offset(i.wrapping_rem(3 as ::core::ffi::c_uint) as isize)
            && ipos < max_pos
        {
            let mut kl: ::core::ffi::c_float = 0.;
            let mut kr: ::core::ffi::c_float = 0.;
            let mut s: ::core::ffi::c_float = if *hdr
                .offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x20 as ::core::ffi::c_int != 0
            {
                1.41421356f32
            } else {
                1 as ::core::ffi::c_int as ::core::ffi::c_float
            };
            if *hdr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x8 as ::core::ffi::c_int != 0
            {
                kl = g_pan[(2 as ::core::ffi::c_uint).wrapping_mul(ipos) as usize];
                kr = g_pan[(2 as ::core::ffi::c_uint)
                    .wrapping_mul(ipos)
                    .wrapping_add(1 as ::core::ffi::c_uint) as usize];
            } else {
                kl = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
                kr = L3_ldexp_q2(
                    1 as ::core::ffi::c_int as ::core::ffi::c_float,
                    ((ipos.wrapping_add(1 as ::core::ffi::c_uint)
                        >> 1 as ::core::ffi::c_int) << mpeg2_sh) as ::core::ffi::c_int,
                );
                if ipos & 1 as ::core::ffi::c_uint != 0 {
                    kl = kr;
                    kr = 1 as ::core::ffi::c_int as ::core::ffi::c_float;
                }
            }
            L3_intensity_stereo_band(
                left,
                *sfb.offset(i as isize) as ::core::ffi::c_int,
                kl * s,
                kr * s,
            );
        } else if *hdr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x20 as ::core::ffi::c_int != 0
        {
            L3_midside_stereo(left, *sfb.offset(i as isize) as ::core::ffi::c_int);
        }
        left = left.offset(*sfb.offset(i as isize) as ::core::ffi::c_int as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn L3_intensity_stereo(
    mut left: *mut ::core::ffi::c_float,
    mut ist_pos: *mut uint8_t,
    mut gr: *const L3_gr_info_t,
    mut hdr: *const uint8_t,
) {
    let mut max_band: [::core::ffi::c_int; 3] = [0; 3];
    let mut n_sfb: ::core::ffi::c_int = (*gr).n_long_sfb as ::core::ffi::c_int
        + (*gr).n_short_sfb as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut max_blocks: ::core::ffi::c_int = if (*gr).n_short_sfb as ::core::ffi::c_int
        != 0
    {
        3 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    L3_stereo_top_band(
        left.offset(576 as ::core::ffi::c_int as isize),
        (*gr).sfbtab,
        n_sfb,
        &raw mut max_band as *mut ::core::ffi::c_int,
    );
    if (*gr).n_long_sfb != 0 {
        max_band[2 as ::core::ffi::c_int as usize] = if (if max_band[0
            as ::core::ffi::c_int as usize] < max_band[1 as ::core::ffi::c_int as usize]
        {
            max_band[1 as ::core::ffi::c_int as usize]
        } else {
            max_band[0 as ::core::ffi::c_int as usize]
        }) < max_band[2 as ::core::ffi::c_int as usize]
        {
            max_band[2 as ::core::ffi::c_int as usize]
        } else if max_band[0 as ::core::ffi::c_int as usize]
            < max_band[1 as ::core::ffi::c_int as usize]
        {
            max_band[1 as ::core::ffi::c_int as usize]
        } else {
            max_band[0 as ::core::ffi::c_int as usize]
        };
        max_band[1 as ::core::ffi::c_int as usize] = max_band[2 as ::core::ffi::c_int
            as usize];
        max_band[0 as ::core::ffi::c_int as usize] = max_band[1 as ::core::ffi::c_int
            as usize];
    }
    i = 0 as ::core::ffi::c_int;
    while i < max_blocks {
        let mut default_pos: ::core::ffi::c_int = if *hdr
            .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x8 as ::core::ffi::c_int != 0
        {
            3 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut itop: ::core::ffi::c_int = n_sfb - max_blocks + i;
        let mut prev: ::core::ffi::c_int = itop - max_blocks;
        *ist_pos.offset(itop as isize) = (if max_band[i as usize] >= prev {
            default_pos
        } else {
            *ist_pos.offset(prev as isize) as ::core::ffi::c_int
        }) as uint8_t;
        i += 1;
    }
    L3_stereo_process(
        left,
        ist_pos,
        (*gr).sfbtab,
        hdr,
        &raw mut max_band as *mut ::core::ffi::c_int,
        (*gr.offset(1 as ::core::ffi::c_int as isize)).scalefac_compress
            as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn L3_reorder(
    mut grbuf: *mut ::core::ffi::c_float,
    mut scratch: *mut ::core::ffi::c_float,
    mut sfb: *const uint8_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut src: *mut ::core::ffi::c_float = grbuf;
    let mut dst: *mut ::core::ffi::c_float = scratch;
    loop {
        len = *sfb as ::core::ffi::c_int;
        if !(0 as ::core::ffi::c_int != len) {
            break;
        }
        i = 0 as ::core::ffi::c_int;
        while i < len {
            let fresh16 = dst;
            dst = dst.offset(1);
            *fresh16 = *src.offset((0 as ::core::ffi::c_int * len) as isize);
            let fresh17 = dst;
            dst = dst.offset(1);
            *fresh17 = *src.offset((1 as ::core::ffi::c_int * len) as isize);
            let fresh18 = dst;
            dst = dst.offset(1);
            *fresh18 = *src.offset((2 as ::core::ffi::c_int * len) as isize);
            i += 1;
            src = src.offset(1);
        }
        sfb = sfb.offset(3 as ::core::ffi::c_int as isize);
        src = src.offset((2 as ::core::ffi::c_int * len) as isize);
    }
    memcpy(
        grbuf as *mut ::core::ffi::c_void,
        scratch as *const ::core::ffi::c_void,
        (dst.offset_from(scratch) as ::core::ffi::c_long as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t),
    );
}
unsafe extern "C" fn L3_antialias(
    mut grbuf: *mut ::core::ffi::c_float,
    mut nbands: ::core::ffi::c_int,
) {
    static mut g_aa: [[::core::ffi::c_float; 8]; 2] = [
        [
            0.85749293f32,
            0.88174200f32,
            0.94962865f32,
            0.98331459f32,
            0.99551782f32,
            0.99916056f32,
            0.99989920f32,
            0.99999316f32,
        ],
        [
            0.51449576f32,
            0.47173197f32,
            0.31337745f32,
            0.18191320f32,
            0.09457419f32,
            0.04096558f32,
            0.01419856f32,
            0.00369997f32,
        ],
    ];
    while nbands > 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut u: ::core::ffi::c_float = *grbuf
                .offset((18 as ::core::ffi::c_int + i) as isize);
            let mut d: ::core::ffi::c_float = *grbuf
                .offset((17 as ::core::ffi::c_int - i) as isize);
            *grbuf.offset((18 as ::core::ffi::c_int + i) as isize) = u
                * g_aa[0 as ::core::ffi::c_int as usize][i as usize]
                - d * g_aa[1 as ::core::ffi::c_int as usize][i as usize];
            *grbuf.offset((17 as ::core::ffi::c_int - i) as isize) = u
                * g_aa[1 as ::core::ffi::c_int as usize][i as usize]
                + d * g_aa[0 as ::core::ffi::c_int as usize][i as usize];
            i += 1;
        }
        nbands -= 1;
        grbuf = grbuf.offset(18 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn L3_dct3_9(mut y: *mut ::core::ffi::c_float) {
    let mut s0: ::core::ffi::c_float = 0.;
    let mut s1: ::core::ffi::c_float = 0.;
    let mut s2: ::core::ffi::c_float = 0.;
    let mut s3: ::core::ffi::c_float = 0.;
    let mut s4: ::core::ffi::c_float = 0.;
    let mut s5: ::core::ffi::c_float = 0.;
    let mut s6: ::core::ffi::c_float = 0.;
    let mut s7: ::core::ffi::c_float = 0.;
    let mut s8: ::core::ffi::c_float = 0.;
    let mut t0: ::core::ffi::c_float = 0.;
    let mut t2: ::core::ffi::c_float = 0.;
    let mut t4: ::core::ffi::c_float = 0.;
    s0 = *y.offset(0 as ::core::ffi::c_int as isize);
    s2 = *y.offset(2 as ::core::ffi::c_int as isize);
    s4 = *y.offset(4 as ::core::ffi::c_int as isize);
    s6 = *y.offset(6 as ::core::ffi::c_int as isize);
    s8 = *y.offset(8 as ::core::ffi::c_int as isize);
    t0 = s0 + s6 * 0.5f32;
    s0 -= s6;
    t4 = (s4 + s2) * 0.93969262f32;
    t2 = (s8 + s2) * 0.76604444f32;
    s6 = (s4 - s8) * 0.17364818f32;
    s4 += s8 - s2;
    s2 = s0 - s4 * 0.5f32;
    *y.offset(4 as ::core::ffi::c_int as isize) = s4 + s0;
    s8 = t0 - t2 + s6;
    s0 = t0 - t4 + t2;
    s4 = t0 + t4 - s6;
    s1 = *y.offset(1 as ::core::ffi::c_int as isize);
    s3 = *y.offset(3 as ::core::ffi::c_int as isize);
    s5 = *y.offset(5 as ::core::ffi::c_int as isize);
    s7 = *y.offset(7 as ::core::ffi::c_int as isize);
    s3 *= 0.86602540f32;
    t0 = (s5 + s1) * 0.98480775f32;
    t4 = (s5 - s7) * 0.34202014f32;
    t2 = (s1 + s7) * 0.64278761f32;
    s1 = (s1 - s5 - s7) * 0.86602540f32;
    s5 = t0 - s3 - t2;
    s7 = t4 - s3 - t0;
    s3 = t4 + s3 - t2;
    *y.offset(0 as ::core::ffi::c_int as isize) = s4 - s7;
    *y.offset(1 as ::core::ffi::c_int as isize) = s2 + s1;
    *y.offset(2 as ::core::ffi::c_int as isize) = s0 - s3;
    *y.offset(3 as ::core::ffi::c_int as isize) = s8 + s5;
    *y.offset(5 as ::core::ffi::c_int as isize) = s8 - s5;
    *y.offset(6 as ::core::ffi::c_int as isize) = s0 + s3;
    *y.offset(7 as ::core::ffi::c_int as isize) = s2 - s1;
    *y.offset(8 as ::core::ffi::c_int as isize) = s4 + s7;
}
unsafe extern "C" fn L3_imdct36(
    mut grbuf: *mut ::core::ffi::c_float,
    mut overlap: *mut ::core::ffi::c_float,
    mut window: *const ::core::ffi::c_float,
    mut nbands: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    static mut g_twid9: [::core::ffi::c_float; 18] = [
        0.73727734f32,
        0.79335334f32,
        0.84339145f32,
        0.88701083f32,
        0.92387953f32,
        0.95371695f32,
        0.97629601f32,
        0.99144486f32,
        0.99904822f32,
        0.67559021f32,
        0.60876143f32,
        0.53729961f32,
        0.46174861f32,
        0.38268343f32,
        0.30070580f32,
        0.21643961f32,
        0.13052619f32,
        0.04361938f32,
    ];
    j = 0 as ::core::ffi::c_int;
    while j < nbands {
        let mut co: [::core::ffi::c_float; 9] = [0.; 9];
        let mut si: [::core::ffi::c_float; 9] = [0.; 9];
        co[0 as ::core::ffi::c_int as usize] = -*grbuf
            .offset(0 as ::core::ffi::c_int as isize);
        si[0 as ::core::ffi::c_int as usize] = *grbuf
            .offset(17 as ::core::ffi::c_int as isize);
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            si[(8 as ::core::ffi::c_int - 2 as ::core::ffi::c_int * i) as usize] = *grbuf
                .offset((4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
                - *grbuf
                    .offset(
                        (4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as isize,
                    );
            co[(1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i) as usize] = *grbuf
                .offset((4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
                + *grbuf
                    .offset(
                        (4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as isize,
                    );
            si[(7 as ::core::ffi::c_int - 2 as ::core::ffi::c_int * i) as usize] = *grbuf
                .offset((4 as ::core::ffi::c_int * i + 4 as ::core::ffi::c_int) as isize)
                - *grbuf
                    .offset(
                        (4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int) as isize,
                    );
            co[(2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i) as usize] = -(*grbuf
                .offset((4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int) as isize)
                + *grbuf
                    .offset(
                        (4 as ::core::ffi::c_int * i + 4 as ::core::ffi::c_int) as isize,
                    ));
            i += 1;
        }
        L3_dct3_9(&raw mut co as *mut ::core::ffi::c_float);
        L3_dct3_9(&raw mut si as *mut ::core::ffi::c_float);
        si[1 as ::core::ffi::c_int as usize] = -si[1 as ::core::ffi::c_int as usize];
        si[3 as ::core::ffi::c_int as usize] = -si[3 as ::core::ffi::c_int as usize];
        si[5 as ::core::ffi::c_int as usize] = -si[5 as ::core::ffi::c_int as usize];
        si[7 as ::core::ffi::c_int as usize] = -si[7 as ::core::ffi::c_int as usize];
        i = 0 as ::core::ffi::c_int;
        while i < 9 as ::core::ffi::c_int {
            let mut ovl: ::core::ffi::c_float = *overlap.offset(i as isize);
            let mut sum: ::core::ffi::c_float = co[i as usize]
                * g_twid9[(9 as ::core::ffi::c_int + i) as usize]
                + si[i as usize] * g_twid9[(0 as ::core::ffi::c_int + i) as usize];
            *overlap.offset(i as isize) = co[i as usize]
                * g_twid9[(0 as ::core::ffi::c_int + i) as usize]
                - si[i as usize] * g_twid9[(9 as ::core::ffi::c_int + i) as usize];
            *grbuf.offset(i as isize) = ovl
                * *window.offset((0 as ::core::ffi::c_int + i) as isize)
                - sum * *window.offset((9 as ::core::ffi::c_int + i) as isize);
            *grbuf.offset((17 as ::core::ffi::c_int - i) as isize) = ovl
                * *window.offset((9 as ::core::ffi::c_int + i) as isize)
                + sum * *window.offset((0 as ::core::ffi::c_int + i) as isize);
            i += 1;
        }
        j += 1;
        grbuf = grbuf.offset(18 as ::core::ffi::c_int as isize);
        overlap = overlap.offset(9 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn L3_idct3(
    mut x0: ::core::ffi::c_float,
    mut x1: ::core::ffi::c_float,
    mut x2: ::core::ffi::c_float,
    mut dst: *mut ::core::ffi::c_float,
) {
    let mut m1: ::core::ffi::c_float = x1 * 0.86602540f32;
    let mut a1: ::core::ffi::c_float = x0 - x2 * 0.5f32;
    *dst.offset(1 as ::core::ffi::c_int as isize) = x0 + x2;
    *dst.offset(0 as ::core::ffi::c_int as isize) = a1 + m1;
    *dst.offset(2 as ::core::ffi::c_int as isize) = a1 - m1;
}
unsafe extern "C" fn L3_imdct12(
    mut x: *mut ::core::ffi::c_float,
    mut dst: *mut ::core::ffi::c_float,
    mut overlap: *mut ::core::ffi::c_float,
) {
    static mut g_twid3: [::core::ffi::c_float; 6] = [
        0.79335334f32,
        0.92387953f32,
        0.99144486f32,
        0.60876143f32,
        0.38268343f32,
        0.13052619f32,
    ];
    let mut co: [::core::ffi::c_float; 3] = [0.; 3];
    let mut si: [::core::ffi::c_float; 3] = [0.; 3];
    let mut i: ::core::ffi::c_int = 0;
    L3_idct3(
        -*x.offset(0 as ::core::ffi::c_int as isize),
        *x.offset(6 as ::core::ffi::c_int as isize)
            + *x.offset(3 as ::core::ffi::c_int as isize),
        *x.offset(12 as ::core::ffi::c_int as isize)
            + *x.offset(9 as ::core::ffi::c_int as isize),
        &raw mut co as *mut ::core::ffi::c_float,
    );
    L3_idct3(
        *x.offset(15 as ::core::ffi::c_int as isize),
        *x.offset(12 as ::core::ffi::c_int as isize)
            - *x.offset(9 as ::core::ffi::c_int as isize),
        *x.offset(6 as ::core::ffi::c_int as isize)
            - *x.offset(3 as ::core::ffi::c_int as isize),
        &raw mut si as *mut ::core::ffi::c_float,
    );
    si[1 as ::core::ffi::c_int as usize] = -si[1 as ::core::ffi::c_int as usize];
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        let mut ovl: ::core::ffi::c_float = *overlap.offset(i as isize);
        let mut sum: ::core::ffi::c_float = co[i as usize]
            * g_twid3[(3 as ::core::ffi::c_int + i) as usize]
            + si[i as usize] * g_twid3[(0 as ::core::ffi::c_int + i) as usize];
        *overlap.offset(i as isize) = co[i as usize]
            * g_twid3[(0 as ::core::ffi::c_int + i) as usize]
            - si[i as usize] * g_twid3[(3 as ::core::ffi::c_int + i) as usize];
        *dst.offset(i as isize) = ovl * g_twid3[(2 as ::core::ffi::c_int - i) as usize]
            - sum * g_twid3[(5 as ::core::ffi::c_int - i) as usize];
        *dst.offset((5 as ::core::ffi::c_int - i) as isize) = ovl
            * g_twid3[(5 as ::core::ffi::c_int - i) as usize]
            + sum * g_twid3[(2 as ::core::ffi::c_int - i) as usize];
        i += 1;
    }
}
unsafe extern "C" fn L3_imdct_short(
    mut grbuf: *mut ::core::ffi::c_float,
    mut overlap: *mut ::core::ffi::c_float,
    mut nbands: ::core::ffi::c_int,
) {
    while nbands > 0 as ::core::ffi::c_int {
        let mut tmp: [::core::ffi::c_float; 18] = [0.; 18];
        memcpy(
            &raw mut tmp as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
            grbuf as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_float; 18]>() as size_t,
        );
        memcpy(
            grbuf as *mut ::core::ffi::c_void,
            overlap as *const ::core::ffi::c_void,
            (6 as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_float>() as size_t),
        );
        L3_imdct12(
            &raw mut tmp as *mut ::core::ffi::c_float,
            grbuf.offset(6 as ::core::ffi::c_int as isize),
            overlap.offset(6 as ::core::ffi::c_int as isize),
        );
        L3_imdct12(
            (&raw mut tmp as *mut ::core::ffi::c_float)
                .offset(1 as ::core::ffi::c_int as isize),
            grbuf.offset(12 as ::core::ffi::c_int as isize),
            overlap.offset(6 as ::core::ffi::c_int as isize),
        );
        L3_imdct12(
            (&raw mut tmp as *mut ::core::ffi::c_float)
                .offset(2 as ::core::ffi::c_int as isize),
            overlap,
            overlap.offset(6 as ::core::ffi::c_int as isize),
        );
        nbands -= 1;
        overlap = overlap.offset(9 as ::core::ffi::c_int as isize);
        grbuf = grbuf.offset(18 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn L3_change_sign(mut grbuf: *mut ::core::ffi::c_float) {
    let mut b: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    b = 0 as ::core::ffi::c_int;
    grbuf = grbuf.offset(18 as ::core::ffi::c_int as isize);
    while b < 32 as ::core::ffi::c_int {
        i = 1 as ::core::ffi::c_int;
        while i < 18 as ::core::ffi::c_int {
            *grbuf.offset(i as isize) = -*grbuf.offset(i as isize);
            i += 2 as ::core::ffi::c_int;
        }
        b += 2 as ::core::ffi::c_int;
        grbuf = grbuf.offset(36 as ::core::ffi::c_int as isize);
    }
}
unsafe extern "C" fn L3_imdct_gr(
    mut grbuf: *mut ::core::ffi::c_float,
    mut overlap: *mut ::core::ffi::c_float,
    mut block_type: ::core::ffi::c_uint,
    mut n_long_bands: ::core::ffi::c_uint,
) {
    static mut g_mdct_window: [[::core::ffi::c_float; 18]; 2] = [
        [
            0.99904822f32,
            0.99144486f32,
            0.97629601f32,
            0.95371695f32,
            0.92387953f32,
            0.88701083f32,
            0.84339145f32,
            0.79335334f32,
            0.73727734f32,
            0.04361938f32,
            0.13052619f32,
            0.21643961f32,
            0.30070580f32,
            0.38268343f32,
            0.46174861f32,
            0.53729961f32,
            0.60876143f32,
            0.67559021f32,
        ],
        [
            1 as ::core::ffi::c_int as ::core::ffi::c_float,
            1 as ::core::ffi::c_int as ::core::ffi::c_float,
            1 as ::core::ffi::c_int as ::core::ffi::c_float,
            1 as ::core::ffi::c_int as ::core::ffi::c_float,
            1 as ::core::ffi::c_int as ::core::ffi::c_float,
            1 as ::core::ffi::c_int as ::core::ffi::c_float,
            0.99144486f32,
            0.92387953f32,
            0.79335334f32,
            0 as ::core::ffi::c_int as ::core::ffi::c_float,
            0 as ::core::ffi::c_int as ::core::ffi::c_float,
            0 as ::core::ffi::c_int as ::core::ffi::c_float,
            0 as ::core::ffi::c_int as ::core::ffi::c_float,
            0 as ::core::ffi::c_int as ::core::ffi::c_float,
            0 as ::core::ffi::c_int as ::core::ffi::c_float,
            0.13052619f32,
            0.38268343f32,
            0.60876143f32,
        ],
    ];
    if n_long_bands != 0 {
        L3_imdct36(
            grbuf,
            overlap,
            &raw const *(&raw const g_mdct_window as *const [::core::ffi::c_float; 18])
                .offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_float,
            n_long_bands as ::core::ffi::c_int,
        );
        grbuf = grbuf
            .offset((18 as ::core::ffi::c_uint).wrapping_mul(n_long_bands) as isize);
        overlap = overlap
            .offset((9 as ::core::ffi::c_uint).wrapping_mul(n_long_bands) as isize);
    }
    if block_type == SHORT_BLOCK_TYPE as ::core::ffi::c_uint {
        L3_imdct_short(
            grbuf,
            overlap,
            (32 as ::core::ffi::c_uint).wrapping_sub(n_long_bands) as ::core::ffi::c_int,
        );
    } else {
        L3_imdct36(
            grbuf,
            overlap,
            &raw const *(&raw const g_mdct_window as *const [::core::ffi::c_float; 18])
                .offset(
                    (block_type == STOP_BLOCK_TYPE as ::core::ffi::c_uint)
                        as ::core::ffi::c_int as isize,
                ) as *const ::core::ffi::c_float,
            (32 as ::core::ffi::c_uint).wrapping_sub(n_long_bands) as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn L3_save_reservoir(
    mut h: *mut mp3dec_t,
    mut s: *mut mp3dec_scratch_t,
) {
    let mut pos: ::core::ffi::c_int = (((*s).bs.pos + 7 as ::core::ffi::c_int)
        as ::core::ffi::c_uint)
        .wrapping_div(8 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    let mut remains: ::core::ffi::c_int = ((*s).bs.limit as ::core::ffi::c_uint)
        .wrapping_div(8 as ::core::ffi::c_uint)
        .wrapping_sub(pos as ::core::ffi::c_uint) as ::core::ffi::c_int;
    if remains > MAX_BITRESERVOIR_BYTES {
        pos += remains - MAX_BITRESERVOIR_BYTES;
        remains = MAX_BITRESERVOIR_BYTES;
    }
    if remains > 0 as ::core::ffi::c_int {
        memmove(
            &raw mut (*h).reserv_buf as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_void,
            (&raw mut (*s).maindata as *mut uint8_t).offset(pos as isize)
                as *const ::core::ffi::c_void,
            remains as size_t,
        );
    }
    (*h).reserv = remains;
}
unsafe extern "C" fn L3_restore_reservoir(
    mut h: *mut mp3dec_t,
    mut bs: *mut bs_t,
    mut s: *mut mp3dec_scratch_t,
    mut main_data_begin: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut frame_bytes: ::core::ffi::c_int = ((*bs).limit - (*bs).pos)
        / 8 as ::core::ffi::c_int;
    let mut bytes_have: ::core::ffi::c_int = if (*h).reserv > main_data_begin {
        main_data_begin
    } else {
        (*h).reserv
    };
    memcpy(
        &raw mut (*s).maindata as *mut uint8_t as *mut ::core::ffi::c_void,
        (&raw mut (*h).reserv_buf as *mut ::core::ffi::c_uchar)
            .offset(
                (if (0 as ::core::ffi::c_int) < (*h).reserv - main_data_begin {
                    (*h).reserv - main_data_begin
                } else {
                    0 as ::core::ffi::c_int
                }) as isize,
            ) as *const ::core::ffi::c_void,
        (if (*h).reserv > main_data_begin { main_data_begin } else { (*h).reserv })
            as size_t,
    );
    memcpy(
        (&raw mut (*s).maindata as *mut uint8_t).offset(bytes_have as isize)
            as *mut ::core::ffi::c_void,
        (*bs).buf.offset(((*bs).pos / 8 as ::core::ffi::c_int) as isize)
            as *const ::core::ffi::c_void,
        frame_bytes as size_t,
    );
    bs_init(
        &raw mut (*s).bs,
        &raw mut (*s).maindata as *mut uint8_t,
        bytes_have + frame_bytes,
    );
    return ((*h).reserv >= main_data_begin) as ::core::ffi::c_int;
}
unsafe extern "C" fn L3_decode(
    mut h: *mut mp3dec_t,
    mut s: *mut mp3dec_scratch_t,
    mut gr_info: *mut L3_gr_info_t,
    mut nch: ::core::ffi::c_int,
) {
    let mut ch: ::core::ffi::c_int = 0;
    ch = 0 as ::core::ffi::c_int;
    while ch < nch {
        let mut layer3gr_limit: ::core::ffi::c_int = (*s).bs.pos
            + (*gr_info.offset(ch as isize)).part_23_length as ::core::ffi::c_int;
        L3_decode_scalefactors(
            &raw mut (*h).header as *mut ::core::ffi::c_uchar,
            &raw mut *(&raw mut (*s).ist_pos as *mut [uint8_t; 39]).offset(ch as isize)
                as *mut uint8_t,
            &raw mut (*s).bs,
            gr_info.offset(ch as isize),
            &raw mut (*s).scf as *mut ::core::ffi::c_float,
            ch,
        );
        L3_huffman(
            &raw mut *(&raw mut (*s).grbuf as *mut [::core::ffi::c_float; 576])
                .offset(ch as isize) as *mut ::core::ffi::c_float,
            &raw mut (*s).bs,
            gr_info.offset(ch as isize),
            &raw mut (*s).scf as *mut ::core::ffi::c_float,
            layer3gr_limit,
        );
        ch += 1;
    }
    if (*h).header[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        & 0x10 as ::core::ffi::c_int != 0
    {
        L3_intensity_stereo(
            &raw mut *(&raw mut (*s).grbuf as *mut [::core::ffi::c_float; 576])
                .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_float,
            &raw mut *(&raw mut (*s).ist_pos as *mut [uint8_t; 39])
                .offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
            gr_info,
            &raw mut (*h).header as *mut ::core::ffi::c_uchar,
        );
    } else if (*h).header[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        & 0xe0 as ::core::ffi::c_int == 0x60 as ::core::ffi::c_int
    {
        L3_midside_stereo(
            &raw mut *(&raw mut (*s).grbuf as *mut [::core::ffi::c_float; 576])
                .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_float,
            576 as ::core::ffi::c_int,
        );
    }
    ch = 0 as ::core::ffi::c_int;
    while ch < nch {
        let mut aa_bands: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
        let mut n_long_bands: ::core::ffi::c_int = (if (*gr_info).mixed_block_flag
            as ::core::ffi::c_int != 0
        {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })
            << (((*h).header[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
                + (((*h).header[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
                    + ((*h).header[1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                        & 1 as ::core::ffi::c_int)) * 3 as ::core::ffi::c_int
                == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if (*gr_info).n_short_sfb != 0 {
            aa_bands = n_long_bands - 1 as ::core::ffi::c_int;
            L3_reorder(
                (&raw mut *(&raw mut (*s).grbuf as *mut [::core::ffi::c_float; 576])
                    .offset(ch as isize) as *mut ::core::ffi::c_float)
                    .offset((n_long_bands * 18 as ::core::ffi::c_int) as isize),
                &raw mut *(&raw mut (*s).syn as *mut [::core::ffi::c_float; 64])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_float,
                (*gr_info)
                    .sfbtab
                    .offset((*gr_info).n_long_sfb as ::core::ffi::c_int as isize),
            );
        }
        L3_antialias(
            &raw mut *(&raw mut (*s).grbuf as *mut [::core::ffi::c_float; 576])
                .offset(ch as isize) as *mut ::core::ffi::c_float,
            aa_bands,
        );
        L3_imdct_gr(
            &raw mut *(&raw mut (*s).grbuf as *mut [::core::ffi::c_float; 576])
                .offset(ch as isize) as *mut ::core::ffi::c_float,
            &raw mut *(&raw mut (*h).mdct_overlap as *mut [::core::ffi::c_float; 288])
                .offset(ch as isize) as *mut ::core::ffi::c_float,
            (*gr_info).block_type as ::core::ffi::c_uint,
            n_long_bands as ::core::ffi::c_uint,
        );
        L3_change_sign(
            &raw mut *(&raw mut (*s).grbuf as *mut [::core::ffi::c_float; 576])
                .offset(ch as isize) as *mut ::core::ffi::c_float,
        );
        ch += 1;
        gr_info = gr_info.offset(1);
    }
}
unsafe extern "C" fn mp3d_DCT_II(
    mut grbuf: *mut ::core::ffi::c_float,
    mut n: ::core::ffi::c_int,
) {
    static mut g_sec: [::core::ffi::c_float; 24] = [
        10.19000816f32,
        0.50060302f32,
        0.50241929f32,
        3.40760851f32,
        0.50547093f32,
        0.52249861f32,
        2.05778098f32,
        0.51544732f32,
        0.56694406f32,
        1.48416460f32,
        0.53104258f32,
        0.64682180f32,
        1.16943991f32,
        0.55310392f32,
        0.78815460f32,
        0.97256821f32,
        0.58293498f32,
        1.06067765f32,
        0.83934963f32,
        0.62250412f32,
        1.72244716f32,
        0.74453628f32,
        0.67480832f32,
        5.10114861f32,
    ];
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while k < n {
        let mut t: [[::core::ffi::c_float; 8]; 4] = [[0.; 8]; 4];
        let mut x: *mut ::core::ffi::c_float = ::core::ptr::null_mut::<
            ::core::ffi::c_float,
        >();
        let mut y: *mut ::core::ffi::c_float = grbuf.offset(k as isize);
        x = &raw mut *(&raw mut t as *mut [::core::ffi::c_float; 8])
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_float;
        i = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut x0: ::core::ffi::c_float = *y
                .offset((i * 18 as ::core::ffi::c_int) as isize);
            let mut x1: ::core::ffi::c_float = *y
                .offset(
                    ((15 as ::core::ffi::c_int - i) * 18 as ::core::ffi::c_int) as isize,
                );
            let mut x2: ::core::ffi::c_float = *y
                .offset(
                    ((16 as ::core::ffi::c_int + i) * 18 as ::core::ffi::c_int) as isize,
                );
            let mut x3: ::core::ffi::c_float = *y
                .offset(
                    ((31 as ::core::ffi::c_int - i) * 18 as ::core::ffi::c_int) as isize,
                );
            let mut t0: ::core::ffi::c_float = x0 + x3;
            let mut t1: ::core::ffi::c_float = x1 + x2;
            let mut t2: ::core::ffi::c_float = (x1 - x2)
                * g_sec[(3 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int)
                    as usize];
            let mut t3: ::core::ffi::c_float = (x0 - x3)
                * g_sec[(3 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int)
                    as usize];
            *x.offset(0 as ::core::ffi::c_int as isize) = t0 + t1;
            *x.offset(8 as ::core::ffi::c_int as isize) = (t0 - t1)
                * g_sec[(3 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int)
                    as usize];
            *x.offset(16 as ::core::ffi::c_int as isize) = t3 + t2;
            *x.offset(24 as ::core::ffi::c_int as isize) = (t3 - t2)
                * g_sec[(3 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int)
                    as usize];
            i += 1;
            x = x.offset(1);
        }
        x = &raw mut *(&raw mut t as *mut [::core::ffi::c_float; 8])
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_float;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let mut x0_0: ::core::ffi::c_float = *x
                .offset(0 as ::core::ffi::c_int as isize);
            let mut x1_0: ::core::ffi::c_float = *x
                .offset(1 as ::core::ffi::c_int as isize);
            let mut x2_0: ::core::ffi::c_float = *x
                .offset(2 as ::core::ffi::c_int as isize);
            let mut x3_0: ::core::ffi::c_float = *x
                .offset(3 as ::core::ffi::c_int as isize);
            let mut x4: ::core::ffi::c_float = *x
                .offset(4 as ::core::ffi::c_int as isize);
            let mut x5: ::core::ffi::c_float = *x
                .offset(5 as ::core::ffi::c_int as isize);
            let mut x6: ::core::ffi::c_float = *x
                .offset(6 as ::core::ffi::c_int as isize);
            let mut x7: ::core::ffi::c_float = *x
                .offset(7 as ::core::ffi::c_int as isize);
            let mut xt: ::core::ffi::c_float = 0.;
            xt = x0_0 - x7;
            x0_0 += x7;
            x7 = x1_0 - x6;
            x1_0 += x6;
            x6 = x2_0 - x5;
            x2_0 += x5;
            x5 = x3_0 - x4;
            x3_0 += x4;
            x4 = x0_0 - x3_0;
            x0_0 += x3_0;
            x3_0 = x1_0 - x2_0;
            x1_0 += x2_0;
            *x.offset(0 as ::core::ffi::c_int as isize) = x0_0 + x1_0;
            *x.offset(4 as ::core::ffi::c_int as isize) = (x0_0 - x1_0) * 0.70710677f32;
            x5 = x5 + x6;
            x6 = (x6 + x7) * 0.70710677f32;
            x7 = x7 + xt;
            x3_0 = (x3_0 + x4) * 0.70710677f32;
            x5 -= x7 * 0.198912367f32;
            x7 += x5 * 0.382683432f32;
            x5 -= x7 * 0.198912367f32;
            x0_0 = xt - x6;
            xt += x6;
            *x.offset(1 as ::core::ffi::c_int as isize) = (xt + x7) * 0.50979561f32;
            *x.offset(2 as ::core::ffi::c_int as isize) = (x4 + x3_0) * 0.54119611f32;
            *x.offset(3 as ::core::ffi::c_int as isize) = (x0_0 - x5) * 0.60134488f32;
            *x.offset(5 as ::core::ffi::c_int as isize) = (x0_0 + x5) * 0.89997619f32;
            *x.offset(6 as ::core::ffi::c_int as isize) = (x4 - x3_0) * 1.30656302f32;
            *x.offset(7 as ::core::ffi::c_int as isize) = (xt - x7) * 2.56291556f32;
            i += 1;
            x = x.offset(8 as ::core::ffi::c_int as isize);
        }
        i = 0 as ::core::ffi::c_int;
        while i < 7 as ::core::ffi::c_int {
            *y.offset((0 as ::core::ffi::c_int * 18 as ::core::ffi::c_int) as isize) = t[0
                as ::core::ffi::c_int as usize][i as usize];
            *y.offset((1 as ::core::ffi::c_int * 18 as ::core::ffi::c_int) as isize) = t[2
                as ::core::ffi::c_int as usize][i as usize]
                + t[3 as ::core::ffi::c_int as usize][i as usize]
                + t[3 as ::core::ffi::c_int
                    as usize][(i + 1 as ::core::ffi::c_int) as usize];
            *y.offset((2 as ::core::ffi::c_int * 18 as ::core::ffi::c_int) as isize) = t[1
                as ::core::ffi::c_int as usize][i as usize]
                + t[1 as ::core::ffi::c_int
                    as usize][(i + 1 as ::core::ffi::c_int) as usize];
            *y.offset((3 as ::core::ffi::c_int * 18 as ::core::ffi::c_int) as isize) = t[2
                as ::core::ffi::c_int as usize][(i + 1 as ::core::ffi::c_int) as usize]
                + t[3 as ::core::ffi::c_int as usize][i as usize]
                + t[3 as ::core::ffi::c_int
                    as usize][(i + 1 as ::core::ffi::c_int) as usize];
            i += 1;
            y = y.offset((4 as ::core::ffi::c_int * 18 as ::core::ffi::c_int) as isize);
        }
        *y.offset((0 as ::core::ffi::c_int * 18 as ::core::ffi::c_int) as isize) = t[0
            as ::core::ffi::c_int as usize][7 as ::core::ffi::c_int as usize];
        *y.offset((1 as ::core::ffi::c_int * 18 as ::core::ffi::c_int) as isize) = t[2
            as ::core::ffi::c_int as usize][7 as ::core::ffi::c_int as usize]
            + t[3 as ::core::ffi::c_int as usize][7 as ::core::ffi::c_int as usize];
        *y.offset((2 as ::core::ffi::c_int * 18 as ::core::ffi::c_int) as isize) = t[1
            as ::core::ffi::c_int as usize][7 as ::core::ffi::c_int as usize];
        *y.offset((3 as ::core::ffi::c_int * 18 as ::core::ffi::c_int) as isize) = t[3
            as ::core::ffi::c_int as usize][7 as ::core::ffi::c_int as usize];
        k += 1;
    }
}
unsafe extern "C" fn mp3d_scale_pcm(mut sample: ::core::ffi::c_float) -> int16_t {
    if sample as ::core::ffi::c_double >= 32766.5f64 {
        return 32767 as ::core::ffi::c_int as int16_t;
    }
    if sample as ::core::ffi::c_double <= -32767.5f64 {
        return -(32768 as ::core::ffi::c_int) as int16_t;
    }
    let mut s: int16_t = (sample + 0.5f32) as int16_t;
    s = (s as ::core::ffi::c_int
        - ((s as ::core::ffi::c_int) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int)
        as int16_t;
    return s;
}
unsafe extern "C" fn mp3d_synth_pair(
    mut pcm: *mut mp3d_sample_t,
    mut nch: ::core::ffi::c_int,
    mut z: *const ::core::ffi::c_float,
) {
    let mut a: ::core::ffi::c_float = 0.;
    a = (*z.offset((14 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
        - *z.offset(0 as ::core::ffi::c_int as isize))
        * 29 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += (*z.offset((1 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            + *z.offset((13 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize))
            * 213 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += (*z.offset((12 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            - *z.offset((2 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize))
            * 459 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += (*z.offset((3 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            + *z.offset((11 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize))
            * 2037 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += (*z.offset((10 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            - *z.offset((4 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize))
            * 5153 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += (*z.offset((5 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            + *z.offset((9 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize))
            * 6574 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += (*z.offset((8 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            - *z.offset((6 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize))
            * 37489 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += *z.offset((7 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            * 75038 as ::core::ffi::c_int as ::core::ffi::c_float;
    *pcm.offset(0 as ::core::ffi::c_int as isize) = mp3d_scale_pcm(a) as mp3d_sample_t;
    z = z.offset(2 as ::core::ffi::c_int as isize);
    a = *z.offset((14 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
        * 104 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += *z.offset((12 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            * 1567 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += *z.offset((10 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            * 9727 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += *z.offset((8 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            * 64019 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += *z.offset((6 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            * -(9975 as ::core::ffi::c_int) as ::core::ffi::c_float;
    a
        += *z.offset((4 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            * -(45 as ::core::ffi::c_int) as ::core::ffi::c_float;
    a
        += *z.offset((2 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            * 146 as ::core::ffi::c_int as ::core::ffi::c_float;
    a
        += *z.offset((0 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize)
            * -(5 as ::core::ffi::c_int) as ::core::ffi::c_float;
    *pcm.offset((16 as ::core::ffi::c_int * nch) as isize) = mp3d_scale_pcm(a)
        as mp3d_sample_t;
}
unsafe extern "C" fn mp3d_synth(
    mut xl: *mut ::core::ffi::c_float,
    mut dstl: *mut mp3d_sample_t,
    mut nch: ::core::ffi::c_int,
    mut lins: *mut ::core::ffi::c_float,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut xr: *mut ::core::ffi::c_float = xl
        .offset((576 as ::core::ffi::c_int * (nch - 1 as ::core::ffi::c_int)) as isize);
    let mut dstr: *mut mp3d_sample_t = dstl
        .offset((nch - 1 as ::core::ffi::c_int) as isize);
    static mut g_win: [::core::ffi::c_float; 240] = [
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
        26 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(31 as ::core::ffi::c_int) as ::core::ffi::c_float,
        208 as ::core::ffi::c_int as ::core::ffi::c_float,
        218 as ::core::ffi::c_int as ::core::ffi::c_float,
        401 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(519 as ::core::ffi::c_int) as ::core::ffi::c_float,
        2063 as ::core::ffi::c_int as ::core::ffi::c_float,
        2000 as ::core::ffi::c_int as ::core::ffi::c_float,
        4788 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(5517 as ::core::ffi::c_int) as ::core::ffi::c_float,
        7134 as ::core::ffi::c_int as ::core::ffi::c_float,
        5959 as ::core::ffi::c_int as ::core::ffi::c_float,
        35640 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(39336 as ::core::ffi::c_int) as ::core::ffi::c_float,
        74992 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
        24 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(35 as ::core::ffi::c_int) as ::core::ffi::c_float,
        202 as ::core::ffi::c_int as ::core::ffi::c_float,
        222 as ::core::ffi::c_int as ::core::ffi::c_float,
        347 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(581 as ::core::ffi::c_int) as ::core::ffi::c_float,
        2080 as ::core::ffi::c_int as ::core::ffi::c_float,
        1952 as ::core::ffi::c_int as ::core::ffi::c_float,
        4425 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(5879 as ::core::ffi::c_int) as ::core::ffi::c_float,
        7640 as ::core::ffi::c_int as ::core::ffi::c_float,
        5288 as ::core::ffi::c_int as ::core::ffi::c_float,
        33791 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(41176 as ::core::ffi::c_int) as ::core::ffi::c_float,
        74856 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
        21 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(38 as ::core::ffi::c_int) as ::core::ffi::c_float,
        196 as ::core::ffi::c_int as ::core::ffi::c_float,
        225 as ::core::ffi::c_int as ::core::ffi::c_float,
        294 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(645 as ::core::ffi::c_int) as ::core::ffi::c_float,
        2087 as ::core::ffi::c_int as ::core::ffi::c_float,
        1893 as ::core::ffi::c_int as ::core::ffi::c_float,
        4063 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(6237 as ::core::ffi::c_int) as ::core::ffi::c_float,
        8092 as ::core::ffi::c_int as ::core::ffi::c_float,
        4561 as ::core::ffi::c_int as ::core::ffi::c_float,
        31947 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(43006 as ::core::ffi::c_int) as ::core::ffi::c_float,
        74630 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
        19 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(41 as ::core::ffi::c_int) as ::core::ffi::c_float,
        190 as ::core::ffi::c_int as ::core::ffi::c_float,
        227 as ::core::ffi::c_int as ::core::ffi::c_float,
        244 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(711 as ::core::ffi::c_int) as ::core::ffi::c_float,
        2085 as ::core::ffi::c_int as ::core::ffi::c_float,
        1822 as ::core::ffi::c_int as ::core::ffi::c_float,
        3705 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(6589 as ::core::ffi::c_int) as ::core::ffi::c_float,
        8492 as ::core::ffi::c_int as ::core::ffi::c_float,
        3776 as ::core::ffi::c_int as ::core::ffi::c_float,
        30112 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(44821 as ::core::ffi::c_int) as ::core::ffi::c_float,
        74313 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
        17 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(45 as ::core::ffi::c_int) as ::core::ffi::c_float,
        183 as ::core::ffi::c_int as ::core::ffi::c_float,
        228 as ::core::ffi::c_int as ::core::ffi::c_float,
        197 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(779 as ::core::ffi::c_int) as ::core::ffi::c_float,
        2075 as ::core::ffi::c_int as ::core::ffi::c_float,
        1739 as ::core::ffi::c_int as ::core::ffi::c_float,
        3351 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(6935 as ::core::ffi::c_int) as ::core::ffi::c_float,
        8840 as ::core::ffi::c_int as ::core::ffi::c_float,
        2935 as ::core::ffi::c_int as ::core::ffi::c_float,
        28289 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(46617 as ::core::ffi::c_int) as ::core::ffi::c_float,
        73908 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_float,
        16 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(49 as ::core::ffi::c_int) as ::core::ffi::c_float,
        176 as ::core::ffi::c_int as ::core::ffi::c_float,
        228 as ::core::ffi::c_int as ::core::ffi::c_float,
        153 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(848 as ::core::ffi::c_int) as ::core::ffi::c_float,
        2057 as ::core::ffi::c_int as ::core::ffi::c_float,
        1644 as ::core::ffi::c_int as ::core::ffi::c_float,
        3004 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(7271 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9139 as ::core::ffi::c_int as ::core::ffi::c_float,
        2037 as ::core::ffi::c_int as ::core::ffi::c_float,
        26482 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(48390 as ::core::ffi::c_int) as ::core::ffi::c_float,
        73415 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(2 as ::core::ffi::c_int) as ::core::ffi::c_float,
        14 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(53 as ::core::ffi::c_int) as ::core::ffi::c_float,
        169 as ::core::ffi::c_int as ::core::ffi::c_float,
        227 as ::core::ffi::c_int as ::core::ffi::c_float,
        111 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(919 as ::core::ffi::c_int) as ::core::ffi::c_float,
        2032 as ::core::ffi::c_int as ::core::ffi::c_float,
        1535 as ::core::ffi::c_int as ::core::ffi::c_float,
        2663 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(7597 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9389 as ::core::ffi::c_int as ::core::ffi::c_float,
        1082 as ::core::ffi::c_int as ::core::ffi::c_float,
        24694 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(50137 as ::core::ffi::c_int) as ::core::ffi::c_float,
        72835 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(2 as ::core::ffi::c_int) as ::core::ffi::c_float,
        13 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(58 as ::core::ffi::c_int) as ::core::ffi::c_float,
        161 as ::core::ffi::c_int as ::core::ffi::c_float,
        224 as ::core::ffi::c_int as ::core::ffi::c_float,
        72 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(991 as ::core::ffi::c_int) as ::core::ffi::c_float,
        2001 as ::core::ffi::c_int as ::core::ffi::c_float,
        1414 as ::core::ffi::c_int as ::core::ffi::c_float,
        2330 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(7910 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9592 as ::core::ffi::c_int as ::core::ffi::c_float,
        70 as ::core::ffi::c_int as ::core::ffi::c_float,
        22929 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(51853 as ::core::ffi::c_int) as ::core::ffi::c_float,
        72169 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(2 as ::core::ffi::c_int) as ::core::ffi::c_float,
        11 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(63 as ::core::ffi::c_int) as ::core::ffi::c_float,
        154 as ::core::ffi::c_int as ::core::ffi::c_float,
        221 as ::core::ffi::c_int as ::core::ffi::c_float,
        36 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(1064 as ::core::ffi::c_int) as ::core::ffi::c_float,
        1962 as ::core::ffi::c_int as ::core::ffi::c_float,
        1280 as ::core::ffi::c_int as ::core::ffi::c_float,
        2006 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(8209 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9750 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(998 as ::core::ffi::c_int) as ::core::ffi::c_float,
        21189 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(53534 as ::core::ffi::c_int) as ::core::ffi::c_float,
        71420 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(2 as ::core::ffi::c_int) as ::core::ffi::c_float,
        10 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(68 as ::core::ffi::c_int) as ::core::ffi::c_float,
        147 as ::core::ffi::c_int as ::core::ffi::c_float,
        215 as ::core::ffi::c_int as ::core::ffi::c_float,
        2 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(1137 as ::core::ffi::c_int) as ::core::ffi::c_float,
        1919 as ::core::ffi::c_int as ::core::ffi::c_float,
        1131 as ::core::ffi::c_int as ::core::ffi::c_float,
        1692 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(8491 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9863 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(2122 as ::core::ffi::c_int) as ::core::ffi::c_float,
        19478 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(55178 as ::core::ffi::c_int) as ::core::ffi::c_float,
        70590 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(3 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(73 as ::core::ffi::c_int) as ::core::ffi::c_float,
        139 as ::core::ffi::c_int as ::core::ffi::c_float,
        208 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(29 as ::core::ffi::c_int) as ::core::ffi::c_float,
        -(1210 as ::core::ffi::c_int) as ::core::ffi::c_float,
        1870 as ::core::ffi::c_int as ::core::ffi::c_float,
        970 as ::core::ffi::c_int as ::core::ffi::c_float,
        1388 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(8755 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9935 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(3300 as ::core::ffi::c_int) as ::core::ffi::c_float,
        17799 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(56778 as ::core::ffi::c_int) as ::core::ffi::c_float,
        69679 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(3 as ::core::ffi::c_int) as ::core::ffi::c_float,
        8 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(79 as ::core::ffi::c_int) as ::core::ffi::c_float,
        132 as ::core::ffi::c_int as ::core::ffi::c_float,
        200 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(57 as ::core::ffi::c_int) as ::core::ffi::c_float,
        -(1283 as ::core::ffi::c_int) as ::core::ffi::c_float,
        1817 as ::core::ffi::c_int as ::core::ffi::c_float,
        794 as ::core::ffi::c_int as ::core::ffi::c_float,
        1095 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(8998 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9966 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(4533 as ::core::ffi::c_int) as ::core::ffi::c_float,
        16155 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(58333 as ::core::ffi::c_int) as ::core::ffi::c_float,
        68692 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(4 as ::core::ffi::c_int) as ::core::ffi::c_float,
        7 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(85 as ::core::ffi::c_int) as ::core::ffi::c_float,
        125 as ::core::ffi::c_int as ::core::ffi::c_float,
        189 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(83 as ::core::ffi::c_int) as ::core::ffi::c_float,
        -(1356 as ::core::ffi::c_int) as ::core::ffi::c_float,
        1759 as ::core::ffi::c_int as ::core::ffi::c_float,
        605 as ::core::ffi::c_int as ::core::ffi::c_float,
        814 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(9219 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9959 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(5818 as ::core::ffi::c_int) as ::core::ffi::c_float,
        14548 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(59838 as ::core::ffi::c_int) as ::core::ffi::c_float,
        67629 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(4 as ::core::ffi::c_int) as ::core::ffi::c_float,
        7 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(91 as ::core::ffi::c_int) as ::core::ffi::c_float,
        117 as ::core::ffi::c_int as ::core::ffi::c_float,
        177 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(106 as ::core::ffi::c_int) as ::core::ffi::c_float,
        -(1428 as ::core::ffi::c_int) as ::core::ffi::c_float,
        1698 as ::core::ffi::c_int as ::core::ffi::c_float,
        402 as ::core::ffi::c_int as ::core::ffi::c_float,
        545 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(9416 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9916 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(7154 as ::core::ffi::c_int) as ::core::ffi::c_float,
        12980 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(61289 as ::core::ffi::c_int) as ::core::ffi::c_float,
        66494 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(5 as ::core::ffi::c_int) as ::core::ffi::c_float,
        6 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(97 as ::core::ffi::c_int) as ::core::ffi::c_float,
        111 as ::core::ffi::c_int as ::core::ffi::c_float,
        163 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(127 as ::core::ffi::c_int) as ::core::ffi::c_float,
        -(1498 as ::core::ffi::c_int) as ::core::ffi::c_float,
        1634 as ::core::ffi::c_int as ::core::ffi::c_float,
        185 as ::core::ffi::c_int as ::core::ffi::c_float,
        288 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(9585 as ::core::ffi::c_int) as ::core::ffi::c_float,
        9838 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(8540 as ::core::ffi::c_int) as ::core::ffi::c_float,
        11455 as ::core::ffi::c_int as ::core::ffi::c_float,
        -(62684 as ::core::ffi::c_int) as ::core::ffi::c_float,
        65290 as ::core::ffi::c_int as ::core::ffi::c_float,
    ];
    let mut zlin: *mut ::core::ffi::c_float = lins
        .offset((15 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize);
    let mut w: *const ::core::ffi::c_float = &raw const g_win
        as *const ::core::ffi::c_float;
    *zlin.offset((4 as ::core::ffi::c_int * 15 as ::core::ffi::c_int) as isize) = *xl
        .offset((18 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize);
    *zlin
        .offset(
            (4 as ::core::ffi::c_int * 15 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as isize,
        ) = *xr.offset((18 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize);
    *zlin
        .offset(
            (4 as ::core::ffi::c_int * 15 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as isize,
        ) = *xl.offset(0 as ::core::ffi::c_int as isize);
    *zlin
        .offset(
            (4 as ::core::ffi::c_int * 15 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) as isize,
        ) = *xr.offset(0 as ::core::ffi::c_int as isize);
    *zlin.offset((4 as ::core::ffi::c_int * 31 as ::core::ffi::c_int) as isize) = *xl
        .offset(
            (1 as ::core::ffi::c_int
                + 18 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize,
        );
    *zlin
        .offset(
            (4 as ::core::ffi::c_int * 31 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) as isize,
        ) = *xr
        .offset(
            (1 as ::core::ffi::c_int
                + 18 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize,
        );
    *zlin
        .offset(
            (4 as ::core::ffi::c_int * 31 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as isize,
        ) = *xl.offset(1 as ::core::ffi::c_int as isize);
    *zlin
        .offset(
            (4 as ::core::ffi::c_int * 31 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int) as isize,
        ) = *xr.offset(1 as ::core::ffi::c_int as isize);
    mp3d_synth_pair(
        dstr,
        nch,
        lins
            .offset((4 as ::core::ffi::c_int * 15 as ::core::ffi::c_int) as isize)
            .offset(1 as ::core::ffi::c_int as isize),
    );
    mp3d_synth_pair(
        dstr.offset((32 as ::core::ffi::c_int * nch) as isize),
        nch,
        lins
            .offset((4 as ::core::ffi::c_int * 15 as ::core::ffi::c_int) as isize)
            .offset(64 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize),
    );
    mp3d_synth_pair(
        dstl,
        nch,
        lins.offset((4 as ::core::ffi::c_int * 15 as ::core::ffi::c_int) as isize),
    );
    mp3d_synth_pair(
        dstl.offset((32 as ::core::ffi::c_int * nch) as isize),
        nch,
        lins
            .offset((4 as ::core::ffi::c_int * 15 as ::core::ffi::c_int) as isize)
            .offset(64 as ::core::ffi::c_int as isize),
    );
    i = 14 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        let mut a: [::core::ffi::c_float; 4] = [0.; 4];
        let mut b: [::core::ffi::c_float; 4] = [0.; 4];
        *zlin.offset((4 as ::core::ffi::c_int * i) as isize) = *xl
            .offset(
                (18 as ::core::ffi::c_int * (31 as ::core::ffi::c_int - i)) as isize,
            );
        *zlin.offset((4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize) = *xr
            .offset(
                (18 as ::core::ffi::c_int * (31 as ::core::ffi::c_int - i)) as isize,
            );
        *zlin.offset((4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as isize) = *xl
            .offset(
                (1 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int * (31 as ::core::ffi::c_int - i)) as isize,
            );
        *zlin.offset((4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int) as isize) = *xr
            .offset(
                (1 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int * (31 as ::core::ffi::c_int - i)) as isize,
            );
        *zlin
            .offset(
                (4 as ::core::ffi::c_int * (i + 16 as ::core::ffi::c_int)) as isize,
            ) = *xl
            .offset(
                (1 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int * (1 as ::core::ffi::c_int + i)) as isize,
            );
        *zlin
            .offset(
                (4 as ::core::ffi::c_int * (i + 16 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int) as isize,
            ) = *xr
            .offset(
                (1 as ::core::ffi::c_int
                    + 18 as ::core::ffi::c_int * (1 as ::core::ffi::c_int + i)) as isize,
            );
        *zlin
            .offset(
                (4 as ::core::ffi::c_int * (i - 16 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_int) as isize,
            ) = *xl
            .offset((18 as ::core::ffi::c_int * (1 as ::core::ffi::c_int + i)) as isize);
        *zlin
            .offset(
                (4 as ::core::ffi::c_int * (i - 16 as ::core::ffi::c_int)
                    + 3 as ::core::ffi::c_int) as isize,
            ) = *xr
            .offset((18 as ::core::ffi::c_int * (1 as ::core::ffi::c_int + i)) as isize);
        let mut j: ::core::ffi::c_int = 0;
        let fresh0 = w;
        w = w.offset(1);
        let mut w0: ::core::ffi::c_float = *fresh0;
        let fresh1 = w;
        w = w.offset(1);
        let mut w1: ::core::ffi::c_float = *fresh1;
        let mut vz: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - 0 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        let mut vy: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - (15 as ::core::ffi::c_int - 0 as ::core::ffi::c_int)
                        * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        j = 0 as ::core::ffi::c_int;
        while j < 4 as ::core::ffi::c_int {
            b[j as usize] = *vz.offset(j as isize) * w1 + *vy.offset(j as isize) * w0;
            a[j as usize] = *vz.offset(j as isize) * w0 - *vy.offset(j as isize) * w1;
            j += 1;
        }
        let mut j_0: ::core::ffi::c_int = 0;
        let fresh2 = w;
        w = w.offset(1);
        let mut w0_0: ::core::ffi::c_float = *fresh2;
        let fresh3 = w;
        w = w.offset(1);
        let mut w1_0: ::core::ffi::c_float = *fresh3;
        let mut vz_0: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - 1 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        let mut vy_0: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - (15 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        j_0 = 0 as ::core::ffi::c_int;
        while j_0 < 4 as ::core::ffi::c_int {
            b[j_0 as usize]
                += *vz_0.offset(j_0 as isize) * w1_0 + *vy_0.offset(j_0 as isize) * w0_0;
            a[j_0 as usize]
                += *vy_0.offset(j_0 as isize) * w1_0 - *vz_0.offset(j_0 as isize) * w0_0;
            j_0 += 1;
        }
        let mut j_1: ::core::ffi::c_int = 0;
        let fresh4 = w;
        w = w.offset(1);
        let mut w0_1: ::core::ffi::c_float = *fresh4;
        let fresh5 = w;
        w = w.offset(1);
        let mut w1_1: ::core::ffi::c_float = *fresh5;
        let mut vz_1: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - 2 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        let mut vy_1: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - (15 as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                        * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        j_1 = 0 as ::core::ffi::c_int;
        while j_1 < 4 as ::core::ffi::c_int {
            b[j_1 as usize]
                += *vz_1.offset(j_1 as isize) * w1_1 + *vy_1.offset(j_1 as isize) * w0_1;
            a[j_1 as usize]
                += *vz_1.offset(j_1 as isize) * w0_1 - *vy_1.offset(j_1 as isize) * w1_1;
            j_1 += 1;
        }
        let mut j_2: ::core::ffi::c_int = 0;
        let fresh6 = w;
        w = w.offset(1);
        let mut w0_2: ::core::ffi::c_float = *fresh6;
        let fresh7 = w;
        w = w.offset(1);
        let mut w1_2: ::core::ffi::c_float = *fresh7;
        let mut vz_2: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - 3 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        let mut vy_2: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - (15 as ::core::ffi::c_int - 3 as ::core::ffi::c_int)
                        * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        j_2 = 0 as ::core::ffi::c_int;
        while j_2 < 4 as ::core::ffi::c_int {
            b[j_2 as usize]
                += *vz_2.offset(j_2 as isize) * w1_2 + *vy_2.offset(j_2 as isize) * w0_2;
            a[j_2 as usize]
                += *vy_2.offset(j_2 as isize) * w1_2 - *vz_2.offset(j_2 as isize) * w0_2;
            j_2 += 1;
        }
        let mut j_3: ::core::ffi::c_int = 0;
        let fresh8 = w;
        w = w.offset(1);
        let mut w0_3: ::core::ffi::c_float = *fresh8;
        let fresh9 = w;
        w = w.offset(1);
        let mut w1_3: ::core::ffi::c_float = *fresh9;
        let mut vz_3: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - 4 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        let mut vy_3: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - (15 as ::core::ffi::c_int - 4 as ::core::ffi::c_int)
                        * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        j_3 = 0 as ::core::ffi::c_int;
        while j_3 < 4 as ::core::ffi::c_int {
            b[j_3 as usize]
                += *vz_3.offset(j_3 as isize) * w1_3 + *vy_3.offset(j_3 as isize) * w0_3;
            a[j_3 as usize]
                += *vz_3.offset(j_3 as isize) * w0_3 - *vy_3.offset(j_3 as isize) * w1_3;
            j_3 += 1;
        }
        let mut j_4: ::core::ffi::c_int = 0;
        let fresh10 = w;
        w = w.offset(1);
        let mut w0_4: ::core::ffi::c_float = *fresh10;
        let fresh11 = w;
        w = w.offset(1);
        let mut w1_4: ::core::ffi::c_float = *fresh11;
        let mut vz_4: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - 5 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        let mut vy_4: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - (15 as ::core::ffi::c_int - 5 as ::core::ffi::c_int)
                        * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        j_4 = 0 as ::core::ffi::c_int;
        while j_4 < 4 as ::core::ffi::c_int {
            b[j_4 as usize]
                += *vz_4.offset(j_4 as isize) * w1_4 + *vy_4.offset(j_4 as isize) * w0_4;
            a[j_4 as usize]
                += *vy_4.offset(j_4 as isize) * w1_4 - *vz_4.offset(j_4 as isize) * w0_4;
            j_4 += 1;
        }
        let mut j_5: ::core::ffi::c_int = 0;
        let fresh12 = w;
        w = w.offset(1);
        let mut w0_5: ::core::ffi::c_float = *fresh12;
        let fresh13 = w;
        w = w.offset(1);
        let mut w1_5: ::core::ffi::c_float = *fresh13;
        let mut vz_5: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - 6 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        let mut vy_5: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - (15 as ::core::ffi::c_int - 6 as ::core::ffi::c_int)
                        * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        j_5 = 0 as ::core::ffi::c_int;
        while j_5 < 4 as ::core::ffi::c_int {
            b[j_5 as usize]
                += *vz_5.offset(j_5 as isize) * w1_5 + *vy_5.offset(j_5 as isize) * w0_5;
            a[j_5 as usize]
                += *vz_5.offset(j_5 as isize) * w0_5 - *vy_5.offset(j_5 as isize) * w1_5;
            j_5 += 1;
        }
        let mut j_6: ::core::ffi::c_int = 0;
        let fresh14 = w;
        w = w.offset(1);
        let mut w0_6: ::core::ffi::c_float = *fresh14;
        let fresh15 = w;
        w = w.offset(1);
        let mut w1_6: ::core::ffi::c_float = *fresh15;
        let mut vz_6: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - 7 as ::core::ffi::c_int * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        let mut vy_6: *mut ::core::ffi::c_float = zlin
            .offset(
                (4 as ::core::ffi::c_int * i
                    - (15 as ::core::ffi::c_int - 7 as ::core::ffi::c_int)
                        * 64 as ::core::ffi::c_int) as isize,
            ) as *mut ::core::ffi::c_float;
        j_6 = 0 as ::core::ffi::c_int;
        while j_6 < 4 as ::core::ffi::c_int {
            b[j_6 as usize]
                += *vz_6.offset(j_6 as isize) * w1_6 + *vy_6.offset(j_6 as isize) * w0_6;
            a[j_6 as usize]
                += *vy_6.offset(j_6 as isize) * w1_6 - *vz_6.offset(j_6 as isize) * w0_6;
            j_6 += 1;
        }
        *dstr.offset(((15 as ::core::ffi::c_int - i) * nch) as isize) = mp3d_scale_pcm(
            a[1 as ::core::ffi::c_int as usize],
        ) as mp3d_sample_t;
        *dstr.offset(((17 as ::core::ffi::c_int + i) * nch) as isize) = mp3d_scale_pcm(
            b[1 as ::core::ffi::c_int as usize],
        ) as mp3d_sample_t;
        *dstl.offset(((15 as ::core::ffi::c_int - i) * nch) as isize) = mp3d_scale_pcm(
            a[0 as ::core::ffi::c_int as usize],
        ) as mp3d_sample_t;
        *dstl.offset(((17 as ::core::ffi::c_int + i) * nch) as isize) = mp3d_scale_pcm(
            b[0 as ::core::ffi::c_int as usize],
        ) as mp3d_sample_t;
        *dstr.offset(((47 as ::core::ffi::c_int - i) * nch) as isize) = mp3d_scale_pcm(
            a[3 as ::core::ffi::c_int as usize],
        ) as mp3d_sample_t;
        *dstr.offset(((49 as ::core::ffi::c_int + i) * nch) as isize) = mp3d_scale_pcm(
            b[3 as ::core::ffi::c_int as usize],
        ) as mp3d_sample_t;
        *dstl.offset(((47 as ::core::ffi::c_int - i) * nch) as isize) = mp3d_scale_pcm(
            a[2 as ::core::ffi::c_int as usize],
        ) as mp3d_sample_t;
        *dstl.offset(((49 as ::core::ffi::c_int + i) * nch) as isize) = mp3d_scale_pcm(
            b[2 as ::core::ffi::c_int as usize],
        ) as mp3d_sample_t;
        i -= 1;
    }
}
unsafe extern "C" fn mp3d_synth_granule(
    mut qmf_state: *mut ::core::ffi::c_float,
    mut grbuf: *mut ::core::ffi::c_float,
    mut nbands: ::core::ffi::c_int,
    mut nch: ::core::ffi::c_int,
    mut pcm: *mut mp3d_sample_t,
    mut lins: *mut ::core::ffi::c_float,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < nch {
        mp3d_DCT_II(grbuf.offset((576 as ::core::ffi::c_int * i) as isize), nbands);
        i += 1;
    }
    memcpy(
        lins as *mut ::core::ffi::c_void,
        qmf_state as *const ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_float>() as size_t)
            .wrapping_mul(15 as size_t)
            .wrapping_mul(64 as size_t),
    );
    i = 0 as ::core::ffi::c_int;
    while i < nbands {
        mp3d_synth(
            grbuf.offset(i as isize),
            pcm.offset((32 as ::core::ffi::c_int * nch * i) as isize),
            nch,
            lins.offset((i * 64 as ::core::ffi::c_int) as isize),
        );
        i += 2 as ::core::ffi::c_int;
    }
    if nch == 1 as ::core::ffi::c_int {
        i = 0 as ::core::ffi::c_int;
        while i < 15 as ::core::ffi::c_int * 64 as ::core::ffi::c_int {
            *qmf_state.offset(i as isize) = *lins
                .offset((nbands * 64 as ::core::ffi::c_int + i) as isize);
            i += 2 as ::core::ffi::c_int;
        }
    } else {
        memcpy(
            qmf_state as *mut ::core::ffi::c_void,
            lins.offset((nbands * 64 as ::core::ffi::c_int) as isize)
                as *const ::core::ffi::c_void,
            (::core::mem::size_of::<::core::ffi::c_float>() as size_t)
                .wrapping_mul(15 as size_t)
                .wrapping_mul(64 as size_t),
        );
    };
}
unsafe extern "C" fn mp3d_match_frame(
    mut hdr: *const uint8_t,
    mut mp3_bytes: ::core::ffi::c_int,
    mut frame_bytes: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut nmatch: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    nmatch = 0 as ::core::ffi::c_int;
    while nmatch < MAX_FRAME_SYNC_MATCHES {
        i
            += hdr_frame_bytes(hdr.offset(i as isize), frame_bytes)
                + hdr_padding(hdr.offset(i as isize));
        if i + HDR_SIZE > mp3_bytes {
            return (nmatch > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        if hdr_compare(hdr, hdr.offset(i as isize)) == 0 {
            return 0 as ::core::ffi::c_int;
        }
        nmatch += 1;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn mp3d_find_frame(
    mut mp3: *const uint8_t,
    mut mp3_bytes: ::core::ffi::c_int,
    mut free_format_bytes: *mut ::core::ffi::c_int,
    mut ptr_frame_bytes: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut k: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < mp3_bytes - HDR_SIZE {
        if hdr_valid(mp3) != 0 {
            let mut frame_bytes: ::core::ffi::c_int = hdr_frame_bytes(
                mp3,
                *free_format_bytes,
            );
            let mut frame_and_padding: ::core::ffi::c_int = frame_bytes
                + hdr_padding(mp3);
            k = HDR_SIZE;
            while frame_bytes == 0 && k < MAX_FREE_FORMAT_FRAME_SIZE
                && i + 2 as ::core::ffi::c_int * k < mp3_bytes - HDR_SIZE
            {
                if hdr_compare(mp3, mp3.offset(k as isize)) != 0 {
                    let mut fb: ::core::ffi::c_int = k - hdr_padding(mp3);
                    let mut nextfb: ::core::ffi::c_int = fb
                        + hdr_padding(mp3.offset(k as isize));
                    if !(i + k + nextfb + HDR_SIZE > mp3_bytes
                        || hdr_compare(
                            mp3,
                            mp3.offset(k as isize).offset(nextfb as isize),
                        ) == 0)
                    {
                        frame_and_padding = k;
                        frame_bytes = fb;
                        *free_format_bytes = fb;
                    }
                }
                k += 1;
            }
            if frame_bytes != 0 && i + frame_and_padding <= mp3_bytes
                && mp3d_match_frame(mp3, mp3_bytes - i, frame_bytes) != 0
                || i == 0 && frame_and_padding == mp3_bytes
            {
                *ptr_frame_bytes = frame_and_padding;
                return i;
            }
            *free_format_bytes = 0 as ::core::ffi::c_int;
        }
        i += 1;
        mp3 = mp3.offset(1);
    }
    *ptr_frame_bytes = 0 as ::core::ffi::c_int;
    return mp3_bytes;
}
pub unsafe extern "C" fn mp3dec_init(mut dec: *mut mp3dec_t) {
    (*dec).header[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_uchar;
}
pub unsafe extern "C" fn mp3dec_decode_frame(
    mut dec: *mut mp3dec_t,
    mut mp3: *const uint8_t,
    mut mp3_bytes: ::core::ffi::c_int,
    mut pcm: *mut mp3d_sample_t,
    mut info: *mut mp3dec_frame_info_t,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut igr: ::core::ffi::c_int = 0;
    let mut frame_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut success: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut hdr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut bs_frame: [bs_t; 1] = [bs_t {
        buf: ::core::ptr::null::<uint8_t>(),
        pos: 0,
        limit: 0,
    }; 1];
    let mut scratch: mp3dec_scratch_t = mp3dec_scratch_t {
        bs: bs_t {
            buf: ::core::ptr::null::<uint8_t>(),
            pos: 0,
            limit: 0,
        },
        maindata: [0; 2815],
        gr_info: [L3_gr_info_t {
            sfbtab: ::core::ptr::null::<uint8_t>(),
            part_23_length: 0,
            big_values: 0,
            scalefac_compress: 0,
            global_gain: 0,
            block_type: 0,
            mixed_block_flag: 0,
            n_long_sfb: 0,
            n_short_sfb: 0,
            table_select: [0; 3],
            region_count: [0; 3],
            subblock_gain: [0; 3],
            preflag: 0,
            scalefac_scale: 0,
            count1_table: 0,
            scfsi: 0,
        }; 4],
        grbuf: [[0.; 576]; 2],
        scf: [0.; 40],
        syn: [[0.; 64]; 33],
        ist_pos: [[0; 39]; 2],
    };
    if mp3_bytes > 4 as ::core::ffi::c_int
        && (*dec).header[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            == 0xff as ::core::ffi::c_int
        && hdr_compare(&raw mut (*dec).header as *mut ::core::ffi::c_uchar, mp3) != 0
    {
        frame_size = hdr_frame_bytes(mp3, (*dec).free_format_bytes) + hdr_padding(mp3);
        if frame_size != mp3_bytes
            && (frame_size + HDR_SIZE > mp3_bytes
                || hdr_compare(mp3, mp3.offset(frame_size as isize)) == 0)
        {
            frame_size = 0 as ::core::ffi::c_int;
        }
    }
    if frame_size == 0 {
        memset(
            dec as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<mp3dec_t>() as size_t,
        );
        i = mp3d_find_frame(
            mp3,
            mp3_bytes,
            &raw mut (*dec).free_format_bytes,
            &raw mut frame_size,
        );
        if frame_size == 0 || i + frame_size > mp3_bytes {
            (*info).frame_bytes = i;
            return 0 as ::core::ffi::c_int;
        }
    }
    hdr = mp3.offset(i as isize);
    memcpy(
        &raw mut (*dec).header as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
        hdr as *const ::core::ffi::c_void,
        HDR_SIZE as size_t,
    );
    (*info).frame_bytes = i + frame_size;
    (*info).frame_offset = i;
    (*info).channels = if *hdr.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int == 0xc0 as ::core::ffi::c_int
    {
        1 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    };
    (*info).hz = hdr_sample_rate_hz(hdr) as ::core::ffi::c_int;
    (*info).layer = 4 as ::core::ffi::c_int
        - (*hdr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int & 3 as ::core::ffi::c_int);
    (*info).bitrate_kbps = hdr_bitrate_kbps(hdr) as ::core::ffi::c_int;
    if pcm.is_null() {
        return hdr_frame_samples(hdr) as ::core::ffi::c_int;
    }
    bs_init(
        &raw mut bs_frame as *mut bs_t,
        hdr.offset(HDR_SIZE as isize),
        frame_size - HDR_SIZE,
    );
    if *hdr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 1 as ::core::ffi::c_int == 0
    {
        get_bits(&raw mut bs_frame as *mut bs_t, 16 as ::core::ffi::c_int);
    }
    if (*info).layer == 3 as ::core::ffi::c_int {
        let mut main_data_begin: ::core::ffi::c_int = L3_read_side_info(
            &raw mut bs_frame as *mut bs_t,
            &raw mut scratch.gr_info as *mut L3_gr_info_t,
            hdr,
        );
        if main_data_begin < 0 as ::core::ffi::c_int
            || (*(&raw mut bs_frame as *mut bs_t)).pos
                > (*(&raw mut bs_frame as *mut bs_t)).limit
        {
            mp3dec_init(dec);
            return 0 as ::core::ffi::c_int;
        }
        success = L3_restore_reservoir(
            dec,
            &raw mut bs_frame as *mut bs_t,
            &raw mut scratch,
            main_data_begin,
        );
        if success != 0 {
            igr = 0 as ::core::ffi::c_int;
            while igr
                < (if *hdr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x8 as ::core::ffi::c_int != 0
                {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                })
            {
                memset(
                    &raw mut *(&raw mut scratch.grbuf
                        as *mut [::core::ffi::c_float; 576])
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_float as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ((576 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as size_t)
                        .wrapping_mul(
                            ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
                        ),
                );
                L3_decode(
                    dec,
                    &raw mut scratch,
                    (&raw mut scratch.gr_info as *mut L3_gr_info_t)
                        .offset((igr * (*info).channels) as isize),
                    (*info).channels,
                );
                mp3d_synth_granule(
                    &raw mut (*dec).qmf_state as *mut ::core::ffi::c_float,
                    &raw mut *(&raw mut scratch.grbuf
                        as *mut [::core::ffi::c_float; 576])
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_float,
                    18 as ::core::ffi::c_int,
                    (*info).channels,
                    pcm,
                    &raw mut *(&raw mut scratch.syn as *mut [::core::ffi::c_float; 64])
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut ::core::ffi::c_float,
                );
                igr += 1;
                pcm = pcm
                    .offset((576 as ::core::ffi::c_int * (*info).channels) as isize);
            }
        }
        L3_save_reservoir(dec, &raw mut scratch);
    } else {
        return 0 as ::core::ffi::c_int
    }
    return (success as ::core::ffi::c_uint)
        .wrapping_mul(
            hdr_frame_samples(&raw mut (*dec).header as *mut ::core::ffi::c_uchar),
        ) as ::core::ffi::c_int;
}
