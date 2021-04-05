#![feature(str_strip)]

use std::path::PathBuf;
use std::slice;

mod hash40;
pub use hash40::{hash40, Hash40};

extern "C" {
    fn arcrop_register_callback(hash: Hash40);
    fn arcrop_load_file(hash: Hash40, buffer: *mut u8, length: usize);
}

pub fn register_callback<H: Into<Hash40>>(hash: H) {
    unsafe { arcrop_register_callback(hash.into()) }
}

pub fn load_original_file<H, B>(hash: H, mut out_buffer: B)
where
    H: Into<Hash40>,
    B: AsMut<[u8]>,
{
    let buf = out_buffer.as_mut();

    unsafe { arcrop_load_file(hash.into(), buf.as_mut_ptr(), buf.len()) }
}
