#![feature(str_strip)]

use std::slice;
use std::path::PathBuf;
pub use arcropolis_core::arc::*;
pub use arcropolis_core::core::*;
pub use arcropolis_core::core::callbacks::*;

extern "C" {
    fn has_hash(hash: u64) -> bool;
    fn get_arc_file(hash: u64) -> *mut ArcInfo;
    fn get_arc_file_path(hash: u64) -> *mut ArcPath;
    fn get_file(hash: u64) -> *mut ArcFile;
    fn discover(path: *const ArcPath, umm: bool);
    fn install_callback(cb_type: CallbackType, arc_cb: ArcCallback);
}

/// Asks Arcropolis-core is a file is associated to this hash.
/// If one is found, relevant informations about it are returned as a ArcFile.
pub fn get_file_info(hash: u64) -> Option<Box<ArcInfo>> {
    unsafe {
        if has_hash(hash) {
            Some(Box::from_raw(get_arc_file(hash)))
        } else {
            None
        }
    }
}

/// Asks Arcropolis-core is a file is associated to this hash.
/// If one is found, the absolute path to the user's file will be returned
pub fn get_file_path(hash: u64) -> Option<PathBuf> {
    unsafe {
        if has_hash(hash) {
            let path = Box::from_raw(get_arc_file_path(hash));
            Some(path.path().unwrap())
        } else {
            None
        }
    }
}

/// Asks Arcropolis-core is a file is associated to this hash.
/// If one is found and a callback is registered, the file will go through the callback before being returned.
pub fn get_file_content<'a>(hash: u64) -> Option<Box<[u8]>> {
    unsafe {
        if has_hash(hash) {
            let result = get_file(hash);
            Some(Box::from_raw(slice::from_raw_parts_mut((*result).file, (*result).len as _) as *mut [u8]))
        } else {
            None
        }
    }
}

// TODO: Should probably change this to return a Result if a callback is not accepted?
// TODO: Make sure there is a matching hash if we receive a CallbackType::File?
pub fn register_callback(infos: CallbackType, callback: ArcCallback) {
    unsafe {
        install_callback(infos, callback);
    }
}

/// Calling this does not necessarily mean every path is going to be read.
pub fn discover_files<T: AsRef<[u8]>>(path: T, umm: bool) {
    unsafe {
        discover(&ArcPath::new(path), umm)
    }
}