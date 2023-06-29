#![doc = include_str!("../README.md")]

pub mod color;
pub mod font;
pub mod input;
pub mod origin;
pub mod platform;
pub mod program;
pub mod quirk;
pub mod rom;
pub mod rotation;

use program::Program;

#[cfg(feature = "hashes")]
use sha1::{Digest, Sha1};

#[cfg(feature = "hashes")]
use std::collections::HashMap;

#[cfg(feature = "platforms")]
use platform::PlatformDetails;

#[cfg(feature = "quirks")]
use quirk::QuirkDetails;

/// Database contains the full contents of the CHIP-8 database, minus any disabled features.
#[derive(Clone, Debug, Default)]
pub struct Database {
    /// A list of all known programs written for a CHIP-8 platform.
    pub programs: Vec<Program>,

    /// A map of all known ROM hashes, used to index into [programs].
    #[cfg(feature = "hashes")]
    pub hashes: HashMap<String, usize>,

    /// A list of all known CHIP-8 variants.
    #[cfg(feature = "platforms")]
    pub platforms: Vec<PlatformDetails>,

    /// A list of common quirks in CHIP-8 implementations.
    #[cfg(feature = "quirks")]
    pub quirks: Vec<QuirkDetails>,
}

impl Database {
    /// Create a new instance of the DB. Does not touch the filesystem or network.
    pub fn new() -> Self {
        // Updating note: Panics if the `.json` files in `../chip-8-database/database/` are not in the
        // expected schema. Update the tests with the new schema and try again.

        let programs = include_str!("../chip-8-database/database/programs.json");
        let programs = serde_json::from_str(programs)
            .expect("programs.json is hardcoded and should never be in an invalid state");

        #[cfg(feature = "hashes")]
        let hashes = {
            let json = include_str!("../chip-8-database/database/sha1-hashes.json");

            serde_json::from_str(json)
                .expect("sha1-hashes.json is hardcoded and should never be in an invalid state")
        };

        #[cfg(feature = "platforms")]
        let platforms = {
            let json = include_str!("../chip-8-database/database/platforms.json");

            serde_json::from_str(json)
                .expect("platforms.json is hardcoded and should never be in an invalid state")
        };

        #[cfg(feature = "quirks")]
        let quirks = {
            let json = include_str!("../chip-8-database/database/quirks.json");

            serde_json::from_str(json)
                .expect("quirks.json is hardcoded and should never be in an invalid state")
        };

        Database {
            programs,

            #[cfg(feature = "hashes")]
            hashes,

            #[cfg(feature = "platforms")]
            platforms,

            #[cfg(feature = "quirks")]
            quirks,
        }
    }

    #[cfg(feature = "hashes")]
    /// Lookup the metadata for a specific ROM file by hashing it.
    /// Requires `hashes` feature to be enabled.
    pub fn get_metadata(&self, rom: &[u8]) -> Option<Program> {
        let mut hasher = Sha1::new();
        hasher.update(rom);
        let hash = hasher.finalize();
        let mut buf = [0u8; 40];
        let hash = base16ct::lower::encode_str(&hash, &mut buf).unwrap();

        self.get_metadata_from_hash(hash)
    }

