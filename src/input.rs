//! Definitions related to user input.

use serde::{Deserialize, Serialize};

/// A list of all named input keys in the DB schema.
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

/// The preferred touch input mode, values as used by Octo.
#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TouchInputMode {
    /// No defined touch input mode exists for this ROM.
    #[default]
    None,

    /// Touch devices should allow players to swipe to control this ROM.
    Swipe,

    /// Touch devices should treat the entire screen as an invisible 4x4 grid of buttons.
    Seg16,

    /// Touch devices should treat the entire screen as a 4x4 grid of buttons.
    Seg16Fill,

    /// Touch devices should display a virtual D-Pad with A and B buttons.
    Gamepad,

    /// Touch devices should display a visible 4x4 grid of buttons separate from the game screen.
    VIP,
}
