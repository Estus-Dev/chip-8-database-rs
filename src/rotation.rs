//! Defintions related to CHIP-8 screen rotations.

use serde_repr::{Deserialize_repr, Serialize_repr};

/// The screen orientation, in degrees rotated clockwise from the normal position.
#[derive(Clone, Debug, Default, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(usize)]
pub enum ScreenRotation {
    /// A landscape view - the default orientation.
    #[default]
    Landscape = 0,

    /// A portrait view - rotated 90 degrees clockwise from the default.
    Portrait = 90,

    /// A landscape view - rotated 180 degrees clockwise from the default.
    LandscapeFlipped = 180,

    /// A portrait view - rotated 270 degrees clockwise from the default.
    PortraitFlipped = 270,
}
