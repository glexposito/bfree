# bfree

A human-friendly view of Linux memory and swap. Inspired by tools like `btop`, `bfree` gives you memory and swap stats in a clean one-line summary.

`bfree` is a lightweight Rust CLI that provides a clearer, human-oriented view of Linux memory and swap usage.

It keeps the speed and simplicity of `free` while improving readability and presenting practical memory semantics by default.

## Features

- Human-readable sizes and percentages
- Multiple output modes: `compact` (default), `extended`, and `pretty`
- Linux-only (currently), using `/proc/meminfo`
- Small dependency surface and fast startup

## Platform Support

`bfree` currently supports Linux only (reads `/proc/meminfo`).

## Installation

### Cargo

`bfree` is published on crates.io:

- https://crates.io/crates/bfree

Install with:

```bash
cargo install bfree
```

### From source

```bash
git clone https://github.com/glexposito/bfree
cd bfree
cargo build --release
./target/release/bfree
```

### Arch Linux (AUR)

Release publishing to AUR is automated by the project release workflow.

Package page:

- https://aur.archlinux.org/packages/bfree

```bash
<aur-helper> -S bfree
```

## Usage

```bash
bfree
```

```bash
bfree --extended
```

```bash
bfree --pretty
```

## Output Semantics

`bfree` currently uses the following memory model:

- `used = MemTotal - MemAvailable`
- `avail = MemAvailable`
- `cache = Cached + SReclaimable - Shmem` (effective reclaimable cache)

Mode notes:

- `compact` (default) shows `used` and `avail` for memory, plus swap totals/usage.
- `--extended` and `--pretty` also show `cache`.

## Project Status

Implemented:

- Linux `/proc/meminfo` parser with typed error handling
- Compact, extended, and pretty renderers
- Unit tests for memory math and render behavior
- CI for pushes and pull requests
- Release automation for GitHub releases and AUR updates

Planned:

- Fedora COPR packaging and publish pipeline
- Container/cgroup-aware mode
- PSI (Pressure Stall Information) support

## Development

```bash
cargo run --
```

```bash
cargo test --all-targets --all-features
```

## Contributing

Issues and pull requests are welcome.

## License

MIT License. See [LICENSE](LICENSE).
