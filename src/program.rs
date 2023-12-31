//! Definitions related to CHIP-8 programs.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{origin::Origin, rom::Rom};

/// A program written for the CHIP-8 or a derivative platform. Can have multiple versions under the
/// ROMs section.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Program {
    /// The title of the program, preferably in the way the original author intended it.
    pub title: String,

    /// A description of the program, preferably in the way the original author described or
    /// published it.
    pub description: Option<String>,

    /// The date at which the program was first released in ISO 8601 date format. Can be a year, a
    /// year and a month or a year, month and day.
    ///
    /// N.B. When even the specific year is not known, one or more digits at the end may be replaced
    /// with `x`, as in `199x` or `19xx`. If sorting is required, sorting as strings should do the
    /// job.
    pub release: Option<String>,

    /// The origin of this program; was it created at some event, for a game jam, or published in a
    /// magazine somewhere?
    pub origin: Option<Origin>,

    /// The copyright situation of this program. May be free form text. If a specific license is
    /// known, please use the `license` field instead.
    pub copyright: Option<String>,

    /// The license(s) applicable to this program. Must be an SPDX license expression
    /// (see https://crates.io/crates/spdx, https://spdx.org/licenses/, and
    /// https://spdx.github.io/spdx-spec/v2.3/SPDX-license-expressions/).
    pub license: Option<String>,

    /// The list of authors who worked on developing this program.
    pub authors: Option<Vec<String>>,

    /// A list of file names of images that display this program.
    pub images: Option<Vec<String>>,

    /// A list of URLs that are relevant for this program, like a source code repository or
    /// additional materials.
    pub urls: Option<Vec<String>>,

    /// A map of SHA1 hashes to ROM files that relate to this program.
    pub roms: HashMap<String, Rom>,
}
