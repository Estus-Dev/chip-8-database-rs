use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Quirk {
    pub shift: bool,
    pub memory_increment_by_x: bool,
    pub memory_leave_i_unchanged: bool,
    pub wrap: bool,
    pub jump: bool,
    pub vblank: bool,
    pub logic: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum QuirkType {
    Shift,
    MemoryIncrementByX,
    MemoryLeaveIUnchanged,
    Wrap,
    Jump,
    #[serde(rename = "vblank")]
    VBlank,
    Logic,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuirkDetails {
    pub id: QuirkType,
    pub name: String,
    pub description: Option<String>,
    pub default: bool,
    pub if_true: String,
    pub if_false: String,
}
