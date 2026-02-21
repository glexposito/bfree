# bfree ✨

A better, human-first `free`, inspired by tools like `btop`.
`bfree` is just starting out, but the goal is clear: a fast, elegant, zero-overhead command you’ll actually enjoy running.

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
- ⚡ **Snappy** — built with `clap`

## Status

- ✅ CLI scaffold
- 🔜 Memory logic

## Run

```bash
cargo run --
```

## Contributing

Ideas, feedback, and PRs are welcome. The project is young — jump in early and shape it.

---

Made with ❤️ for the terminal.
