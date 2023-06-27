use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Platform {
    #[serde(rename = "originalChip8")]
    OriginalChip8,

    #[serde(rename = "hybridVIP")]
    HybridVIP,

    #[serde(rename = "modernChip8")]
    ModernChip8,

    #[serde(rename = "chip8X")]
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

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QuirkSet {
    pub shift: bool,
    pub memory_increment_by_x: bool,
    pub memory_leave_i_unchanged: bool,
    pub wrap: bool,
    pub jump: bool,
    pub vblank: bool,
    pub logic: bool,
}
