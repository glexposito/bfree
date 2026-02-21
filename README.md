# bfree

A better `free`, human by default. Inspired by tools like `btop`, `bfree` gives you memory and swap stats in a clean one-line summary.

`bfree` is a lightweight Rust CLI that provides a clearer, human-oriented view of Linux memory and swap usage.

It keeps the speed and simplicity of `free` while improving readability and presenting practical memory semantics by default.

## Features

- Human-readable sizes and percentages
- Multiple output modes: `compact` (default), `extended`, and `pretty`
- Linux-native data source: `/proc/meminfo`
- Small dependency surface and fast startup

## Installation

### From source

```bash
git clone https://github.com/glexposito/bfree
cd bfree
cargo build --release
./target/release/bfree
```

### Arch Linux (AUR)

Release publishing to AUR is automated by the project release workflow.

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
- `cache = Cached + SReclaimable - Shmem` (effective reclaimable cache)
- `avail = MemAvailable`

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
