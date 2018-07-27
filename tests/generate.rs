// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

extern crate build_details;
#[macro_use]
extern crate lazy_static;
extern crate tempfile;

use build_details::error::Error;
use build_details::{BuildDetail, BuildDetails};

use std::io::prelude::*;
use std::io::SeekFrom;
use std::sync::Mutex;

use tempfile::tempfile;

#[test]
fn version_required() {
    let mut file = tempfile().unwrap();

    BuildDetails::none()
        .require(BuildDetail::Version)
        .write_to(&mut file)
        .unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

    let mut actual = String::new();
    file.read_to_string(&mut actual).unwrap();

    assert_eq!(
        "pub const VERSION: &\'static str = env!(\"CARGO_PKG_VERSION\");\n",
        &actual
    );
}

#[test]
fn version_optional() {
    let mut file = tempfile().unwrap();

    BuildDetails::none()
        .include(BuildDetail::Version)
        .write_to(&mut file)
        .unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

    let mut actual = String::new();
    file.read_to_string(&mut actual).unwrap();

    assert_eq!(
        "pub const VERSION: Option<&\'static str> = option_env!(\"CARGO_PKG_VERSION\");\n",
        &actual
    );
}

#[test]
fn timestamp_required() {
    let mut file = tempfile().unwrap();

    BuildDetails::none()
        .require(BuildDetail::Timestamp)
        .write_to(&mut file)
        .unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

    let mut actual = String::new();
    file.read_to_string(&mut actual).unwrap();

    assert!(actual.starts_with("pub const TIMESTAMP: u64 ="));
    assert!(actual.ends_with(";\n"));
}

#[test]
fn timestamp_optional() {
    let mut file = tempfile().unwrap();

    BuildDetails::none()
        .include(BuildDetail::Timestamp)
        .write_to(&mut file)
        .unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

    let mut actual = String::new();
    file.read_to_string(&mut actual).unwrap();

    assert!(actual.starts_with("pub const TIMESTAMP: Option<u64> = Some("));
    assert!(actual.ends_with(");\n"));
}

lazy_static! {
    static ref PROFILE: Mutex<()> = Mutex::new(());
}

#[test]
fn profile_required_missing() {
    let mut file = tempfile().unwrap();

    let lock = PROFILE.lock().unwrap();

    ::std::env::remove_var("PROFILE");

    let result = BuildDetails::none()
        .require(BuildDetail::Profile)
        .write_to(&mut file)
        .unwrap_err();

    match result {
        Error::MissingDetail(ref x) if x == "PROFILE" => (),
        _ => panic!("Expected Error::MissingDetail(PROFILE)"),
    }

    ::std::mem::drop(lock);
}

#[test]
fn profile_required_available() {
    let mut file = tempfile().unwrap();

    let lock = PROFILE.lock().unwrap();

    ::std::env::set_var("PROFILE", "abc");

    BuildDetails::none()
        .require(BuildDetail::Profile)
        .write_to(&mut file)
        .unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

    let mut actual = String::new();
    file.read_to_string(&mut actual).unwrap();

    assert_eq!("pub const PROFILE: &\'static str = \"abc\";\n", &actual);

    ::std::mem::drop(lock);
}

#[test]
fn profile_optional_available() {
    let mut file = tempfile().unwrap();

    let lock = PROFILE.lock().unwrap();

    ::std::env::set_var("PROFILE", "123");

    BuildDetails::none()
        .include(BuildDetail::Profile)
        .write_to(&mut file)
        .unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

    let mut actual = String::new();
    file.read_to_string(&mut actual).unwrap();

    assert_eq!(
        "pub const PROFILE: Option<&\'static str> = Some(\"123\");\n",
        &actual
    );

    ::std::mem::drop(lock);
}

#[test]
fn profile_optional_missing() {
    let mut file = tempfile().unwrap();

    let lock = PROFILE.lock().unwrap();

    ::std::env::remove_var("PROFILE");

    BuildDetails::none()
        .include(BuildDetail::Profile)
        .write_to(&mut file)
        .unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

    let mut actual = String::new();
    file.read_to_string(&mut actual).unwrap();

    assert_eq!(
        "pub const PROFILE: Option<&\'static str> = None;\n",
        &actual
    );

    ::std::mem::drop(lock);
}
