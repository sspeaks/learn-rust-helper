use ex17_generic_samples::{format_window, newest_sample, strongest_sample, SampleWindow};

fn is_stub_panic(e: &Box<dyn std::any::Any + Send>) -> bool {
    e.downcast_ref::<&str>()
        .is_some_and(|s| s.contains("not yet implemented"))
        || e.downcast_ref::<String>()
            .is_some_and(|s| s.contains("not yet implemented"))
}

macro_rules! call_or_hint {
    ($ex:expr, $fn:expr, $body:expr) => {{
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| $body)) {
            Ok(v) => v,
            Err(e) => {
                if is_stub_panic(&e) {
                    panic!(
                        "\n\n  ✖  {} '{}' not started — fill in src/lib.rs\n",
                        $ex, $fn
                    );
                }
                std::panic::resume_unwind(e)
            }
        }
    }};
}

#[test]
fn newest_sample_returns_none_for_empty_window() {
    let window: SampleWindow<i32> = SampleWindow {
        source: "alpha".to_string(),
        samples: vec![],
    };

    let newest = call_or_hint!("ex17", "newest_sample", newest_sample(&window));
    assert_eq!(newest, None);
}

#[test]
fn newest_sample_clones_last_value() {
    let window = SampleWindow {
        source: "beta".to_string(),
        samples: vec!["cold".to_string(), "warm".to_string(), "hot".to_string()],
    };

    let newest = call_or_hint!("ex17", "newest_sample", newest_sample(&window));
    assert_eq!(newest, Some("hot".to_string()));
}

#[test]
fn strongest_sample_works_for_signed_integers() {
    let window = SampleWindow {
        source: "gamma".to_string(),
        samples: vec![-10, 4, 2, 4, -3],
    };

    let strongest = call_or_hint!("ex17", "strongest_sample", strongest_sample(&window));
    assert_eq!(strongest, Some(4));
}

#[test]
fn strongest_sample_supports_other_ordered_types() {
    let window = SampleWindow {
        source: "delta".to_string(),
        samples: vec!['a', 'x', 'm'],
    };

    let strongest = call_or_hint!("ex17", "strongest_sample", strongest_sample(&window));
    assert_eq!(strongest, Some('x'));
}

#[test]
fn strongest_sample_returns_none_for_empty_window() {
    let window: SampleWindow<u64> = SampleWindow {
        source: "epsilon".to_string(),
        samples: vec![],
    };

    let strongest = call_or_hint!("ex17", "strongest_sample", strongest_sample(&window));
    assert_eq!(strongest, None);
}

#[test]
fn format_window_renders_empty_and_non_empty_windows() {
    let empty: SampleWindow<i32> = SampleWindow {
        source: "zeta".to_string(),
        samples: vec![],
    };
    let values = SampleWindow {
        source: "eta".to_string(),
        samples: vec![3, 5, 8],
    };

    let empty_text = call_or_hint!("ex17", "format_window", format_window(&empty));
    let values_text = call_or_hint!("ex17", "format_window", format_window(&values));

    assert_eq!(empty_text, "zeta []");
    assert_eq!(values_text, "eta [3, 5, 8]");
}
