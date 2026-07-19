use std::collections::HashMap;

pub fn count_loot(items: &[&str]) -> HashMap<String, usize> {
    todo!("Count each item occurrence into a HashMap<String, usize>")
}

pub fn total_items(counts: &HashMap<String, usize>) -> usize {
    counts.values().sum()
}
