#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissionView<'a> {
    pub code: &'a str,
    pub captain: &'a str,
}

pub fn longer_label<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

pub fn mission_view<'a>(code: &'a str, captain: &'a str) -> MissionView<'a> {
    MissionView { code, captain }
}

pub fn clipped_prefix<'a>(text: &'a str, max_len: usize) -> &'a str {
    let temp_len: usize = usize::min(max_len, text.len());
    &text[..temp_len]
}
