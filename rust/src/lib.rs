use libc;
use std::ffi::{CStr, CString};


/// Concatenates two strings, or null on error
#[no_mangle]
pub extern "C" fn concat_strs(a: *const libc::c_char, b: *const libc::c_char) -> *mut libc::c_char {
    let (mut a, b) = match (c_char_to_string(a), c_char_to_string(b)) {
        (Ok(sa), Ok(sb)) => (sa, sb),
        (Err(_), _) => return std::ptr::null_mut(),
        (_, Err(_)) => return std::ptr::null_mut(),
    };
    a.push_str(&b);
    
    match CString::new(a.as_bytes()) {
        Ok(c) => c.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

fn c_char_to_string(c_char: *const libc::c_char) -> Result<String, std::string::FromUtf8Error> {
    let buf = unsafe { CStr::from_ptr(c_char).to_bytes() };
    String::from_utf8(buf.to_vec())
}

/// Frees CStrings formerly created in Rust code
#[no_mangle]
pub extern "C" fn free_cstring(s: *mut libc::c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CString};

    #[test]
    fn it_works() {
        let a = CString::new("a").unwrap();
        let b = CString::new("b").unwrap();

        let actual_ptr = concat_strs(a.as_ptr(), b.as_ptr());
        let actual = unsafe { CString::from_raw(actual_ptr) };
        
        let expected = CString::new("ab").unwrap();
        assert_eq!(expected, actual, "a and b concatenated should yield 'ab'");

        // No need to call free_cstring(actual_ptr) here because we already called from_raw above!
    }
}
