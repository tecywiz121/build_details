// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

extern crate phf;

pub mod build_details {
    include!(concat!(env!("OUT_DIR"), "/build_details.rs"));
}

pub mod required_build_details {
    include!(concat!(env!("OUT_DIR"), "/required_build_details.rs"));
}
