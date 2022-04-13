#![feature(str_strip)]
#![allow(stable_features)]

mod hash40;
pub use hash40::{hash40, Hash40};

mod stream_path;
use smash_arc::LoadedArc;
pub use stream_path::*;

pub use arcropolis_api_macro::*;

extern "C" {
    fn arcrop_register_callback(hash: u64, length: usize, cb: CallbackFn);
    fn arcrop_register_callback_with_path(hash: u64, cb: StreamCallbackFn);
    fn arcrop_load_file(hash: u64, buffer: *mut u8, length: usize, out_size: &mut usize) -> bool;
    fn arcrop_api_version() -> &'static ApiVersion;
    fn arcrop_require_api_version(major: u32, minor: u32);
    fn arcrop_register_extension_callback(hash: u64, cb: ExtCallbackFn);
    fn arcrop_get_decompressed_size(hash: u64, out_size: &mut usize) -> bool;
    fn arcrop_get_loaded_arc(out: &mut &'static LoadedArc) -> bool;
    fn arcrop_register_event_callback(ty: Event, callback: EventCallbackFn);
    fn arcrop_is_file_loaded(hash: u64) -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Event {
    ArcFilesystemMounted,
    ModFilesystemMounted,
}

pub type EventCallbackFn = extern "C" fn(Event);

// Hash, out_buffer, length, out_size
pub type CallbackFn = extern "C" fn(u64, *mut u8, usize, &mut usize) -> bool;
// Hash, out_path, out_size
pub type StreamCallbackFn = extern "C" fn(u64, *mut u8, &mut usize) -> bool;

// Extension hash, out_buffer, length, out_size
pub type ExtCallbackFn = extern "C" fn(u64, *mut u8, usize, &mut usize) -> bool;

pub fn register_callback<H: Into<Hash40>>(hash: H, length: usize, cb: CallbackFn) {
    unsafe { arcrop_register_callback(hash.into().as_u64(), length, cb) }
}

pub fn register_stream_callback<H>(hash: H, cb: StreamCallbackFn)
where
    H: Into<Hash40>,
{
    require_api_version(1, 1);
    unsafe { arcrop_register_callback_with_path(hash.into().as_u64(), cb) }
}

pub fn register_extension_callback<H>(hash: H, cb: ExtCallbackFn)
where
    H: Into<Hash40>,
{
    require_api_version(1, 2);
    unsafe { arcrop_register_extension_callback(hash.into().as_u64(), cb) }
}

pub fn load_original_file<H, B>(hash: H, mut buffer: B) -> Option<usize>
where
    H: Into<Hash40>,
    B: AsMut<[u8]>,
{
    let buf = buffer.as_mut();

    let mut out_size: usize = 0;

    let success = unsafe {
        arcrop_load_file(
            hash.into().as_u64(),
            buf.as_mut_ptr(),
            buf.len(),
            &mut out_size,
        )
    };

    if success {
        Some(out_size)
    } else {
        None
    }
}

pub fn is_file_loaded<H>(hash: H) -> bool
where
    H: Into<Hash40>
{   
    unsafe {
        arcrop_is_file_loaded(
            hash.into().as_u64()
        )
    }
}

pub fn get_api_version() -> &'static ApiVersion {
    unsafe { arcrop_api_version() }
}

pub fn require_api_version(major: u32, minor: u32) {
    unsafe { arcrop_require_api_version(major, minor) }
}

#[repr(C)]
pub struct ApiVersion {
    major: u32,
    minor: u32,
}
