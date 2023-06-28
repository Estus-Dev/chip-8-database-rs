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
    #[serde(rename = "file")]
    pub file_name: Option<String>,

    pub embedded_title: Option<String>,

    pub description: Option<String>,

    // TODO: This should be a date of some kind
    pub release: Option<String>,

    pub platforms: Vec<Platform>,

    pub quirky_platforms: Option<HashMap<Platform, HashMap<Quirk, bool>>>,

    pub authors: Option<Vec<String>>,

    // TODO: Support real images here
    pub images: Option<Vec<String>>,

    // TODO: Use an appropriate URL type here
    pub urls: Option<Vec<String>>,

    pub tickrate: Option<usize>,

    pub start_address: Option<u16>,

    pub screen_rotation: Option<ScreenRotation>,

    /// A mapping of common keys to hexadecimal key values for the CHIP-8 keypad. For systems with
    /// dual keypads, add 0x10 (16) to the hexadecimal key value to indicate keypad number two.
    pub keys: Option<HashMap<Keymap, u8>>,

    pub touch_input_mode: Option<TouchInputMode>,

    pub font_style: Option<FontStyle>,

    pub colors: Option<Colors>,
}
