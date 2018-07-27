Build Details
=============

## Introduction

`build_details` is a code generation helper that provides build information
at runtime.

There are two steps to adding `build_details` to a crate:

 * Adding/modifying `build.rs`; and
 * Including the generated file.

## Getting Started

### Invoking Build Details

Invoking `build_details` is as simple as adding the following snippet to
`build.rs`:

```rust
extern crate build_details;

fn main() {
    build_details::BuildDetails::default()
        .generate("build_details.rs")
        .unwrap();
}
```

### Including Generated File

In `src/lib.rs`:

```rust
pub mod build_details {
    include!(concat!(env!("OUT_DIR"), "/build_details.rs"));
}
```

### A note on `BuildDetail::Cfg`

Using `BuildDetail::Cfg` requires a runtime dependency on `phf`.

In `Cargo.toml`, add:

```toml
[dependencies]
phf = "0.7"
```

In `src/lib.rs` or `src/main.rs`:

```rust
extern crate phf;
```

## Limitations

 * Build timestamp isn't regenerated every build. [Issue #1][i1].

[i1]: https://github.com/tecywiz121/build_details/issues/1

## License

Licensed under the [Mozilla Public License, Version 2.0](LICENSE.md).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the Covered Software by you, as defined in the Mozilla Public
License, shall be licensed as above, without any additional terms or conditions.
