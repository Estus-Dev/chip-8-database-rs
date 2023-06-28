//! Defintions related to CHIP-8 screen rotations.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// The screen orientation, in degrees rotated clockwise from the normal position.
#[derive(Clone, Debug, Default, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(usize)]
pub enum ScreenRotation {
    #[default]
    Landscape = 0,
    Portrait = 90,
    LandscapeFlipped = 180,
    PortraitFlipped = 270,
}
