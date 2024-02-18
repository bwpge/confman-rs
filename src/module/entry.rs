use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::serde_utils;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Kind {
    File,
    #[serde(alias = "dir")]
    Directory,
    Glob,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    /// The type of entry this is.
    #[serde(rename = "type")]
    pub kind: Kind,
    /// Value to match for this entry, based on `kind`.
    #[serde(
        alias = "path",
        alias = "glob",
        skip_serializing_if = "String::is_empty"
    )]
    pub value: String,
    /// Whether or not this entry should be linked.
    #[serde(
        default = "serde_utils::default_true",
        skip_serializing_if = "serde_utils::is_true"
    )]
    pub link: bool,
    /// Output directory for this entry, relative to the base path.
    #[serde(default, rename = "maps_to", skip_serializing_if = "Option::is_none")]
    pub map_dir: Option<PathBuf>,
    /// Whether or not to make matched files flat in the output directory, regardless of nested
    /// structure.
    ///
    /// Ignored for `directory` entries.
    #[serde(default, skip_serializing_if = "serde_utils::is_false")]
    pub flatten: bool,
    /// Operating systems that this entry applies to.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub os: Vec<String>,
    /// A map of matched files to rename.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub rename: HashMap<PathBuf, PathBuf>,
}

impl Entry {
    pub fn new(s: &str, kind: Kind) -> Self {
        Self {
            kind,
            value: s.into(),
            link: true,
            map_dir: Default::default(),
            flatten: false,
            os: Default::default(),
            rename: Default::default(),
        }
    }
}

impl Default for Entry {
    fn default() -> Self {
        Self::new("**/*", Kind::Glob)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::make_de_test;

    make_de_test!(
        de_file,
        Entry,
        "type: file\nvalue: foo/bar.baz",
        Entry::new("foo/bar.baz", Kind::File)
    );

    make_de_test!(
        de_dir,
        Entry,
        "type: directory\nvalue: foo/bar",
        Entry::new("foo/bar", Kind::Directory)
    );

    make_de_test!(
        de_dir_alias,
        Entry,
        "type: dir\nvalue: foo/bar",
        Entry::new("foo/bar", Kind::Directory)
    );

    make_de_test!(
        de_glob,
        Entry,
        "type: glob\nvalue: '**/*'",
        Entry::new("**/*", Kind::Glob)
    );
}
