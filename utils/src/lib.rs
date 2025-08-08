pub mod radix_array;
pub use radix_array::*;

pub fn intersperse<I>(iter: I, arg: &str) -> String
where
    I: Iterator<Item = String>,
{
    let mut iter = iter.peekable();
    let mut string = String::new();
    loop {
        match iter.next() {
            Some(s) => string.push_str(s.as_str()),
            None => break,
        }
        if iter.peek().is_some() {
            string.push_str(arg);
        }
    }
    string
}

pub fn flush_cache() {
    let mut buffer = vec![0u8; 128 * 1024 * 1024];
    for i in (0..buffer.len()).step_by(64) {
        buffer[i] = i as u8;
    }
    std::hint::black_box(buffer);
}
