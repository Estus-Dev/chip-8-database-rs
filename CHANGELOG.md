# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- New `Metadata` struct to ensure the hash is always available, even on a failed lookup.

### Changed

- `Database.get_metadata()` and `Database.get_metadata_from_hash()` now return `Metadata` rather than `Option<Program>`.

### Removed

- Removed `Program.lookup_hash` property in favor of `Metadata.hash`.

## [1.0.0] - 2023-06-30

Initial Release
