# CHIP-8 Database RS

Easily access the data from the [CHIP-8 Database][] from Rust.

## Usage

```rust
# use chip_8_database_rs::Database;
#
# let rom = [0u8; 4096];

let db = Database::new().unwrap();
let metadata = db.get_metadata(&rom).unwrap_or_default();

println!("ROM Title: {}", metadata.title);

if let Some(description) = metadata.description {
    println!("ROM Description: {description}");
}
```

[CHIP-8 Database]: https://github.com/chip-8/chip-8-database
