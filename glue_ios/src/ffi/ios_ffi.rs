use glue_core::{gcd, record_custom_event};

use libc::c_char;
use std::ffi::CStr;
use std::str;

#[no_mangle]
pub extern "C" fn rust_gcd(n: u64, m: u64) -> u64 {
    return gcd(n, m);
}

#[no_mangle]
pub extern "C" fn record_custom_event_glue(event_type: *const c_char) {
    let c_str: &CStr = unsafe { CStr::from_ptr(event_type) };
    let str_slice: &str = c_str.to_str().unwrap();
    record_custom_event(str_slice);
}
