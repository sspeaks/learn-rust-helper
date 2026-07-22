use ex26_sensor_array::{
    calibrate_readings_parallel, total_calibrated_power_parallel, CalibratedReading, SensorReading,
};

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

fn reading(id: &str, value: i64) -> SensorReading {
    SensorReading {
        sensor_id: id.to_string(),
        raw_value: value,
    }
}

#[test]
fn calibrate_empty_slice_returns_empty_vec() {
    let calibrated = call_or_hint!(
        "ex26",
        "calibrate_readings_parallel",
        calibrate_readings_parallel(&[], 5)
    );

    assert!(calibrated.is_empty());
}

#[test]
fn calibrate_single_reading_applies_offset() {
    let calibrated = call_or_hint!(
        "ex26",
        "calibrate_readings_parallel",
        calibrate_readings_parallel(&[reading("s1", 10)], 3)
    );

    assert_eq!(
        calibrated,
        vec![CalibratedReading {
            sensor_id: "s1".to_string(),
            calibrated_value: 13,
        }]
    );
}

#[test]
fn calibrate_multiple_readings_preserves_order_and_ids() {
    let input = vec![reading("a", 1), reading("b", 2), reading("c", 3)];
    let calibrated = call_or_hint!(
        "ex26",
        "calibrate_readings_parallel",
        calibrate_readings_parallel(&input, 10)
    );

    let ids: Vec<&str> = calibrated
        .iter()
        .map(|item| item.sensor_id.as_str())
        .collect();
    let values: Vec<i64> = calibrated
        .iter()
        .map(|item| item.calibrated_value)
        .collect();

    assert_eq!(ids, vec!["a", "b", "c"]);
    assert_eq!(values, vec![11, 12, 13]);
}

#[test]
fn calibrate_supports_negative_offsets() {
    let input = vec![reading("a", 5), reading("b", -2)];
    let calibrated = call_or_hint!(
        "ex26",
        "calibrate_readings_parallel",
        calibrate_readings_parallel(&input, -4)
    );

    let values: Vec<i64> = calibrated
        .iter()
        .map(|item| item.calibrated_value)
        .collect();
    assert_eq!(values, vec![1, -6]);
}

#[test]
fn total_power_empty_slice_is_zero() {
    let total = call_or_hint!(
        "ex26",
        "total_calibrated_power_parallel",
        total_calibrated_power_parallel(&[])
    );

    assert_eq!(total, 0);
}

#[test]
fn total_power_sums_positive_values() {
    let readings = vec![
        CalibratedReading {
            sensor_id: "a".to_string(),
            calibrated_value: 7,
        },
        CalibratedReading {
            sensor_id: "b".to_string(),
            calibrated_value: 9,
        },
        CalibratedReading {
            sensor_id: "c".to_string(),
            calibrated_value: 11,
        },
    ];

    let total = call_or_hint!(
        "ex26",
        "total_calibrated_power_parallel",
        total_calibrated_power_parallel(&readings)
    );

    assert_eq!(total, 27);
}

#[test]
fn total_power_handles_mixed_sign_values() {
    let readings = vec![
        CalibratedReading {
            sensor_id: "a".to_string(),
            calibrated_value: -3,
        },
        CalibratedReading {
            sensor_id: "b".to_string(),
            calibrated_value: 10,
        },
        CalibratedReading {
            sensor_id: "c".to_string(),
            calibrated_value: -1,
        },
    ];

    let total = call_or_hint!(
        "ex26",
        "total_calibrated_power_parallel",
        total_calibrated_power_parallel(&readings)
    );

    assert_eq!(total, 6);
}

#[test]
fn calibrate_then_total_matches_manual_sum() {
    let input = vec![reading("a", 2), reading("b", 4), reading("c", 6)];
    let calibrated = call_or_hint!(
        "ex26",
        "calibrate_readings_parallel",
        calibrate_readings_parallel(&input, 5)
    );

    let total = call_or_hint!(
        "ex26",
        "total_calibrated_power_parallel",
        total_calibrated_power_parallel(&calibrated)
    );

    assert_eq!(total, 27, "(2+5) + (4+5) + (6+5) = 27");
}
