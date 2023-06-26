use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sha1::{Digest, Sha1};
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct Database {
    programs: Vec<Program>,
    hashes: HashMap<String, usize>,
}

impl Database {
    pub fn new() -> std::io::Result<Self> {
        let programs = include_str!("../chip-8-database/database/programs.json");
        let programs = serde_json::from_str(programs)?;

        let hashes = include_str!("../chip-8-database/database/sha1-hashes.json");
        let hashes = serde_json::from_str(hashes)?;

        Ok(Database { programs, hashes })
    }

    pub fn get_metadata(&self, rom: &[u8]) -> Option<Program> {
        let mut hasher = Sha1::new();
        hasher.update(rom);
        let hash = hasher.finalize();
        let mut buf = [0u8; 40];
        let hash = base16ct::lower::encode_str(&hash, &mut buf).unwrap();

        log::info!("Looking up ROM with hash {hash}");

        self.hashes.get(hash).map(|i| {
            let mut program = self.programs[*i].clone();
            let rom = program
                .roms
                .get(hash)
                .expect("If the hash matches above it should match here")
                .clone();

            program.roms = HashMap::new();
            program.roms.insert(hash.to_owned(), rom);

            log::info!(r#"Found ROM "{}" ({hash})"#, program.title);

            program
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Program {
    title: String,
    description: Option<String>,
    // TODO: This should be a date of some kind
    release: Option<String>,
    origin: Option<Origin>,
    copyright: Option<String>,
    // TODO: See https://crates.io/crates/spdx
    license: Option<String>,
    authors: Option<Vec<String>>,
    // TODO: Support real images here
    images: Option<Vec<String>>,
    // TODO: Use an appropriate URL type here
    urls: Option<Vec<String>>,
    // TODO: Use an appropriate type for hashes
    roms: HashMap<String, Rom>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum OriginType {
    GameJam,
    Event,
    Magazine,
    Manual,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Origin {
    #[serde(rename = "type")]
    origin_type: Option<OriginType>,

    reference: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Rom {
    #[serde(rename = "file")]
    file_name: Option<String>,

    embedded_title: Option<String>,
    description: Option<String>,
    // TODO: This should be a date of some kind
    release: Option<String>,
    platforms: Vec<Platform>,
    quirky_platforms: Option<HashMap<Platform, QuirkSet>>,
    authors: Option<Vec<String>>,
    // TODO: Support real images here
    images: Option<Vec<String>>,
    // TODO: Use an appropriate URL type here
    urls: Option<Vec<String>>,
    tickrate: Option<usize>,
    start_address: Option<u16>,
    screen_rotation: Option<ScreenRotation>,
    keys: Option<HashMap<Keymap, u8>>,
    touch_input_mode: Option<TouchInputMode>,
    font_style: Option<FontStyle>,
    colors: Option<Colors>,
}

#[non_exhaustive]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Platform {
    #[serde(rename = "originalChip8")]
    OriginalChip8,

    #[serde(rename = "hybridVIP")]
    HybridVIP,

    #[serde(rename = "modernChip8")]
    ModernChip8,

    #[serde(rename = "chip48")]
    Chip48,

    #[serde(rename = "superchip1")]
    Superchip1,

    #[serde(rename = "superchip")]
    Superchip,

    #[serde(rename = "xochip")]
    XOChip,

    #[serde(rename = "chip8X")]
    Chip8X,

    #[serde(rename = "megachip8")]
    MegaChip8,
}

#[derive(Clone, Debug, Default, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(usize)]
pub enum ScreenRotation {
    #[default]
    Landscape = 0,
    Portrait = 90,
    LandscapeFlipped = 180,
    PortraitFlipped = 270,
}

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

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TouchInputMode {
    #[default]
    None,
    Swipe,
    Seg16,
    Seg16Fill,
    Gamepad,
    VIP,
}

#[non_exhaustive]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FontStyle {
    #[default]
    VIP,
    Octo,
    SCHIP,
    Dream6800,
    ETI660,
    Fish,
    Akouz1,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QuirkSet {
    shift: bool,
    memory_increment_by_x: bool,
    memory_leave_i_unchanged: bool,
    wrap: bool,
    jump: bool,
    vblank: bool,
    logic: bool,
}

// TODO: Better color type than strings here
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Colors {
    pixels: Option<Vec<String>>,
    buzzer: Option<String>,
    silence: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    mod serde {
        use super::*;
        use serde_json::Result;

        #[test]
        fn deserialize_program() -> Result<()> {
            let input = r##"{
                "title": "Test Program",
                "origin": {
                    "type": "manual",
                    "reference": "What's this supposed to be?"
                },
                "description": "A description of the program",
                "release": "2023-06-24",
                "copyright": "Probably copyrighted or something",
                "license": "MIT",
                "authors": ["Someone"],
                "images": ["https://example.com/chip8/test-program.png"],
                "urls": ["https://example.com/chip8/test-program.html"],
                "roms": {
                    "0123456789abcdef0123456789abcdef01234567": {
                        "file": "test-program.ch8",
                        "embeddedTitle": "Test Program Embedded",
                        "description": "The test program to test all programs",
                        "release": "2023-06-24",
                        "platforms": ["originalChip8"],
                        "quirkyPlatforms": {
                            "originalChip8": {
                                "shift": true,
                                "memoryIncrementByX": false,
                                "memoryLeaveIUnchanged": true,
                                "wrap": false,
                                "jump": true,
                                "vblank": false,
                                "logic": true
                            }
                        },
                        "authors": ["Someone Else"],
                        "images": ["https://example.com/chip8/test-program-detail.png"],
                        "urls": ["https://example.com/chip8/test-program.ch8"],
                        "tickrate": 10,
                        "startAddress": 512,
                        "screenRotation": 0,
                        "keys": {
                            "up": 0,
                            "down": 1,
                            "left": 2,
                            "right": 3,
                            "a": 4,
                            "b": 5,
                            "player2Up": 16,
                            "player2Down": 17,
                            "player2Left": 18,
                            "player2Right": 19,
                            "player2A": 20,
                            "player2B": 21
                        },
                        "touchInputMode": "none",
                        "fontStyle": "vip",
                        "colors": {
                            "pixels": ["#000000", "#ff0000", "#00ff00", "#0000ff"],
                            "buzzer": "#cccccc",
                            "silence": "#555555"
                        }
                    }
                }
            }"##;

            let program: Program = serde_json::from_str(input)?;

            assert_eq!(program.title, "Test Program");
            assert_eq!(
                &OriginType::Manual,
                program
                    .origin
                    .as_ref()
                    .unwrap()
                    .origin_type
                    .as_ref()
                    .unwrap()
            );
            assert_eq!(
                "What's this supposed to be?",
                program.origin.unwrap().reference.unwrap()
            );
            assert_eq!(
                "A description of the program",
                &program.description.unwrap()
            );
            assert_eq!("2023-06-24", &program.release.unwrap());
            assert_eq!(
                "Probably copyrighted or something",
                &program.copyright.unwrap()
            );
            assert_eq!("MIT", &program.license.unwrap());
            assert_eq!(vec!["Someone".to_owned()], program.authors.unwrap());
            assert_eq!(
                vec!["https://example.com/chip8/test-program.png"],
                program.images.unwrap()
            );
            assert_eq!(
                vec!["https://example.com/chip8/test-program.html"],
                program.urls.unwrap()
            );

            let rom = program.roms["0123456789abcdef0123456789abcdef01234567"].clone();

            assert_eq!("test-program.ch8", &rom.file_name.unwrap());
            assert_eq!("Test Program Embedded", &rom.embedded_title.unwrap());
            assert_eq!(
                "The test program to test all programs",
                rom.description.unwrap()
            );
            assert_eq!("2023-06-24", rom.release.unwrap());
            assert_eq!(vec![Platform::OriginalChip8], rom.platforms);

            let quirks = rom.quirky_platforms.unwrap()[&Platform::OriginalChip8].clone();

            assert!(quirks.shift);
            assert!(!quirks.memory_increment_by_x);
            assert!(quirks.memory_leave_i_unchanged);
            assert!(!quirks.wrap);
            assert!(quirks.jump);
            assert!(!quirks.vblank);
            assert!(quirks.logic);

            assert_eq!(vec!["Someone Else"], rom.authors.unwrap());
            assert_eq!(
                vec!["https://example.com/chip8/test-program-detail.png"],
                rom.images.unwrap()
            );
            assert_eq!(
                vec!["https://example.com/chip8/test-program.ch8"],
                rom.urls.unwrap()
            );
            assert_eq!(10, rom.tickrate.unwrap());
            assert_eq!(0x200, rom.start_address.unwrap());
            assert_eq!(ScreenRotation::Landscape, rom.screen_rotation.unwrap());

            let keys = rom.keys.unwrap();

            assert_eq!(0x00, keys[&Keymap::P1Up]);
            assert_eq!(0x01, keys[&Keymap::P1Down]);
            assert_eq!(0x02, keys[&Keymap::P1Left]);
            assert_eq!(0x03, keys[&Keymap::P1Right]);
            assert_eq!(0x04, keys[&Keymap::P1A]);
            assert_eq!(0x05, keys[&Keymap::P1B]);

            assert_eq!(0x10, keys[&Keymap::P2Up]);
            assert_eq!(0x11, keys[&Keymap::P2Down]);
            assert_eq!(0x12, keys[&Keymap::P2Left]);
            assert_eq!(0x13, keys[&Keymap::P2Right]);
            assert_eq!(0x14, keys[&Keymap::P2A]);
            assert_eq!(0x15, keys[&Keymap::P2B]);

            assert_eq!(TouchInputMode::None, rom.touch_input_mode.unwrap());
            assert_eq!(FontStyle::VIP, rom.font_style.unwrap());

            let colors = rom.colors.unwrap();

            assert_eq!(
                vec!["#000000", "#ff0000", "#00ff00", "#0000ff"],
                colors.pixels.unwrap()
            );
            assert_eq!("#cccccc", colors.buzzer.unwrap());
            assert_eq!("#555555", colors.silence.unwrap());

            Ok(())
        }

        #[test]
        fn deserialize_program_minimal() -> Result<()> {
            let input = r##"{
                "title": "Minimal",
                "roms": {
                    "0123456789abcdef0123456789abcdef01234567": {
                        "platforms": ["originalChip8"]
                    }
                }
            }"##;

            let program: Program = serde_json::from_str(input)?;

            assert_eq!(program.title, "Minimal");

            let rom = program.roms["0123456789abcdef0123456789abcdef01234567"].clone();

            assert_eq!(vec![Platform::OriginalChip8], rom.platforms);

            Ok(())
        }
    }
}