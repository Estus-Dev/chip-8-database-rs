//! Definitions related to font data.

use serde::{Deserialize, Serialize};

/// The preferred style of font to use by the interpreter.
#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FontStyle {
    /// The font used on the original [COSMAC VIP](https://en.wikipedia.org/wiki/COSMAC_VIP)
    /// hobbyist computer.
    #[default]
    VIP,

    /// The font used by the [Octo IDE](http://johnearnest.github.io/Octo/)'s built-in emulator.
    Octo,

    /// The font used by the SCHIP interpreter for the HP-48 calculator.
    SCHIP,

    /// The font used on the DREAM 6800 hobbyist computer.
    Dream6800,

    /// The font used on the ETI 660 hobbyist computer.
    ETI660,

    /// The font used by the Fish N Chips emulator.
    Fish,

    /// Unknown, possibly a font used in ROMs by AKouZ1?
    Akouz1,
}
