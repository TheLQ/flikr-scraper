pub fn last_position_of(input: &str, needle: u8) -> usize {
    input
        .as_bytes()
        .iter()
        .enumerate()
        .filter(|(_i, c)| **c == needle)
        .next_back()
        .unwrap()
        .0
}
