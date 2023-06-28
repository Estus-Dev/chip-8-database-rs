use serde::{Deserialize, Serialize};

/// An object with hexadecimal color properties (#RRGGBB).
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Colors {
    /// Colors for all the states a pixel can be in. The length of this array depends on the
    /// platform and the number of colors used by the ROM. The colors are sorted by the binary
    /// values that represent the pixel's state. So for a single plane, the first color is the 'off'
    ///  or background color (`0`) and the second color is the 'on' or foreground color (`1`). For
    /// programs with two planes the order of colors is: both 'off' (`00`), plane 2 'off' and plane
    /// 1 'on' (`01`), plane 2 'on' and plane 1 'off' (`10`) and finally both 'on' (`11`). This
    /// extends to more planes for the few programs that support them: adding another plane adds
    /// another most significant bit.
    pub pixels: Option<Vec<String>>,

    /// A color that can be used to show a visual 'beep', to accompany the sound timer.
    pub buzzer: Option<String>,

    /// The color of the visual 'beep' when there is no sound (the sound timer is zero).
    pub silence: Option<String>,
}
