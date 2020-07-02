use wasm_bindgen::prelude::*;
use rust_lib;
use js_sys;

#[wasm_bindgen]
pub fn concatenate_strings(a: String, b: String) -> Result<String, JsValue> {
    let a = Some(a.as_str());
    let b = Some(b.as_str());

    rust_lib::concatenate_strings(&a, &b)
        .or_else(map_to_js_error)
}

fn map_to_js_error(_err: rust_lib::ConcatenateStringsError) -> Result<String, JsValue> {
    // TODO Error::new takes &str and thus only allows static strings. Would be nice to have sth for String to actually
    // pass on the given error message...
    Err(js_sys::Error::new("concatenation error").into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = concatenate_strings(String::from("a"), String::from("b"));
        let expected = Ok(String::from("ab"));
        assert_eq!(expected, actual, "expected result to be 'ab', got '{:?}'", actual);
    }
}
