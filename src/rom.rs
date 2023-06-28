//! Defintions related to CHIP-8 ROMs.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    color::Colors,
    font::FontStyle,
    input::{Keymap, TouchInputMode},
    platform::Platform,
    quirk::Quirk,
    rotation::ScreenRotation,
};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rom {
    /// The file name of this ROM as it was observed when added to the database.
    #[serde(rename = "file")]
    pub file_name: Option<String>,

    /// The title that was extracted from the ROM file. Mostly superchip ROMs could start with a
    /// JUMP instruction over a string, allowing the interpreter to show the user the program title.
    pub embedded_title: Option<String>,

    /// If there is anything relevant to add about this specific ROM that isn't already in the
    /// program description, it can be added in this field.
    pub description: Option<String>,

    // TODO: This should be a date of some kind
    /// The date at which this particular ROM was first released, if it differs from the program's
    /// release date. Can be a year, a year and a month or a year, month and day.
    pub release: Option<String>,

    /// An ordered list of the platforms that this ROM can successfully run on with that platform's
    /// default quirks. The list is sorted by 'best' to 'worst' gameplay. See
    /// [PlatformDetails] for platform specifications. See also [quirky_platforms].
    pub platforms: Vec<Platform>,

    /// In some cases, a ROM was designed for a combination of a platform and quirks that is not
    /// considered an historic platform. For example because someone wrote the ROM for a badly
    /// written interpreter or made liberal use of quirks settings to achieve some goal. In those
    /// cases, this map of quirky platforms can be used to override the quirks of a base platform.
    /// If an interpreter supports quirky platforms, a quirky platform should be preferred over the
    /// regular [platforms] list. Otherwise, `quirky_platforms` should just be ignored.
    pub quirky_platforms: Option<HashMap<Platform, HashMap<Quirk, bool>>>,

    /// The list of authors who worked on developing this ROM.
    pub authors: Option<Vec<String>>,

    // TODO: Support real images here
    /// A list of file names of images that display this ROM.
    pub images: Option<Vec<String>>,

    // TODO: Use an appropriate URL type here
    /// A list of URLs that are relevant for this ROM, like a source code repository or additional
    /// materials.
    pub urls: Option<Vec<String>>,

    /// The preferred number of cycles per frame to run the ROM at. CHIP-8 runs at a framerate of
    /// 60Hz, so this tickrate times 60 is the desired 'CPU clockspeed' of the system.
    pub tickrate: Option<usize>,

    /// The start address from which the ROM should be run. This defaults to 0x200 (512), so this
    /// field should only be specified if the program needs to be run from a different address.
    pub start_address: Option<u16>,

    /// The screen orientation, in degrees rotated clockwise from the normal position.
    pub screen_rotation: Option<ScreenRotation>,

    /// A mapping of common keys to hexadecimal key values for the CHIP-8 keypad. For systems with
    /// dual keypads, add 0x10 (16) to the hexadecimal key value to indicate keypad number two.
    pub keys: Option<HashMap<Keymap, u8>>,

    /// The preferred touch input mode, values as used by Octo.
    pub touch_input_mode: Option<TouchInputMode>,

    /// The preferred style of font to use by the interpreter.
    pub font_style: Option<FontStyle>,

    /// An object with hexadecimal color properties (#RRGGBB).
    pub colors: Option<Colors>,
}
