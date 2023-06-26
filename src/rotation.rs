use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Default, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(usize)]
pub enum ScreenRotation {
    #[default]
    Landscape = 0,
    Portrait = 90,
    LandscapeFlipped = 180,
    PortraitFlipped = 270,
}
