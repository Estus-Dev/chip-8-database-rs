# CHIP-8 Database RS

[![Github Badge][]][Github]
![License Badge][]
[![Crates.io Badge][]][Crates.io]
[![Docs.rs Badge][]][Docs.rs]

Easily access the data from the [CHIP-8 Database][] from your own [CHIP-8] implementation written in Rust.

Use this data to automatically apply the needed quirks for a specific ROM, or give the user more information about individual games. You can show the name of the game in the titlebar, or display a description of the ROM underneath the main window.

You can also provide descriptions of the various CHIP-8 platforms out there, or describe the effects of enabling various quirks.

> Isn't this "cheating"? If I'm trying to learn how to build an emulator, why would I pull in this package?

Don't worry. The only things in this crate that you should use in your emulator's core are the ROM lookup functions, to determine which quirks to apply. It's still up to you to build your emulator and implement those quirks.

The goal of the rest of this crate is to help with the polish in your emulator's frontend. If you wouldn't shy away from using a crate like `winit` or `eframe` to create a window, you shouldn't worry about using this either.

## Usage

```rust
// The CHIP-8 Database is included in this library, no need to open or download files
let db = Database::new();

// Get metadata from a rom directly
let metadata = db.get_metadata(&rom);

// Get metadata from a hash string
let metadata = db.get_metadata_from_hash("0df2789f661358d8f7370e6cf93490c5bcd44b01").unwrap();

println!("ROM Title: {}", metadata.title);

// Most fields are optional in the base schema
if let Some(description) = metadata.description {
    println!("ROM Description: {description}");
}
```

## Features

While the ROM database is always enabled, there is additional data from `platforms.json` and `quirks.json` that you can choose to include with the `extra-data` feature.

```toml
chip_8_database_rs = { version = "X", features = ["extra-data"] }
```

[CHIP-8]: https://chip-8.github.io/links/
[CHIP-8 Database]: https://github.com/chip-8/chip-8-database
[Crates.io]: https://crates.io/crates/chip8_db
[Crates.io Badge]: https://img.shields.io/crates/v/chip8_db
[Docs.rs]: https://docs.rs/crate/chip8_db
[Docs.rs Badge]: https://img.shields.io/docsrs/chip8_db/latest?logo=docsdotrs
[License Badge]: https://img.shields.io/github/license/Estus-Dev/chip-8-database-rs
[Github]: https://github.com/Estus-Dev/chip-8-database-rs
[Github Badge]: https://img.shields.io/badge/github-source-%20?logo=github
