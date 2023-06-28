use serde::{Deserialize, Serialize};

/// The origin of this program; was it created at some event, for a game jam, or published in a
/// magazine somewhere?
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OriginType {
    GameJam,
    Event,
    Magazine,
    Manual,
}

/// The origin of this program; was it created at some event, for a game jam, or published in a
/// magazine somewhere?
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Origin {
    /// The origin of this program; was it created at some event, for a game jam, or published in a
    /// magazine somewhere?
    #[serde(rename = "type")]
    pub origin_type: Option<OriginType>,

    /// A freeform reference to the origin of this program.
    pub reference: Option<String>,
}
