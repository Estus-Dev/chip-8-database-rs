use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{origin::Origin, rom::Rom};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Program {
    pub title: String,

    pub description: Option<String>,

    // TODO: This should be a date of some kind
    pub release: Option<String>,

    pub origin: Option<Origin>,

    pub copyright: Option<String>,

    // TODO: See https://crates.io/crates/spdx
    pub license: Option<String>,

    pub authors: Option<Vec<String>>,

    // TODO: Support real images here
    pub images: Option<Vec<String>>,

    // TODO: Use an appropriate URL type here
    pub urls: Option<Vec<String>>,

    // TODO: Use an appropriate type for hashes
    pub roms: HashMap<String, Rom>,
}
