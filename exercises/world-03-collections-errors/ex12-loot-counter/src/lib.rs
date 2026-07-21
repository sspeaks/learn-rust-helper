use std::collections::HashMap;

pub fn count_loot(items: &[&str]) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for &item in items {
        *counts.entry(item.to_string()).or_insert(0) += 1;
    }
    counts
}

pub fn total_items(counts: &HashMap<String, usize>) -> usize {
    counts.values().sum()
}
