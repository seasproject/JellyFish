use std::{fmt::Display, path::PathBuf};

use guid_create::GUID;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Package {
    pub author: String,
    pub name: String,
    pub friendly_name: String,
    pub versions: Vec<PackageVersion>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackageVersion {
    pub id: GUID,
    pub name: String,
    pub version: String, // TODO: make this a proper version struct
    pub required: Vec<PathBuf>,
    pub dependencies: Vec<GUID>,
    pub flavor: String,
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} v{}",
            self.friendly_name,
            self.versions
                .last()
                .map(|x| -> String { x.version.clone() })
                .unwrap_or("0.0.0".to_string())
        )
    }
}
