extern crate build_details_test;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use build_details_test::build_details::*;

#[test]
fn timestamp_is_recent() {
    let now = SystemTime::now();
    let built = UNIX_EPOCH + Duration::from_secs(TIMESTAMP.unwrap());

    let delta = now.duration_since(built).unwrap();

    assert!(delta.as_secs() < 30 * 60 * 60 * 24);
}

#[test]
fn authors() {
    let expected = "Sam Wilson <tecywiz121@hotmail.com>:John Smith <jsmith@example.com>";
    assert_eq!(expected, AUTHORS.unwrap());
}

#[test]
fn rust_flags() {
    // Can't exactly control what the value of RUSTFLAGS was during the build.
    // Just verify that it exists I guess?
    let _flags: Option<&'static str> = RUST_FLAGS;
}

#[test]
fn homepage() {
    let expected = "http://example.com/?a_weird_character=\"\"";
    assert_eq!(expected, HOMEPAGE.unwrap());
}

#[test]
fn profile() {
    match PROFILE {
        Some("debug") | Some("release") => (),
        _ => panic!("expected profile to be 'debug' or 'release'"),
    }
}

#[test]
fn version() {
    assert_eq!(Some("0.1.0"), VERSION);
}

#[test]
fn name() {
    assert_eq!(Some("build_details_test"), NAME);
}

#[test]
fn description() {
    assert_eq!(Some("Crate for testing\nbuild_details"), DESCRIPTION);
}

#[test]
#[cfg(unix)]
fn cfg_unix() {
    let cfg = CFG.unwrap();

    assert!(cfg.contains_key("UNIX"));
}

#[test]
#[cfg(windows)]
fn cfg_windows() {
    let cfg = CFG.unwrap();

    assert!(cfg.contains_key("WINDOWS"));
}

#[test]
#[cfg(target_pointer_width = "32")]
fn cfg_32() {
    let cfg = CFG.unwrap();
    assert_eq!(cfg.get("TARGET_POINTER_WIDTH"), Some(&"32"));
}

#[test]
#[cfg(target_pointer_width = "64")]
fn cfg_64() {
    let cfg = CFG.unwrap();
    assert_eq!(cfg.get("TARGET_POINTER_WIDTH"), Some(&"64"));
}

#[test]
fn features_on() {
    assert!(FEATURES.unwrap().contains(&"ON_BY_DEFAULT"));
}

#[test]
fn features_off() {
    assert!(!FEATURES.unwrap().contains(&"OFF_BY_DEFAULT"));
}
