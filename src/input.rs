use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Keymap {
    #[serde(rename = "up")]
    P1Up,

    #[serde(rename = "down")]
    P1Down,

    #[serde(rename = "left")]
    P1Left,

    #[serde(rename = "right")]
    P1Right,

    #[serde(rename = "a")]
    P1A,

    #[serde(rename = "b")]
    P1B,

    #[serde(rename = "player2Up")]
    P2Up,

    #[serde(rename = "player2Down")]
    P2Down,

    #[serde(rename = "player2Left")]
    P2Left,

    #[serde(rename = "player2Right")]
    P2Right,

    #[serde(rename = "player2A")]
    P2A,

    #[serde(rename = "player2B")]
    P2B,
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TouchInputMode {
    #[default]
    None,
    Swipe,
    Seg16,
    Seg16Fill,
    Gamepad,
    VIP,
}
