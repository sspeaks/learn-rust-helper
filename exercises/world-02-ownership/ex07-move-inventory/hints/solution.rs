#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupplyCrate {
    pub id: String,
    pub contents: Vec<String>,
}

pub fn move_crate_to_shuttle(
    manifest: &mut Vec<SupplyCrate>,
    crate_id: &str,
) -> Option<SupplyCrate> {
    let pos = manifest.iter().position(|c| c.id == crate_id)?;
    Some(manifest.remove(pos))
}
