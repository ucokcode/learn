#[cfg(target_os="android")]
mod android;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

fn print_hello() -> &'static str {
    return "hello world from print_hello function";
}

#[no_mangle]
pub unsafe extern "C" fn hello(to: *const c_char) -> *mut c_char {
    let c_str = CStr::from_ptr(to);
    let recipient = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => "you",
    };
    CString::new(format!("Hello from Rust: {} {}", recipient, print_hello())).unwrap().into_raw()
}



#[no_mangle]
pub unsafe extern "C" fn hello_release(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    CString::from_raw(s);
}
