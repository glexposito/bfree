# bfree ✨

A better `free`, human by default.
Inspired by tools like `btop`, `bfree` gives you memory and swap stats in a clean one-line summary.

The command nobody asked for — but your RAM deserves it. 🚀

## Why bfree?

The traditional `free` command is powerful but outdated in UX and modern workload awareness.

### Problems with `free`

- ❌ Not human-readable by default (KB output)
- ❌ Confusing memory semantics (`used` vs `free` vs `buff/cache`)
- ❌ No percentages
- ❌ No visual signal
- ❌ Not container/cgroup aware
- ❌ No context about reclaimable memory
- ❌ No PSI (pressure stall info)
- ❌ Misleading inside Kubernetes / Docker / Podman

Modern systems deserve better visibility.

## Highlights

- 🧼 **Clean by design** — minimal, focused, and pleasant to read
- 📊 **Human-first output** — sizes + percentages in one line
- 🧠 **Useful memory semantics** — includes effective reclaimable cache
- ⚡ **Snappy** — lightweight Rust CLI

## Status

- ✅ Linux `/proc/meminfo` parser
- ✅ One-line memory + swap output
- ✅ Percentages for used/cache/available/swap
- ✅ Unit tests for memory math
- 🔜 Container/cgroup-aware mode
- 🔜 PSI (pressure stall information)
- 🔜 Optional visual signal (bars/colors)

## Current Semantics

- `used` = `MemTotal - MemAvailable`
- `cache` = `Cached + SReclaimable - Shmem` (effective reclaimable cache)
- `avail` = `MemAvailable`

## Run

```bash
cargo run --
```

## Contributing

Ideas, feedback, and PRs are welcome. The project is young — jump in early and shape it.

---

Made with ❤️ for the terminal.
