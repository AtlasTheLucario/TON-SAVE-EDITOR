# ToN Save Editor

A native Windows desktop application written in Rust with `egui`.

## Current status: 0.1.0 starter

This first milestone provides:
- Native egui desktop window (no embedded browser).
- Save-code text buffer.
- Import from a text file.
- Export to a text file.
- Automatic timestamped backups before replacing editor contents or overwriting an existing export target.
- A modular roadmap matching the visible sections of the original Nepushiro save editor.

**Important:** the game's save-code encoding has not yet been reverse-engineered in this project. This starter deliberately treats save codes as opaque text and does not claim to parse or edit their internal fields yet. Do not use it to modify a valuable save until the parser and round-trip tests are implemented.

## Build on Windows

Install the Rust toolchain from https://rustup.rs/ and then, in this folder, run:

```powershell
cargo run
```

Build an optimized executable:

```powershell
cargo build --release
```

The executable will be at:

```text
target\\release\\ton-save-editor.exe
```

## Planned implementation order

1. Identify the save-code format and implement a lossless parser/serializer.
2. Add round-trip tests using user-provided sample codes (redact any personal information first).
3. Implement the Save Stats panel.
4. Add collection groups: Terrors, Alternates, Moons, Specials, Unbound Rounds and Beyond's Masks.
5. Add Locations, Achievements and Collab Codes.
6. Add Unlock All with a confirmation prompt and automatic pre-edit backup.
7. Add backup history/restore UI and validate exports against the original editor.

## Credits and scope

This is an independent implementation. The original online editor credits Nepushiro and the original tracker credits Cinossu. This project does not bundle their website code or assets.
