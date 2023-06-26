use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OriginType {
    GameJam,
    Event,
    Magazine,
    Manual,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Origin {
    #[serde(rename = "type")]
    pub origin_type: Option<OriginType>,

    pub reference: Option<String>,
}
