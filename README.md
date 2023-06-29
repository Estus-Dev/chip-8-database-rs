# CHIP-8 Database RS

Easily access the data from the [CHIP-8 Database][] from your own [CHIP-8] implementation written in Rust.

Use this data to automatically apply the needed quirks for a specific ROM, or give the user more information about individual games. You can show the name of the game in the titlebar, or display example images in menus or while emulation is stopped.

You can also provide descriptions of the various CHIP-8 platforms out there, or describe the effects of enabling various quirks.

## Usage

```rust
# #[cfg(not(feature = "hashes"))]
# fn main() {}
#
# #[cfg(feature = "hashes")]
# fn main() {
# use chip_8_database_rs::Database;
#
# let rom = [0u8; 4096];
#
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
# }
```

## Features

While the ROM database is always enabled, you can choose to not include platforms, hashes, or quirk data if you'd like to reduce your binary size. Just disable default features and pick and choose the ones you'd like.

```toml
chip_8_database_rs = { version = "X", default-features = false, features = ["hashes"]}
```

### Hashes

Whether to include the contents of `sha1-hashes.json`.

Note that without this feature, you will be unable to automatically lookup ROM data.

### Platforms

Whether to include the contents of `platforms.json`.

This is information about the various CHIP-8 platforms out there. You'll still be able to see what platform a specific ROM was built for, but you won't be able to lookup detailed information about each platform.

### Quirks

Whether to include the contents of `quirks.json`.

This is information about the quirks that may be encountered on various interpreters or emulators. Bugs, misunderstandings and/or technical limitations led to different platforms having different quirks. Many ROMs were built with a specific set of quirks in mind based on which platform they were written against.


[CHIP-8]: https://chip-8.github.io/links/
[CHIP-8 Database]: https://github.com/chip-8/chip-8-database
