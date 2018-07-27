extern crate phf;

pub mod build_details {
    include!(concat!(env!("OUT_DIR"), "/build_details.rs"));
}

pub mod required_build_details {
    include!(concat!(env!("OUT_DIR"), "/required_build_details.rs"));
}
