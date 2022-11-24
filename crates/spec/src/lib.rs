use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Spec {
    pub modules: HashMap<String, Module>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub dirs: Option<Vec<Dir>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Dir {
    pub source: String,
    pub target: String,
}

impl Spec {
    pub fn from_file<T: AsRef<Path>>(file: T) -> Result<Self> {
        let d = fs::read_to_string(file)?;
        Self::deserialize(d)
    }

    pub fn deserialize(s: String) -> Result<Self> {
        let spec = serde_yaml::from_str(&s)?;
        Ok(spec)
    }
}
