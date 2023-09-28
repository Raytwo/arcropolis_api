#![feature(str_strip)]
#![allow(stable_features)]
#![feature(vec_into_raw_parts)]

mod hash40;
pub use hash40::{hash40, Hash40};

mod stream_path;
pub use stream_path::*;
use std::ffi::CString;

pub use arcropolis_api_macro::*;

extern "C" {
    fn arcrop_register_callback(hash: u64, length: usize, cb: CallbackFn);
    fn arcrop_register_callback_with_path(hash: u64, cb: StreamCallbackFn);
    fn arcrop_load_file(hash: u64, buffer: *mut u8, length: usize, out_size: &mut usize) -> bool;
    fn arcrop_api_version() -> &'static ApiVersion;
    fn arcrop_require_api_version(major: u32, minor: u32);
    fn arcrop_register_extension_callback(hash: u64, cb: ExtCallbackFn);
    fn arcrop_get_decompressed_size(hash: u64, out_size: &mut usize) -> bool;
    fn arcrop_register_event_callback(ty: Event, callback: EventCallbackFn);
    fn arcrop_is_file_loaded(hash: u64) -> bool;
    fn arcrop_is_mod_enabled(hash: u64) -> bool;
    fn arcrop_show_mod_manager();
    fn arcrop_show_config_editor();
    fn arcrop_show_main_menu();
    fn arcorp_add_lua_menu_manager(name: *mut u8, reg_vec_ptr: *mut luaL_Reg_to_arcrop, reg_vec_size: usize, reg_vec_cap: usize) -> bool;
    fn arcorp_add_lua_item_manager(name: *mut u8, reg_vec_ptr: *mut luaL_Reg_to_arcrop, reg_vec_size: usize, reg_vec_cap: usize) -> bool;
    fn arcrop_lua_state_get_string(lua_state: &mut lua_state) -> *const u8;
    fn arcrop_lua_state_get_number(lua_state: &mut lua_state) -> f32;
    fn arcrop_lua_state_get_integer(lua_state: &mut lua_state) -> u64;
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

#[arcrop_api(version="1.1")]
pub fn register_stream_callback<H>(hash: H, cb: StreamCallbackFn)
where
    H: Into<Hash40>,
{
    unsafe { arcrop_register_callback_with_path(hash.into().as_u64(), cb) }
}

#[arcrop_api(version="1.2")]
pub fn register_extension_callback<H>(hash: H, cb: ExtCallbackFn)
where
    H: Into<Hash40>,
{
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

#[arcrop_api(version = "1.5")]
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

/// Requires an absolute path, including the ``sd:/`` root.
/// Do NOT include a trailing slash after the directory's name.
#[arcrop_api(version = "1.8")]
pub fn is_mod_enabled<H>(hash: H) -> bool
where
    H: Into<Hash40>
{
    unsafe {
        arcrop_is_mod_enabled(
            hash.into().as_u64()
        )
    }
}

#[arcrop_api(version = "1.7")]
pub fn show_mod_manager() {
    unsafe { arcrop_show_mod_manager(); }
}

#[arcrop_api(version="1.8")]
pub fn show_config_editor() {
    unsafe { arcrop_show_config_editor(); }
}

#[arcrop_api(version="1.8")]
pub fn show_main_menu() {
    unsafe { arcrop_show_main_menu(); }
}

#[arcrop_api(version="1.9")]
pub fn add_lua_menu_manager(name: impl AsRef<str>, functions: Vec<luaL_Reg>) -> bool {
    unsafe {
        let name = CString::new(name.as_ref()).expect(&format!("Failed turning {} into a CString!", name.as_ref()));
        let to_arcrop = functions.iter().map(|x|
            luaL_Reg_to_arcrop {
                name: CString::new(x.name.clone()).expect("Failed!").into_raw(),
                func: x.func
            }
        ).collect::<Vec<luaL_Reg_to_arcrop>>();
        let (ptr, size, cap) = to_arcrop.into_raw_parts();
        arcorp_add_lua_menu_manager(name.into_raw() as _, ptr as _, size, cap)
    }
}

#[arcrop_api(version="1.9")]
pub fn add_lua_item_manager(name: impl AsRef<str>, functions: Vec<luaL_Reg>) -> bool {
    unsafe {
        let name = CString::new(name.as_ref()).expect(&format!("Failed turning {} into a CString!", name.as_ref()));
        let to_arcrop = functions.iter().map(|x|
            luaL_Reg_to_arcrop {
                name: CString::new(x.name.clone()).expect("Failed!").into_raw(),
                func: x.func
            }
        ).collect::<Vec<luaL_Reg_to_arcrop>>();
        let (ptr, size, cap) = to_arcrop.into_raw_parts();
        arcorp_add_lua_item_manager(name.into_raw() as _, ptr as _, size, cap)
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
    pub major: u32,
    pub minor: u32,
}

#[repr(C)]
pub struct lua_state {}

impl lua_state {
    pub fn get_string_arg(&mut self) -> String {
        unsafe {
            let ptr = arcrop_lua_state_get_string(self);
            skyline::from_c_str(ptr)
        }
    }
    pub fn get_number_arg(&mut self) -> f32 {
        unsafe { arcrop_lua_state_get_number(self) }
    }
    pub fn get_integer_arg(&mut self) -> u64 {
        unsafe { arcrop_lua_state_get_integer(self) }
    }
}

pub type LuaCfunction = ::std::option::Option<unsafe extern "C" fn(L: &mut lua_state) -> ::std::os::raw::c_int>;

#[repr(C)]
pub struct luaL_Reg {
    pub name: String,
    pub func: LuaCfunction,
}

#[repr(C)]
struct luaL_Reg_to_arcrop {
    pub name: *mut u8,
    pub func: LuaCfunction,
}