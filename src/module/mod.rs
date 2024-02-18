pub mod entry;
pub mod source;

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{serde_utils, utils};

use self::{entry::Entry, source::Source};

#[derive(Debug, Error)]
pub enum ModuleError {
    #[error("module name must not be empty or whitespace")]
    MissingName,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Module {
    /// The name of the module.
    ///
    /// This serves as a unique identifier for the module, used in profiles and various commands.
    /// It does not affect destination paths or deployment logic.
    #[serde(deserialize_with = "serde_utils::de_nonempty_string")]
    name: String,
    /// Source repository or directory of the module.
    source: Source,
    /// Base output path of module entries.
    ///
    /// If not specified, the user's home directory is used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    base: Option<PathBuf>,
    /// Defines a listing of sources and where they map to when deployed.
    ///
    /// If no entries are specified, all source files are mapped relative to the module structure.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    entries: Vec<Entry>,
    /// A list of glob patterns to exclude from matched files in `entries`.
    ///
    /// Useful for ignoring common files like README, LICENSE, etc.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    exclude: Vec<String>,
}

impl Module {
    pub fn new<S: Into<Source>>(name: &str, source: S) -> Result<Self, ModuleError> {
        if utils::is_empty_or_whitespace(name) {
            return Err(ModuleError::MissingName);
        }
        Ok(Self {
            name: name.to_owned(),
            source: source.into(),
            base: Default::default(),
            entries: Default::default(),
            exclude: Default::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! make_de_test {
        ($fn:ident, $t:ty, $in:expr, $expected:expr $(,)?) => {
            #[test]
            fn $fn() {
                let s = serde_yaml::from_str::<$t>($in).unwrap();
                let expected = $expected;
                assert_eq!(s, expected);
            }
        };
    }

    make_de_test!(
        de_module_minimal,
        Module,
        "name: foo\nsource: foo/bar",
        Module {
            name: "foo".into(),
            source: Source::with_git_shorthand("foo/bar").unwrap(),
            base: None,
            entries: vec![],
            exclude: vec![]
        },
    );

    make_de_test!(
        de_module,
        Module,
        r#"name: foo
source: foo/bar
base: "~"
entries:
  - type: glob
    value: "**/*"
    link: true
exclude: [".git/", "**/*.md", "LICENSE"]"#,
        Module {
            name: "foo".into(),
            source: Source::with_git_shorthand("foo/bar").unwrap(),
            base: Some("~".into()),
            entries: vec![Entry::default()],
            exclude: vec![".git/".into(), "**/*.md".into(), "LICENSE".into()]
        },
    );
}
