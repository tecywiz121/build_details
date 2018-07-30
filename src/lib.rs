// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.

//! # Build Details
//!
//! `build_details` is a code generation helper that provides build information
//! at runtime.
//!
//! There are two steps to adding `build_details` to a crate:
//!
//!   * Adding/modifying `build.rs`; and
//!   * Including the generated file.
//!
//! ## Invoking Build Details
//!
//! Invoking `build_details` is as simple as adding the following snippet to
//! `build.rs`:
//!
//! ```no_run
//! extern crate build_details;
//!
//! fn main() {
//!     build_details::BuildDetails::default()
//!         .generate("build_details.rs")
//!         .unwrap();
//! }
//! ```
//!
//! ## Including Generated File
//!
//! In `src/lib.rs`:
//!
//! ```no_compile
//! pub mod build_details {
//!     include!(concat!(env!("OUT_DIR"), "/build_details.rs"));
//! }
//! ```
//!
//! ## A note on [`BuildDetail::Cfg`]
//!
//! Using [`BuildDetail::Cfg`] requires a runtime dependency on `phf`.
//!
//! In `Cargo.toml`, add:
//!
//! ```toml
//! [dependencies]
//! phf = "0.7"
//! ```
//!
//! In `src/lib.rs` or `src/main.rs`:
//!
//! ```no_compile
//! extern crate phf;
//! ```
#![deny(
    missing_debug_implementations, missing_docs, trivial_casts, trivial_numeric_casts,
    unused_extern_crates, unused_import_braces, unused_qualifications
)]

#[macro_use]
extern crate maplit;
extern crate phf_codegen;

pub mod error;

use error::*;

use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Code generator for build details. See the crate documentation for an example.
#[derive(Debug, Clone)]
pub struct BuildDetails {
    optional: HashSet<BuildDetail>,
    required: HashSet<BuildDetail>,
}

impl Default for BuildDetails {
    fn default() -> Self {
        Self {
            optional: hashset![
                BuildDetail::Version,
                BuildDetail::Profile,
                BuildDetail::RustFlags,
            ],
            required: HashSet::new(),
        }
    }
}

impl BuildDetails {
    /// Construct a [`BuildDetails`] instance with all available details marked
    /// as optional.
    pub fn all() -> Self {
        Self {
            optional: hashset![
                BuildDetail::Timestamp,
                BuildDetail::Version,
                BuildDetail::Profile,
                BuildDetail::RustFlags,
                BuildDetail::Name,
                BuildDetail::Authors,
                BuildDetail::Description,
                BuildDetail::Homepage,
                BuildDetail::Cfg,
                BuildDetail::Features,
            ],
            required: HashSet::new(),
        }
    }

    /// Construct a [`BuildDetails`] instance with all available details marked
    /// as required.
    ///
    /// This method isn't particularly useful by itself, and will probably need
    /// customization with [`BuildDetails::include`] and [`BuildDetails::exclude`].
    ///
    /// It is impossible to use this method and not break API compatibility if
    /// new [`BuildDetail`] variants are added.
    #[doc(hidden)]
    pub fn require_all() -> Self {
        let mut x = Self::all();
        ::std::mem::swap(&mut x.optional, &mut x.required);
        x
    }

    /// Construct a [`BuildDetails`] instance with no included details.
    ///
    /// This method isn't particularly useful by itself, and will probably need
    /// customization with [`BuildDetails::include`] and [`BuildDetails::exclude`].
    pub fn none() -> Self {
        Self {
            optional: HashSet::new(),
            required: HashSet::new(),
        }
    }

    /// Include a [`BuildDetail`], and mark it as required.
    ///
    /// If a detail is marked as required and isn't available at build time, the
    /// build will fail.
    pub fn require(&mut self, detail: BuildDetail) -> &mut Self {
        self.optional.remove(&detail);
        self.required.insert(detail);
        self
    }

    /// Include a [`BuildDetail`], and mark it as optional.
    ///
    /// If a detail is marked as optional and isn't available at build time, the
    /// generated value will be `None`.
    pub fn include(&mut self, detail: BuildDetail) -> &mut Self {
        self.required.remove(&detail);
        self.optional.insert(detail);
        self
    }

