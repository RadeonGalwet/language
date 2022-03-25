pub fn utf8_slice(s: &str, start: usize, end: usize) -> &str {
    let mut iter = s
        .char_indices()
        .map(|(pos, _)| pos)
        .chain(Some(s.len()))
        .skip(start)
        .peekable();
    let start_pos = *iter.peek().unwrap();
    for _ in start..end {
        iter.next();
    }
    &s[start_pos..*iter.peek().unwrap()]
}
