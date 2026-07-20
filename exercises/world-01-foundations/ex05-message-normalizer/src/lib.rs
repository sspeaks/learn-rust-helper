pub fn normalize_call_sign(input: &str) -> String {
    input.to_uppercase().trim().split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn normalize_manifest(manifest: &[&str]) -> Vec<String> {
    manifest
        .iter()
        .map(|entry| normalize_call_sign(entry))
        .collect()
}
