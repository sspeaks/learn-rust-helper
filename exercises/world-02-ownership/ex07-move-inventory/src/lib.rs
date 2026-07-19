#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupplyCrate {
    pub id: String,
    pub contents: Vec<String>,
}

pub fn move_crate_to_shuttle(
    manifest: &mut Vec<SupplyCrate>,
    crate_id: &str,
) -> Option<SupplyCrate> {
    todo!("Find a crate by id, remove it from manifest, and move ownership to caller")
}
