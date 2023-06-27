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

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
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

impl QuirkSet {
    pub fn get(&self, quirk: Quirk) -> bool {
        use Quirk::*;

        match quirk {
            Shift => self.shift,
            MemoryIncrementByX => self.memory_increment_by_x,
            MemoryLeaveIUnchanged => self.memory_leave_i_unchanged,
            Wrap => self.wrap,
            Jump => self.jump,
            VBlank => self.vblank,
            Logic => self.logic,
        }
    }

    pub fn set(&mut self, quirk: Quirk, value: bool) {
        use Quirk::*;

        match quirk {
            Shift => self.shift = value,
            MemoryIncrementByX => self.memory_increment_by_x = value,
            MemoryLeaveIUnchanged => self.memory_leave_i_unchanged = value,
            Wrap => self.wrap = value,
            Jump => self.jump = value,
            VBlank => self.vblank = value,
            Logic => self.logic = value,
        }
    }
}
