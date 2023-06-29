//! Defintions related to CHIP-8 quirks.

use serde::{Deserialize, Serialize};

/// An ID for each quirk, by which to reference it in a [Program] or [Platform].
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

/// A detailed breakdown of the meaning of a [Quirk].
#[cfg(feature = "extra-data")]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuirkDetails {
    /// An ID for each quirk, by which to reference it in a [Program] or [Platform].
    pub id: Quirk,

    /// The name of this quirk.
    pub name: String,

    /// A description of the quirk and why it exists.
    pub description: Option<String>,

    /// Whether or not this quirk should be enabled by default.
    pub default: bool,

    /// A description of CHIP-8 behavior when this quirk is enabled.
    pub if_true: String,

    /// A description of CHIP-8 behavior when this quirk is not enabled.
    pub if_false: String,
}
