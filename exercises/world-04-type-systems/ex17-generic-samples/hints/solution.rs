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
    let mut iter = window.samples.iter();
    let mut best = *iter.next()?;

    for &sample in iter {
        if sample > best {
            best = sample;
        }
    }

    Some(best)
}

pub fn format_window<T: Display>(window: &SampleWindow<T>) -> String {
    if window.samples.is_empty() {
        return format!("{} []", window.source);
    }

    let rendered = window
        .samples
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ");

    format!("{} [{rendered}]", window.source)
}
