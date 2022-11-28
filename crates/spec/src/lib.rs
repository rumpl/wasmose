use std::{collections::HashMap, fs, path::Path};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Spec {
    pub modules: HashMap<String, Module>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Module {
    pub module: String,
    pub dirs: Option<Vec<Dir>>,
    pub environment: Option<HashMap<String, String>>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Dir {
    pub source: String,
    pub target: String,
}

pub struct SpecLoader {}

impl SpecLoader {
    pub fn from_file<T: AsRef<Path>>(file: T) -> Result<Spec> {
        let d = fs::read_to_string(file)?;
        Self::deserialize(d)
    }

    pub fn deserialize(s: String) -> Result<Spec> {
        let spec = serde_yaml::from_str(&s)?;
        Ok(spec)
    }

    pub fn deserialize_module(s: String) -> Result<Module> {
        let spec = serde_yaml::from_str(&s)?;
        Ok(spec)
    }
}

pub struct RunOptions {
    pub runtime: String,
}

impl Spec {}
