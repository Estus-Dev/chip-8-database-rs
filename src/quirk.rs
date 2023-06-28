use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Quirk {
    Shift,
    MemoryIncrementByX,
    MemoryLeaveIUnchanged,
    Wrap,
    Jump,
    #[serde(rename = "vblank")]
    VBlank,
    Logic,
}

#[cfg(feature = "quirks")]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuirkDetails {
    pub id: Quirk,
    pub name: String,
    pub description: Option<String>,
    pub default: bool,
    pub if_true: String,
    pub if_false: String,
}
