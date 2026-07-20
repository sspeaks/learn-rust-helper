pub fn normalize_call_sign(input: &str) -> String {
    input
        .split_whitespace()
        .map(str::to_uppercase)
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn normalize_manifest(manifest: &[&str]) -> Vec<String> {
    manifest
        .iter()
        .map(|entry| normalize_call_sign(entry))
        .collect()
}
