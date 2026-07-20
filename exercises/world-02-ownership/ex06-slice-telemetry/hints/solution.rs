pub fn first_sentence(report: &str) -> &str {
    match report.find('.') {
        Some(pos) => &report[..=pos],
        None => report,
    }
}

pub fn trailing_readings(readings: &[i32], count: usize) -> &[i32] {
    let start = readings.len().saturating_sub(count);
    &readings[start..]
}