    /// Exclude a [`BuildDetail`]. It will not show up in the generated output.
    pub fn exclude(&mut self, detail: BuildDetail) -> &mut Self {
        self.required.remove(&detail);
        self.optional.remove(&detail);
        self
    }

    /// Creates a file called `path` in the build's `OUT_DIR` directory. See
    /// the crate documentation for an example.
    pub fn generate<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let out_dir = match env::var_os("OUT_DIR") {
            Some(x) => x,
            None => return Err(Error::MissingEnv("OUT_DIR")),
        };

        let mut out_path = PathBuf::from(out_dir);
        out_path.push(path);

        let mut out_file = File::create(out_path)?;

        self.write_to(&mut out_file)
    }

    /// Writes the generated code to a [`::std::io::Write'] instead of to a file.
    pub fn write_to(&self, out_file: &mut Write) -> Result<()> {
        for detail in &self.optional {
            writeln!(out_file, "{}", detail.render_option()?)?;
        }

        for detail in &self.required {
            writeln!(out_file, "{}", detail.render()?)?;
        }

        Ok(())
    }
}

/// List of build details that can be included in the generated code.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BuildDetail {
    /// Number of seconds since [`::std::time::UNIX_EPOCH`]
    Timestamp,

    /// Equivalent to the `CARGO_PKG_VERSION` environment variable.
    Version,

    /// Equivalent to `PROFILE` in environment variables passed to `build.rs'.
    ///
    /// Should usually be `"debug"` or `"release"`.
    Profile,

    /// Equivalent to the `RUSTFLAGS` environment variable.
    ///
    /// Note that this isn't _all_ of the flags passed to `rustc`, but instead
    /// it is only the custom extra flags.
    RustFlags,

    /// Equivalent to the `CARGO_PKG_NAME` environment variable.
    Name,

    /// Equivalent to the `CARGO_PKG_AUTHORS` environment variable.
    Authors,

    /// Equivalent to the `CARGO_PKG_DESCRIPTION` environment variable.
    Description,

    /// Equivalent to the `CARGO_PKG_HOMEPAGE` environment variable.
    Homepage,

    /// Equivalent to the `OPT_LEVEL` environment variable in `build.rs`.
    OptLevel,

    /// Equivalent to the `CARGO_CFG_*` environment variables in `build.rs`.
    Cfg,

    /// Equivalent to the `CARGO_FEATURE_*` environment variables in `build.rs`.
    Features,

    #[doc(hidden)]
    __Nonexhaustive,
}

impl BuildDetail {
    fn into_render(self) -> Box<Render> {
        use self::BuildDetail::*;

        match self {
            Timestamp => Box::from(self::Timestamp::new()),

            Version => Box::from(Env::new("VERSION", "CARGO_PKG_VERSION")),
            Name => Box::from(Env::new("NAME", "CARGO_PKG_NAME")),
            Authors => Box::from(Env::new("AUTHORS", "CARGO_PKG_AUTHORS")),
            Description => Box::from(Env::new("DESCRIPTION", "CARGO_PKG_DESCRIPTION")),
            Homepage => Box::from(Env::new("HOMEPAGE", "CARGO_PKG_HOMEPAGE")),
            RustFlags => Box::from(Env::new("RUST_FLAGS", "RUSTFLAGS")),

            Profile => Box::from(BuildEnv::new("PROFILE", "PROFILE")),
            OptLevel => Box::from(BuildEnv::new("OPT_LEVEL", "OPT_LEVEL")),

            Cfg => Box::from(BuildEnvMap::new("CFG", "CARGO_CFG_")),
            Features => Box::from(BuildEnvList::new("FEATURES", "CARGO_FEATURE_")),

            __Nonexhaustive => unreachable!(),
        }
    }
}

impl Render for BuildDetail {
    fn render_option(&self) -> Result<String> {
        self.into_render().render_option()
    }

    fn render(&self) -> Result<String> {
        self.into_render().render()
    }
}

struct Detail<T>
where
    T: Render,
{
    name: &'static str,
    value_type: &'static str,
    value: T,
}

impl<T> Render for Detail<T>
where
    T: Render,
{
    fn render_option(&self) -> Result<String> {
        let value = self.value.render_option()?;

        Ok(format!(
            "pub const {}: Option<{}> = {};",
            self.name, self.value_type, value
        ))
    }

    fn render(&self) -> Result<String> {
        let value = match self.value.render() {
            Ok(x) => x,
            Err(Error::Missing) => {
                return Err(Error::MissingDetail(self.name.to_owned()));
            }
            e => return e,
        };

        Ok(format!(
            "pub const {}: {} = {};",
            self.name, self.value_type, value
        ))
    }
}

