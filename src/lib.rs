#![feature(str_strip)]

use std::path::PathBuf;
use std::slice;

mod hash40;
pub use hash40::{hash40, Hash40};

extern "C" {
    fn arcrop_load_file(hash: Hash40, buffer: *mut u8, length: u32);
}

pub fn load_original_file<H, B>(hash: H, buffer: B)
where
    H: Into<Hash40>,
    B: AsRef<[u8]>,
{
    unimplemented!()
}
