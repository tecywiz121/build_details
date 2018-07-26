#[macro_use]
extern crate maplit;

pub mod error;

use error::*;

use std::collections::HashSet;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct BuildDetails {
    optional: HashSet<BuildDetail>,
    required: HashSet<BuildDetail>,
}

impl Default for BuildDetails {
    fn default() -> Self {
        Self {
            optional: hashset![
                BuildDetail::Timestamp,
                BuildDetail::Version,
                BuildDetail::Profile,
                BuildDetail::RustFlags,
            ],
            required: HashSet::new(),
        }
    }
}

impl BuildDetails {
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
            ],
            required: HashSet::new(),
        }
    }

    pub fn require_all() -> Self {
        let mut x = Self::all();
        ::std::mem::swap(&mut x.optional, &mut x.required);
        x
    }

    pub fn none() -> Self {
        Self {
            optional: HashSet::new(),
            required: HashSet::new(),
        }
    }

    pub fn require(&mut self, detail: BuildDetail) -> &mut Self {
        self.optional.remove(&detail);
        self.required.insert(detail);
        self
    }

    pub fn include(&mut self, detail: BuildDetail) -> &mut Self {
        self.required.remove(&detail);
        self.optional.insert(detail);
        self
    }

    pub fn exclude(&mut self, detail: BuildDetail) -> &mut Self {
        self.required.remove(&detail);
        self.optional.remove(&detail);
        self
    }

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

    pub fn write_to(&self, out_file: &mut File) -> Result<()> {
        for detail in &self.optional {
            writeln!(out_file, "{}", detail.render_option()?)?;
        }

        for detail in &self.required {
            writeln!(out_file, "{}", detail.render()?)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum BuildDetail {
    Timestamp,
    Version,
    Profile,
    RustFlags,
    Name,
    Authors,
    Description,
    Homepage,
    OptLevel,
    /*
    VersionMajor,
    VersionMinor,
    VersionPatch,
    VersionPre,
    */
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
