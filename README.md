
<p align="center">
  <img src="branding/logo.svg" alt="guid logo" width="128" height="128">
</p>

# guid

A minimal, zero-dependency CLI tool for generating **RFC 4122-compliant UUID v4** (also known as GUID) strings, written in [Zig](https://ziglang.org/).

## Features

- **RFC 4122 compliant** — correctly sets version (4) and variant bits on every generated UUID.
- **Zero external dependencies** — only uses Zig's standard library.
- **Minimal and fast** — a single source file with ~45 lines of logic.
- **Portable** — works on any platform that Zig targets.

## Output Format

Each generated UUID is printed to stdout wrapped in curly braces:

```
{550e8400-e29b-41d4-a716-446655440000}
```

## Getting Started

### Prerequisites

- [Zig](https://ziglang.org/download/) (tested with 0.14.x, but should work with recent versions)

### Build

```bash
zig build
```

The compiled binary will be placed in `zig-out/bin/guid` (or `guid.exe` on Windows).

### Run

```bash
zig build run
```

Or run the binary directly:

```bash
./zig-out/bin/guid
```

### Install

Copy the binary to a directory in your `PATH`:

```bash
# Linux / macOS
cp zig-out/bin/guid ~/.local/bin/

# Windows (PowerShell)
copy zig-out\bin\guid.exe %USERPROFILE%\.local\bin\
```

## Usage

```bash
guid
```

Simply run the command — it generates one UUID v4 and prints it to stdout. There are no flags or options.

## How It Works

1. **Random bytes** — 16 random bytes are generated using Zig's `io.random()`.
2. **Bit manipulation** — The version and variant bits are set according to RFC 4122:
   - `buf[6] = (buf[6] & 0x0f) | 0x40` — sets the version field to `0100` (version 4).
   - `buf[8] = (buf[8] & 0x3f) | 0x80` — sets the variant to `10` (RFC 4122).
3. **Hex encoding** — The 16 bytes are converted to a 36-character hexadecimal string with hyphens inserted at the standard positions.

## Project Structure

```
guid/
├── src/
│   └── main.zig       # Main source code
├── build.zig          # Zig build configuration
├── .gitignore         # Git ignore rules
└── README.md          # This file
```

## License

[MIT](LICENSE) (or choose your own — this project does not yet include a license file).
