use std::fs;

use crate::core::memory_stats::MemoryStats;

const MEMINFO_PATH: &str = "/proc/meminfo";

#[derive(Debug)]
pub enum LinuxMemError {
    Io(std::io::Error),
    ParseLine { line: usize, content: String },
    MissingKey(&'static str),
    UnsupportedUnit { key: &'static str, unit: String },
}

impl std::fmt::Display for LinuxMemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LinuxMemError::Io(e) => write!(f, "failed to read {MEMINFO_PATH}: {e}"),
            LinuxMemError::ParseLine { line, content } => {
                write!(f, "failed to parse {MEMINFO_PATH} line {line}: {content}")
            }
            LinuxMemError::MissingKey(key) => write!(f, "missing key in {MEMINFO_PATH}: {key}"),
            LinuxMemError::UnsupportedUnit { key, unit } => {
                write!(f, "unsupported unit for {key} in {MEMINFO_PATH}: {unit}")
            }
        }
    }
}

impl From<std::io::Error> for LinuxMemError {
    fn from(e: std::io::Error) -> Self {
        LinuxMemError::Io(e)
    }
}

/// Read memory stats from Linux `/proc/meminfo` and return a typed domain model.
///
/// This is intentionally Linux-specific. Callers get `MemoryStats` and do not
/// need to know anything about `/proc` keys or parsing.
pub fn read_memory_stats() -> Result<MemoryStats, LinuxMemError> {
    let mut parsed = ParsedMeminfo::default();
    let content = fs::read_to_string(MEMINFO_PATH)?;

    for (i, line) in content.lines().enumerate() {
        let line_no = i + 1;
        let Some((key, value, unit)) = parse_meminfo_line(line_no, line)? else {
            continue;
        };

        match key {
            "MemTotal" => parsed.mem_total = Some(to_bytes_kb_required("MemTotal", value, unit)?),
            "MemAvailable" => {
                parsed.mem_available = Some(to_bytes_kb_required("MemAvailable", value, unit)?)
            }
            "Cached" => parsed.mem_cached = Some(to_bytes_kb_required("Cached", value, unit)?),
            "SReclaimable" => {
                parsed.mem_sreclaimable = Some(to_bytes_kb_required("SReclaimable", value, unit)?)
            }
            "Shmem" => parsed.mem_shmem = Some(to_bytes_kb_required("Shmem", value, unit)?),
            "SwapTotal" => parsed.swap_total = to_bytes_kb_optional(value, unit),
            "SwapFree" => parsed.swap_free = to_bytes_kb_optional(value, unit),
            _ => {}
        }
    }

    Ok(MemoryStats::new(
        required(parsed.mem_total, "MemTotal")?,
        required(parsed.mem_available, "MemAvailable")?,
        required(parsed.mem_cached, "Cached")?,
        required(parsed.mem_sreclaimable, "SReclaimable")?,
        required(parsed.mem_shmem, "Shmem")?,
        parsed.swap_total.unwrap_or(0),
        parsed.swap_free.unwrap_or(0),
    ))
}

#[derive(Default)]
struct ParsedMeminfo {
    mem_total: Option<u64>,
    mem_available: Option<u64>,
    mem_cached: Option<u64>,
    mem_sreclaimable: Option<u64>,
    mem_shmem: Option<u64>,
    swap_total: Option<u64>,
    swap_free: Option<u64>,
}

fn parse_meminfo_line(
    line_no: usize,
    line: &str,
) -> Result<Option<(&str, u64, Option<&str>)>, LinuxMemError> {
    let line_trimmed = line.trim();
    if line_trimmed.is_empty() {
        return Ok(None);
    }

    let (key, rest) = line_trimmed
        .split_once(':')
        .ok_or_else(|| LinuxMemError::ParseLine {
            line: line_no,
            content: line_trimmed.to_string(),
        })?;

    let mut parts = rest.split_whitespace();
    let value = parts
        .next()
        .ok_or_else(|| LinuxMemError::ParseLine {
            line: line_no,
            content: line_trimmed.to_string(),
        })?
        .parse::<u64>()
        .map_err(|_| LinuxMemError::ParseLine {
            line: line_no,
            content: line_trimmed.to_string(),
        })?;
    let unit = parts.next();

    Ok(Some((key.trim(), value, unit)))
}

fn required(value: Option<u64>, key: &'static str) -> Result<u64, LinuxMemError> {
    value.ok_or(LinuxMemError::MissingKey(key))
}

/// Convert a required key to bytes, expecting kB.
fn to_bytes_kb_required(
    key: &'static str,
    value: u64,
    unit: Option<&str>,
) -> Result<u64, LinuxMemError> {
    match unit {
        Some("kB") => Ok(value * 1024),
        None => Ok(value), // defensive: if unit missing, treat as bytes
        Some(other) => Err(LinuxMemError::UnsupportedUnit {
            key,
            unit: other.to_string(),
        }),
    }
}

/// Convert an optional key to bytes (kB). Returns None for unsupported units.
fn to_bytes_kb_optional(value: u64, unit: Option<&str>) -> Option<u64> {
    match unit {
        Some("kB") => Some(value * 1024),
        None => Some(value),
        Some(_) => None,
    }
}
