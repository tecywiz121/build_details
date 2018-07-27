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
