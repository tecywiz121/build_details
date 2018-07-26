extern crate build_details;

fn main() {
    build_details::BuildDetails::all()
        .generate("build_details.rs")
        .unwrap();
}
