/// Concatenates two strings
pub fn concatenate_strings(a: &Option<&str>, b: &Option<&str>) -> Option<String> {
    match (a, b) {
        (Some(a), Some(b)) => Some(format!("{}{}", a, b)),
        (None, _) => None,
        (_, None) => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let actual = concatenate_strings(&Some("a"), &Some("b"));
        let expected = Some(String::from("ab"));
        assert_eq!(expected, actual, "should yield '{:?}', got '{:?}'", expected, actual);
    }

    #[test]
    fn none_on_empty_string() {
        let actual = concatenate_strings(&None, &Some("b"));
        let expected = None;
        assert_eq!(expected, actual, "should yield '{:?}', got '{:?}'", expected, actual);
    }
}
