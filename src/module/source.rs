use std::{path::PathBuf, str::FromStr};

use serde::{de::Error, Deserialize, Serialize};
use thiserror::Error;
use url::Url;

use crate::utils;

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("source must not be empty or whitespace")]
    MissingValue,
    #[error("source is not a valid url ({0})")]
    InvalidUrl(#[from] url::ParseError),
    #[error("source does not appear to be a git repository nor an absolute path")]
    Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Source {
    Git(Url),
    Path(PathBuf),
}

impl Source {
    pub fn with_git_shorthand(s: &str) -> Result<Self, url::ParseError> {
        let url = Url::parse(&format!("https://github.com/{s}.git"))?;
        Ok(Self::Git(url))
    }
}

impl FromStr for Source {
    type Err = SourceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if utils::is_empty_or_whitespace(s) {
            return Err(SourceError::MissingValue);
        }

        if utils::is_repo_shorthand(&s) {
            return Ok(Source::with_git_shorthand(s)?);
        }

        // catch absolute windows paths that would parse the partition as url scheme
        #[cfg(windows)]
        if utils::is_maybe_win_path(s) {
            return Ok(Self::Path(PathBuf::from(s)));
        }

        if let Ok(url) = Url::parse(s) {
            return Ok(Self::Git(url));
        }

        let p = PathBuf::from(s);
        if !p.starts_with("~") && !(p.is_absolute() || p.has_root()) {
            return Err(SourceError::Unknown);
        }

        Ok(Self::Path(PathBuf::from(s)))
    }
}

impl ToString for Source {
    fn to_string(&self) -> String {
        match self {
            Source::Git(url) => utils::to_repo_shorthand(&url).unwrap_or_else(|| url.to_string()),
            Source::Path(p) => p.display().to_string(),
        }
    }
}

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        Source::from_str(s).map_err(D::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use crate::make_de_test;

    use super::*;

    make_de_test!(
        de_source_git_shorthand,
        Source,
        "foo/bar",
        Source::Git("https://github.com/foo/bar.git".try_into().unwrap()),
    );

    make_de_test!(
        de_source_git,
        Source,
        "https://gitlab.com/foo/bar",
        Source::Git("https://gitlab.com/foo/bar".try_into().unwrap()),
    );

    make_de_test!(
        de_source_path_home,
        Source,
        "~/foo/bar",
        Source::Path("~/foo/bar".into()),
    );

    make_de_test!(
        de_source_win_path_abs,
        Source,
        "C:/foo/bar",
        Source::Path("C:\\foo\\bar".into()),
    );

    make_de_test!(
        de_source_win_path_server,
        Source,
        "\\\\server\\foo\\bar",
        Source::Path("\\\\server\\foo\\bar".into()),
    );

    make_de_test!(
        de_source_unix_path_abs,
        Source,
        "/foo/bar",
        Source::Path("/foo/bar".into()),
    );
}
