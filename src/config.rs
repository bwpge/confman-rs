use std::path::PathBuf;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::Module;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkMode {
    #[default]
    Always,
    Prefer,
    Never,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    link_mode: LinkMode,
    #[serde(default)]
    include: Vec<String>,
    #[serde(default)]
    modules: Vec<Module>,
}

impl Config {
    pub fn load(path: Option<&PathBuf>) -> Result<Self> {
        if let Some(p) = path {
            let s = std::fs::read_to_string(p)?;
            return Ok(serde_yaml::from_str(&s)?);
        }

        // TODO: try to find config file

        bail!("failed to locate config file")
    }
}
