use ex18_lifetime_observer::{clipped_prefix, longer_label, mission_view, MissionView};

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
fn longer_label_returns_longer_slice() {
    let left = "ion";
    let right = "reactor";

    let chosen = call_or_hint!("ex18", "longer_label", longer_label(left, right));
    assert_eq!(chosen, "reactor");
}

#[test]
fn longer_label_returns_left_when_lengths_match() {
    let left = "grid";
    let right = "flux";

    let chosen = call_or_hint!("ex18", "longer_label", longer_label(left, right));
    assert_eq!(chosen, "grid");
}

#[test]
fn mission_view_borrows_both_fields() {
    let code = String::from("A-17");
    let captain = String::from("Rhea");

    let view = call_or_hint!("ex18", "mission_view", mission_view(&code, &captain));
    assert_eq!(
        view,
        MissionView {
            code: "A-17",
            captain: "Rhea"
        }
    );
}

#[test]
fn clipped_prefix_obeys_boundaries() {
    let text = "telemetry";

    let empty = call_or_hint!("ex18", "clipped_prefix", clipped_prefix(text, 0));
    let partial = call_or_hint!("ex18", "clipped_prefix", clipped_prefix(text, 4));
    let full = call_or_hint!("ex18", "clipped_prefix", clipped_prefix(text, 99));

    assert_eq!(empty, "");
    assert_eq!(partial, "tele");
    assert_eq!(full, "telemetry");
}

#[test]
fn lifetime_relationship_allows_borrowed_result_usage() {
    let chosen_owned = {
        let long = String::from("navigator");
        let short = String::from("map");
        call_or_hint!("ex18", "longer_label", longer_label(&long, &short)).to_string()
    };

    assert_eq!(chosen_owned, "navigator");
}