    #[cfg(feature = "hashes")]
    /// Lookup the metadata for a specific hash string.
    /// Requires `hashes` feature to be enabled.
    pub fn get_metadata_from_hash(&self, hash: &str) -> Option<Program> {
        log::info!("Looking up ROM with hash {hash}");

        self.hashes.get(hash).map(|i| {
            let mut program = self.programs[*i].clone();
            let rom = program
                .roms
                .get(hash)
                .unwrap_or_else(|| {
                    panic!(
                        "Are the chip-8-database tests passing? ({})",
                        "✔ All hashes in sha1-hashes.json point to the right programs"
                    )
                })
                .clone();

            program.roms = HashMap::new();
            program.roms.insert(hash.to_owned(), rom);

            log::info!(r#"Found ROM "{}" ({hash})"#, program.title);

            program
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        font::FontStyle,
        input::{Keymap, TouchInputMode},
        origin::OriginType,
        platform::Platform,
        quirk::Quirk,
        rotation::ScreenRotation,
    };
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

        assert!(quirks[&Quirk::Shift]);
        assert!(!quirks[&Quirk::MemoryIncrementByX]);
        assert!(quirks[&Quirk::MemoryLeaveIUnchanged]);
        assert!(!quirks[&Quirk::Wrap]);
        assert!(quirks[&Quirk::Jump]);
        assert!(!quirks[&Quirk::VBlank]);
        assert!(quirks[&Quirk::Logic]);

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

        assert_eq!("Minimal", program.title);

        let rom = program.roms["0123456789abcdef0123456789abcdef01234567"].clone();

        assert_eq!(vec![platform::Platform::OriginalChip8], rom.platforms);

        Ok(())
    }

    #[test]
    #[cfg(feature = "platforms")]
    fn deserialize_platform_minimal() -> Result<()> {
        let input = r##"{
            "id": "originalChip8",
            "name": "Minimal Platform Example",
            "displayResolutions": ["64x32"],
            "defaultTickrate": 15,
            "quirks": {
                "shift": false,
                "memoryIncrementByX": false,
                "memoryLeaveIUnchanged": false,
                "wrap": false,
                "jump": false,
                "vblank": true,
                "logic": true
            }
        }"##;

        let platform: PlatformDetails = serde_json::from_str(input)?;

        assert_eq!(Platform::OriginalChip8, platform.id);
        assert_eq!("Minimal Platform Example", platform.name);
        assert_eq!(vec!["64x32"], platform.display_resolutions);
        assert_eq!(15, platform.default_tickrate);

        assert!(!platform.quirks[&Quirk::Shift]);
        assert!(!platform.quirks[&Quirk::MemoryIncrementByX]);
        assert!(!platform.quirks[&Quirk::MemoryLeaveIUnchanged]);
        assert!(!platform.quirks[&Quirk::Wrap]);
        assert!(!platform.quirks[&Quirk::Jump]);
        assert!(platform.quirks[&Quirk::VBlank]);
        assert!(platform.quirks[&Quirk::Logic]);

        Ok(())
    }

    #[test]
    #[cfg(feature = "platforms")]
    fn deserialize_platform() -> Result<()> {
        let input = r##"{
            "id": "hybridVIP",
            "name": "Platform Example",
            "description": "A description goes here",
            "release": "1999-12-31",
            "authors": ["Who Knows?"],
            "urls": ["https://example.com"],
            "copyright": "Probably copyrighted or something",
            "license": "GPL",
            "displayResolutions": ["128x64"],
            "defaultTickrate": 999,
            "quirks": {
                "shift": true,
                "memoryIncrementByX": false,
                "memoryLeaveIUnchanged": true,
                "wrap": false,
                "jump": true,
                "vblank": false,
                "logic": true
            }
        }"##;

        let platform: PlatformDetails = serde_json::from_str(input)?;

        assert_eq!(Platform::HybridVIP, platform.id);
        assert_eq!("Platform Example", platform.name);

        assert_eq!("A description goes here", &platform.description.unwrap());
        assert_eq!("1999-12-31", &platform.release.unwrap());
        assert_eq!(vec!["Who Knows?".to_owned()], platform.authors.unwrap());

        assert_eq!(
            vec!["https://example.com".to_owned()],
            platform.urls.unwrap()
        );

        assert_eq!(
            "Probably copyrighted or something",
            &platform.copyright.unwrap()
        );

        assert_eq!("GPL", &platform.license.unwrap());
        assert_eq!(vec!["128x64"], platform.display_resolutions);
        assert_eq!(999, platform.default_tickrate);

        assert!(platform.quirks[&Quirk::Shift]);
        assert!(!platform.quirks[&Quirk::MemoryIncrementByX]);
        assert!(platform.quirks[&Quirk::MemoryLeaveIUnchanged]);
        assert!(!platform.quirks[&Quirk::Wrap]);
        assert!(platform.quirks[&Quirk::Jump]);
        assert!(!platform.quirks[&Quirk::VBlank]);
        assert!(platform.quirks[&Quirk::Logic]);

        Ok(())
    }

    #[test]
    #[cfg(feature = "quirks")]
    fn deserialize_quirks_minimal() -> Result<()> {
        let input = r##"{
            "id": "shift",
            "name": "Minimal Quirk Example",
            "default": false,
            "ifTrue": "Do some thing",
            "ifFalse": "Do some other thing"
        }"##;

        let quirk: QuirkDetails = serde_json::from_str(input)?;

        assert_eq!(Quirk::Shift, quirk.id);
        assert_eq!("Minimal Quirk Example", quirk.name);

        assert!(!quirk.default);

        assert_eq!("Do some thing", quirk.if_true);
        assert_eq!("Do some other thing", quirk.if_false);

        Ok(())
    }

    #[test]
    #[cfg(feature = "quirks")]
    fn deserialize_quirks() -> Result<()> {
        let input = r##"{
            "id": "jump",
            "name": "Quirk Example",
            "description": "An example of a quirk",
            "default": true,
            "ifTrue": "Do some more things",
            "ifFalse": "Do some more other things"
        }"##;

        let quirk: QuirkDetails = serde_json::from_str(input)?;

        assert_eq!(Quirk::Jump, quirk.id);
        assert_eq!("Quirk Example", quirk.name);
        assert_eq!("An example of a quirk", quirk.description.unwrap());

        assert!(quirk.default);

        assert_eq!("Do some more things", quirk.if_true);
        assert_eq!("Do some more other things", quirk.if_false);

        Ok(())
    }
}
