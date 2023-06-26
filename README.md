# CHIP-8 Database RS

Easily access the data from the [CHIP-8 Database][] from Rust.

## Usage

```rust
# use chip_8_database_rs::Database;
#
# let rom = [0u8; 4096];

let db = Database::new().unwrap();

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

[CHIP-8 Database]: https://github.com/chip-8/chip-8-database
