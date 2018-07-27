extern crate build_details_test;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use build_details_test::required_build_details::*;

#[test]
fn timestamp_is_recent() {
    let now = SystemTime::now();
    let built = UNIX_EPOCH + Duration::from_secs(TIMESTAMP);

    let delta = now.duration_since(built).unwrap();

    assert!(delta.as_secs() < 30 * 60 * 60 * 24);
}

#[test]
fn authors() {
    let expected = "Sam Wilson <tecywiz121@hotmail.com>:John Smith <jsmith@example.com>";
    assert_eq!(expected, AUTHORS);
}

#[test]
fn homepage() {
    let expected = "http://example.com/?a_weird_character=\"\"";
    assert_eq!(expected, HOMEPAGE);
}

#[test]
fn profile() {
    match PROFILE {
        "debug" | "release" => (),
        _ => panic!("expected profile to be 'debug' or 'release'"),
    }
}

#[test]
fn version() {
    assert_eq!("0.1.0", VERSION);
}

#[test]
fn name() {
    assert_eq!("build_details_test", NAME);
}

#[test]
fn description() {
    assert_eq!("Crate for testing\nbuild_details", DESCRIPTION);
}

#[test]
#[cfg(unix)]
fn cfg_unix() {
    assert!(CFG.contains_key("UNIX"));
}

#[test]
#[cfg(windows)]
fn cfg_windows() {
    assert!(CFG.contains_key("WINDOWS"));
}

#[test]
#[cfg(target_pointer_width = "32")]
fn cfg_32() {
    assert_eq!(CFG.get("TARGET_POINTER_WIDTH"), Some(&"32"));
}

#[test]
#[cfg(target_pointer_width = "64")]
fn cfg_64() {
    assert_eq!(CFG.get("TARGET_POINTER_WIDTH"), Some(&"64"));
}

#[test]
fn features_on() {
    assert!(FEATURES.contains(&"ON_BY_DEFAULT"));
}

#[test]
fn features_off() {
    assert!(!FEATURES.contains(&"OFF_BY_DEFAULT"));
}
