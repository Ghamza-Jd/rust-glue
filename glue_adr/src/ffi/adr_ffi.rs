use glue_core::{gcd, record_custom_event};

use std::ffi::CStr;
use std::os::raw::c_char;
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

// https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::jstring;
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_proximie_glue_EventTracker_record_custom_event_glue(
        env: JNIEnv,
        _: JClass,
        java_pattern: JString,
    ) {
        record_custom_event_glue(
            env.get_string(java_pattern)
                .expect("invalid pattern string")
                .as_ptr(),
        );
    }
}
