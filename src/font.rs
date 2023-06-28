use serde::{Deserialize, Serialize};

/// The preferred style of font to use by the interpreter.
#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FontStyle {
    #[default]
    VIP,
    Octo,
    SCHIP,
    Dream6800,
    ETI660,
    Fish,
    Akouz1,
}
