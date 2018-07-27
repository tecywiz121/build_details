// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

extern crate build_details;

fn main() {
    build_details::BuildDetails::all()
        .generate("build_details.rs")
        .unwrap();

    build_details::BuildDetails::require_all()
        .exclude(build_details::BuildDetail::RustFlags)
        .generate("required_build_details.rs")
        .unwrap();
}
