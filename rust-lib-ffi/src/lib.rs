#![allow(clippy::missing_safety_doc)]

use licensorlib as licensor;
use licensor::{Feature, Evaluator};

use std::convert::TryFrom;
use std::ffi::{CStr, CString};

use libc::c_char;
use chrono::Duration;

/// It turns out that:
///  - cbindgen does not support:
///    - tuple return types (why?) => no Go-style results
///  - cgo does not support:
///    - unions (because Go does not have the), to which Rust enums are translated to => no Rust-style return types
/// This makes this a tad more cumbersome than expected as we have to manually implement our Result type
#[repr(C)]
pub struct Result {
    evaluator: *mut Eval,
    pub err: *mut c_char,
}

impl Result {
    fn ok(evaluator: Evaluator) -> Result {
        let evaluator = Box::into_raw(Box::new(Eval(evaluator)));
        Result {
            evaluator,
            err: std::ptr::null_mut()
        }
    }
    fn err(msg: &str) -> Result {
        Result {
            evaluator: std::ptr::null_mut(),
            err: string_to_c_char(msg),
        }
    }
}

/// Also, cbindgend does not support associated functions (because C doen't have namespaces?)
#[no_mangle]
pub extern "C" fn result_is_ok(result: &Result) -> bool {
    !result.evaluator.is_null()
}

#[no_mangle]
pub unsafe extern "C" fn free_result(result: Result) {
    if result_is_ok(&result) {
        Box::from_raw(result.evaluator);
    } else {
        free_cstring(result.err);
    }
}

#[repr(C)]
pub struct BoolResult {
    pub result: bool,
    pub err: *mut c_char,
}

#[no_mangle]
pub extern "C" fn bool_is_ok(r: &BoolResult) -> bool {
    r.err.is_null()
}

#[no_mangle]
pub unsafe extern "C" fn free_bool_result(result: BoolResult) {
    if !bool_is_ok(&result) {
        free_cstring(result.err);
    }
}

/// We need the extra-indirection so we don't need to expose Evaluator's layout. This translates to an typedef
pub struct Eval(Evaluator);

/// This creates a new Evaluator for the given key and domain to check against
#[no_mangle]
pub extern "C" fn create_from_license_key(key: *const c_char, domain: *const c_char) -> Result {
    let key = c_char_to_string(key);
    let domain = c_char_to_string(domain);

    let eval = licensor::from_license_key_bytes(key, domain);
    if let Err(e) = eval {
        return Result::err(&format!("{}", e));
    }
    let eval = eval.unwrap();

    // we don't have to call std::mem::forget on anything as Result owns everything!
    Result::ok(eval)
}

#[no_mangle]
pub extern "C" fn validate(eval: &Eval) -> BoolResult {
    match eval.0.validate() {
        Ok(_) => BoolResult{ result: true, err: std::ptr::null_mut()},
        Err(e) => BoolResult{ result: false, err: string_to_c_char(&format!("{}", e))},
    }
}

#[no_mangle]
pub extern "C" fn enabled(eval: &Eval, feature: *const c_char) -> bool {
    let feature = c_char_to_string(feature);
    let feature = match Feature::try_from(feature.as_str()) {
        Ok(f) => f,
        Err(_) => return false,
    };
    eval.0.enabled(&feature)
}

#[no_mangle]
pub extern "C" fn has_enough_seats(eval: &Eval, seats: i32) -> bool {
    eval.0.has_enough_seats(seats)
}

#[no_mangle]
pub extern "C" fn can_use_prebuild(eval: &Eval, total_prebuild_time_spent_seconds: i64) -> bool {
    let total_pbt = Duration::seconds(total_prebuild_time_spent_seconds);
    eval.0.can_use_prebuild(&total_pbt)
}

#[no_mangle]
pub extern "C" fn evaluator_inspect(eval: &Eval) -> (*mut c_char, *mut c_char) {
    match licensor::inspect_str(&eval.0) {
        Ok(v) => (string_to_c_char(v.as_str()), std::ptr::null_mut()),
        Err(e) => (std::ptr::null_mut(), string_to_c_char(&format!("{}", e))),
    }
}

/// Wraps a *char (owned by the C-side) into a Rust-y String
fn c_char_to_string(c_char: *const libc::c_char) -> String {
    let buf = unsafe { CStr::from_ptr(c_char).to_bytes() };
    String::from_utf8_lossy(buf).into_owned()
}

/// Copies a String into a CString to hand it off to the C-side (!!! be sure to call free_cstring afterwards !!!)
fn string_to_c_char(s: &str) -> *mut c_char {
    CString::new(s).unwrap().into_raw()
}

/// Frees CStrings formerly created in Rust code using CString::from_raw
#[no_mangle]
pub unsafe extern "C" fn free_cstring(s: *mut libc::c_char) {
    if s.is_null() {
        return;
    }
    CString::from_raw(s);
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::ffi::{CString};

//     #[test]
//     fn it_works() {
//         let a = CString::new("a").unwrap();
//         let b = CString::new("b").unwrap();

//         let actual_ptr = concat_strs(a.as_ptr(), b.as_ptr());
//         let actual = unsafe { CString::from_raw(actual_ptr) };
        
//         let expected = CString::new("ab").unwrap();
//         assert_eq!(expected, actual, "a and b concatenated should yield 'ab'");

//         // No need to call free_cstring(actual_ptr) here because we already called from_raw above!
//     }
// }
