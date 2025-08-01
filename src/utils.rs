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
