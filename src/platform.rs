use serde::{Deserialize, Serialize};

use crate::quirk::QuirkSet;

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Platform {
    #[serde(rename = "originalChip8")]
    OriginalChip8,

    #[serde(rename = "hybridVIP")]
    HybridVIP,

    #[serde(rename = "modernChip8")]
    ModernChip8,

    #[serde(rename = "chip8x")]
    Chip8X,

    #[serde(rename = "chip48")]
    Chip48,

    #[serde(rename = "superchip1")]
    Superchip1,

    #[serde(rename = "superchip")]
    Superchip,

    #[serde(rename = "megachip8")]
    MegaChip8,

    #[serde(rename = "xochip")]
    XOChip,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformDetails {
    pub id: Platform,
    pub name: String,
    pub description: Option<String>,
    pub release: Option<String>,
    pub authors: Option<Vec<String>>,
    pub urls: Option<Vec<String>>,
    pub copyright: Option<String>,
    pub license: Option<String>,
    // TODO: More appropriate type here, tuple or struct of usize?
    pub display_resolutions: Vec<String>,
    pub default_tickrate: usize,
    pub quirks: QuirkSet,
}
