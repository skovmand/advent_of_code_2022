use regex::Captures;

pub fn regex_capture_to_u8(captures: &Captures, index: usize) -> u8 {
    captures
        .get(index)
        .unwrap()
        .as_str()
        .parse::<u8>()
        .expect("parse capture to u8")
}
