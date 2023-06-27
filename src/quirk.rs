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
