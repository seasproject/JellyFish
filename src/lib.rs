use std::{fmt::Display, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JellyFishPackage {
    pub author: String,
    pub name: String,
    pub friendly_name: String,
    pub versions: Vec<PackageVersion>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackageVersion {
    pub name: String,
    pub version: String,
    pub required: Vec<PathBuf>,
    pub dependencies: Vec<String>, // A vec of URLS pointing to required packages. These packages MUST be in the JellyFish format (or any flavour)
    pub flavour: String,
}

impl Display for JellyFishPackage {
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
