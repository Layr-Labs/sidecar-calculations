pub mod nile_calculations;
pub mod math;
pub mod amazon_calculations;
pub mod token_calculations;


use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn free_c_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return }
        let _ = CString::from_raw(s);
    };
}
