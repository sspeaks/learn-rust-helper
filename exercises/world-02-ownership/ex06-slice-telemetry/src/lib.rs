pub fn first_sentence(report: &str) -> &str {
    for (ind, byte) in report.as_bytes().iter().enumerate() {
        if *byte == b'.' {
            return &report[0..ind+1];
        }
    }
    report

}

pub fn trailing_readings(readings: &[i32], count: usize) -> &[i32] {
    let len = readings.len();
    if count >= len { return readings; }
    &readings[len-count..len]
}
