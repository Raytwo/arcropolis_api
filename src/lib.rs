#![feature(str_strip)]

use std::{ffi::CString, path::PathBuf};
use std::slice;

mod hash40;
pub use hash40::{hash40, Hash40};

extern "C" {
    fn arcrop_register_callback(hash: u64, length: usize, cb: CallbackFn);
    fn arcrop_register_callback_with_path(hash: u64, length: usize, path: *const u8, cb: CallbackFn);
    fn arcrop_load_file(hash: u64, buffer: *mut u8, length: usize, out_size: &mut usize) -> bool;
    fn arcrop_api_version();
}

// Hash, out_buffer, length
pub type CallbackFn = extern "C" fn(*mut usize, u64, *mut u8, usize) -> bool;
pub type StreamCallbackFn = extern "C" fn(*mut usize, u64, *mut u8, usize) -> bool;

/// /!\ TEMP IMPLEMENTATION, SUBJECT TO CHANGE /!\  
/// Register your callback to ARCropolis.  
/// Do note that, for the time being, hooking a shared file means your callback will be called for all instances of it.
pub fn register_callback<H: Into<Hash40>>(hash: H, length: usize, cb: CallbackFn) {
    unsafe { arcrop_register_callback(hash.into().as_u64(), length, cb) }
}

pub fn register_stream_callback<H>(hash: H, length: usize, path: &str, cb: CallbackFn) 
where
    H: Into<Hash40>
{
    let c_path = CString::new(path).unwrap();
    unsafe { arcrop_register_callback_with_path(hash.into().as_u64(), length, c_path.as_bytes_with_nul().as_ptr(), cb) }
}

/// /!\ TEMP IMPLEMENTATION, SUBJECT TO CHANGE /!\  
/// Provide the original data.arc file, or the user's if one is present.  
/// To be called from within your callback function if you desire to work on an existing file. Make sure the buffer is at least as large as the size you provided when registering.
pub fn load_original_file<H, B>(hash: H, mut buffer: B) -> usize
where
    H: Into<Hash40>,
    B: AsMut<[u8]>,
{
    let buf = buffer.as_mut();

    let mut out_size: usize = 0;

    let result = unsafe { arcrop_load_file(hash.into().as_u64(), buf.as_mut_ptr(), buf.len(), &mut out_size) };

    out_size
}

/// /!\ TEMP IMPLEMENTATION, SUBJECT TO CHANGE /!\  
/// Provide the version of the API supported by the current build of ARCropolis.  
/// Use it to ensure your plugin is still compatible before performing API calls.
pub fn get_api_version() {
    unsafe { arcrop_api_version() }
    unimplemented!()
}