trait Render {
    fn render_option(&self) -> Result<String>;
    fn render(&self) -> Result<String>;
}

impl<T> Render for Option<T>
where
    T: fmt::Display,
{
    fn render_option(&self) -> Result<String> {
        match self {
            Some(x) => Ok(format!("Some({})", x)),
            None => Ok(format!("None")),
        }
    }

    fn render(&self) -> Result<String> {
        match self {
            Some(x) => Ok(format!("{}", x)),
            None => Err(Error::Missing),
        }
    }
}

struct Timestamp;

impl Timestamp {
    pub fn new() -> Detail<Option<u64>> {
        // TODO: Touch build.rs to trigger a rebuild every time

        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .as_ref()
            .map(Duration::as_secs)
            .ok();

        Detail {
            name: "TIMESTAMP",
            value_type: "u64",
            value: secs,
        }
    }
}

struct Env(&'static str);

impl Render for Env {
    fn render_option(&self) -> Result<String> {
        Ok(format!("option_env!(\"{}\")", self.0))
    }

    fn render(&self) -> Result<String> {
        Ok(format!("env!(\"{}\")", self.0))
    }
}

impl Env {
    pub fn new(name: &'static str, env: &'static str) -> Detail<Env> {
        Detail {
            name,
            value_type: "&'static str",
            value: Env(env),
        }
    }
}

struct BuildEnv(Option<String>);

impl Render for BuildEnv {
    fn render_option(&self) -> Result<String> {
        match self.0 {
            Some(ref x) => Ok(format!("Some({:?})", x)),
            None => Ok("None".to_owned()),
        }
    }

    fn render(&self) -> Result<String> {
        match self.0 {
            Some(ref x) => Ok(format!("{:?}", x)),
            None => Err(Error::Missing),
        }
    }
}

impl BuildEnv {
    pub fn new(name: &'static str, env: &'static str) -> Detail<Self> {
        let env = env::var(env).ok();

        Detail {
            name,
            value_type: "&'static str",
            value: BuildEnv(env),
        }
    }
}

fn find_matching_vars(prefix: &'static str) -> HashMap<String, String> {
    env::vars()
        .filter_map(|(k, v)| {
            if k.starts_with(prefix) {
                let k = k[prefix.len()..].to_owned();
                Some((k, v))
            } else {
                None
            }
        })
        .collect()
}

struct BuildEnvList(Vec<String>);

impl BuildEnvList {
    pub fn new(name: &'static str, prefix: &'static str) -> Detail<Self> {
        Detail {
            name,
            value_type: "&'static [&'static str]",
            value: BuildEnvList(
                find_matching_vars(prefix)
                    .into_iter()
                    .map(|(k, _)| k)
                    .collect(),
            ),
        }
    }
}

impl Render for BuildEnvList {
    fn render_option(&self) -> Result<String> {
        Ok(format!("Some({})", self.render()?))
    }

    fn render(&self) -> Result<String> {
        use std::fmt::Write;

        let mut txt = String::from("&[\n");

        for item in &self.0 {
            write!(txt, "    {:?},\n", item)?;
        }

        write!(txt, "]")?;

        Ok(txt)
    }
}

struct BuildEnvMap(HashMap<String, String>);

impl BuildEnvMap {
    pub fn new(name: &'static str, prefix: &'static str) -> Detail<Self> {
        Detail {
            name,
            value_type: "::phf::Map<&'static str, &'static str>",
            value: BuildEnvMap(find_matching_vars(prefix)),
        }
    }
}

impl Render for BuildEnvMap {
    fn render_option(&self) -> Result<String> {
        Ok(format!("Some({})", self.render()?))
    }

    fn render(&self) -> Result<String> {
        let mut txt = vec![];

        let mut map = phf_codegen::Map::<&str>::new();

        for (k, v) in &self.0 {
            map.entry(k, &format!("{:?}", v));
        }

        map.build(&mut txt)?;

        Ok(String::from_utf8(txt).unwrap())
    }
}
