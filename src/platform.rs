use serde::{Deserialize, Serialize};

/// An ID for this platform, by which to reference it in programs.json.
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

#[cfg(feature = "platforms")]
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformDetails {
    /// An ID for this platform, by which to reference it in programs.json.
    pub id: Platform,

    /// The full name of the platform, by which it is most commonly known.
    pub name: String,

    /// A generic description of the platform.
    pub description: Option<String>,

    /// The date at which the platform was first released in ISO 8601 date format. Can be a year, a
    /// year and a month or a year, month and day.
    pub release: Option<String>,

    /// The list of authors who worked on developing this platform.
    pub authors: Option<Vec<String>>,

    /// A list of URLs that are relevant for this platform, like a systems specification or
    /// additional materials.
    pub urls: Option<Vec<String>>,

    /// The copyright situation of this platform. May be free form text. If a specific license is
    /// known, please use the `license` field instead.
    pub copyright: Option<String>,

    /// The license(s) applicable to this platform. Must be an SPDX license expression
    /// (see https://spdx.github.io/spdx-spec/v2.3/SPDX-license-expressions/ and
    /// https://spdx.org/licenses/).
    pub license: Option<String>,

    // TODO: More appropriate type here, tuple or struct of usize?
    /// All the display resolutions that this platform supports.
    pub display_resolutions: Vec<String>,

    /// The preferred number of cycles per frame to run the interpreter at. It's the default because
    /// ROMs can overwrite this value. CHIP-8 runs at a framerate of 60Hz, so this tickrate times 60
    /// is the desired 'CPU clockspeed' of the system.
    pub default_tickrate: usize,

    /// The quirk settings as they are known for this platform.
    pub quirks: std::collections::HashMap<crate::quirk::Quirk, bool>,
}
