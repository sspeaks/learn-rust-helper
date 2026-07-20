pub fn normalize_call_sign(input: &str) -> String {
    // ════════════════════════════════════════════════════════════════════════════
    // 🚀 YOUR MISSION: Replace the todo!() below with your solution.
    // ════════════════════════════════════════════════════════════════════════════
    todo!("Normalize spacing/casing in this call sign and return an owned String")
}

pub fn normalize_manifest(manifest: &[&str]) -> Vec<String> {
    manifest
        .iter()
        .map(|entry| normalize_call_sign(entry))
        .collect()
}
