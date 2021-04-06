#![feature(str_strip)]

use std::path::PathBuf;
use std::slice;

mod hash40;
pub use hash40::{hash40, Hash40};

extern "C" {
    fn arcrop_register_callback(hash: Hash40);
    fn arcrop_load_file(hash: Hash40, buffer: *mut u8, length: usize);
    fn arcrop_api_version();
}

/// /!\ TEMP IMPLEMENTATION, SUBJECT TO CHANGE /!\  
/// Register your callback to ARCropolis.  
/// Do note that, for the time being, hooking a shared file means your callback will be called for all instances of it.
pub fn register_callback<H: Into<Hash40>>(hash: H) {
    unsafe { arcrop_register_callback(hash.into()) }
}

/// /!\ TEMP IMPLEMENTATION, SUBJECT TO CHANGE /!\  
/// Provide the original data.arc file, or the user's if one is present.  
/// To be called from within your callback function if you desire to work on an existing file. Make sure the buffer is at least as large as the size you provided when registering.
pub fn load_original_file<H, B>(hash: H, mut out_buffer: B)
where
    H: Into<Hash40>,
    B: AsMut<[u8]>,
{
    let buf = out_buffer.as_mut();

    unsafe { arcrop_load_file(hash.into(), buf.as_mut_ptr(), buf.len()) }
}

/// /!\ TEMP IMPLEMENTATION, SUBJECT TO CHANGE /!\  
/// Provide the version of the API supported by the current build of ARCropolis.  
/// Use it to ensure your plugin is still compatible before performing API calls.
pub fn get_api_version() {
    unsafe { arcrop_api_version() }
}