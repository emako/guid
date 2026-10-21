<p align="center">
  <img src="branding/logo.svg" alt="guid logo" width="128" height="128">
</p>

# guid

**Version 2.0** — a minimal, zero-dependency CLI tool for generating **RFC 4122-compliant UUID v4** (also known as GUID) strings, written in Rust.

## Features

- **RFC 4122 compliant** — correctly sets version (4) and variant bits on every generated UUID.
- **Zero external dependencies** — only the Rust standard library.
- **Small release binary** — `opt-level = "z"`, fat LTO, one codegen unit, `panic = abort`, and stripped symbols.
- **Static CRT on Windows** — MSVC builds link `libcmt` into the executable and do not need `vcruntime140.dll`. They still load Windows system libraries (`kernel32.dll`, `bcrypt.dll`).
- **Portable** — Windows, Linux, and macOS.

## Output Format

Each generated UUID is printed to stdout wrapped in curly braces:

```
{550e8400-e29b-41d4-a716-446655440000}
```

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) 1.85 or newer (edition 2024). Continuous integration uses 1.99.0.

### Build

```bash
cargo build --release
```

The compiled binary is `target/release/guid` (`target/release/guid.exe` on Windows).

On Windows, `.cargo/config.toml` passes `-C target-feature=+crt-static` for the MSVC targets. Leave `RUSTFLAGS` unset so that setting is not replaced.

### Run

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/guid
```

### Install

Copy the binary to a directory in your `PATH`:

```bash
# Linux / macOS
cp target/release/guid ~/.local/bin/

# Windows (PowerShell)
copy target\release\guid.exe %USERPROFILE%\.local\bin\
```

### Tests

```bash
cargo test
```

## Usage

```bash
guid
```

Simply run the command — it generates one UUID v4 and prints it to stdout. There are no flags or options.

## Releases

Tagged commits publish ZIP archives for Windows:

- `guid-windows-x86_64.zip`
- `guid-windows-aarch64.zip`

Each archive contains `guid.exe`, built with the size-first release profile and a statically linked C runtime.

## How It Works

1. **Random bytes** — 16 random bytes come from `BCryptGenRandom` on Windows, and from `/dev/urandom` on other platforms.
2. **Bit manipulation** — The version and variant bits are set according to RFC 4122:
   - `bytes[6] = (bytes[6] & 0x0f) | 0x40` — sets the version field to `0100` (version 4).
   - `bytes[8] = (bytes[8] & 0x3f) | 0x80` — sets the variant to `10` (RFC 4122).
3. **Hex encoding** — The 16 bytes are written as a 36-character lowercase hexadecimal string with hyphens at the standard positions, wrapped in braces.

## Project Structure

```
guid/
├── src/
│   └── main.rs                 # Program and tests
├── Cargo.toml                  # Package manifest and size-first release profile
├── Cargo.lock                  # Locked dependency graph (no third-party crates)
├── .cargo/config.toml          # Static CRT flags for Windows MSVC
├── .github/check_static_crt.py    # Rejects Windows binaries that import the dynamic CRT
├── .github/workflows/release.yml
├── branding/
├── .gitignore
├── LICENSE
└── README.md
```

## License

[MIT](LICENSE)
