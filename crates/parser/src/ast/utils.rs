

pub fn resolve_line_endings<'a>(len: usize) -> &'a str {
    match len {
        1 => "\n",
        _ => "\r\n"
    }
}