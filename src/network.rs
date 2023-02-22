pub fn is_request_ready(src: &[u8]) -> bool {
    return find_subsequence(src, b"\n\n\n\n").is_some();
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
