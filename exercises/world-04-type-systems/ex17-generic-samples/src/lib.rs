use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SampleWindow<T> {
    pub source: String,
    pub samples: Vec<T>,
}

pub fn newest_sample<T: Clone>(window: &SampleWindow<T>) -> Option<T> {
    window.samples.last().cloned()
}

pub fn strongest_sample<T: PartialOrd + Copy>(window: &SampleWindow<T>) -> Option<T> {
    window.samples.iter().max_by(|a, b| a.partial_cmp(b).expect("Values should be comparable")).copied()
}

pub fn format_window<T: Display>(window: &SampleWindow<T>) -> String {
    format!("{} [{}]", window.source, window.samples.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", "))
}
