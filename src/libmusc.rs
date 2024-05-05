
use crate::*;
use std::ffi::{c_char, CString};

#[no_mangle]
pub extern "C" fn PitchStep_ToNumber(step: PitchStep) -> u8 {
    return step.to_number();
}

#[no_mangle]
pub extern "C" fn PitchStep_ToString(step: PitchStep) -> *const c_char {
    let s1 = step.to_string();
    let s2 = CString::new(s1).unwrap();
    return s2.into_raw();
}
