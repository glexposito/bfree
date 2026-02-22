use crate::core::memory_stats::MemoryStats;

use super::LinuxMemError;

pub(super) fn parse_meminfo_content(content: &str) -> Result<ParsedMeminfo, LinuxMemError> {
    let mut parsed = ParsedMeminfo::default();
    for (i, line) in content.lines().enumerate() {
        let line_no = i + 1;
        parsed.apply_meminfo_line(line_no, line)?;
    }
    Ok(parsed)
}

pub(super) fn build_memory_stats(parsed: ParsedMeminfo) -> Result<MemoryStats, LinuxMemError> {
    Ok(MemoryStats::new(
        parsed
            .mem_total
            .ok_or(LinuxMemError::MissingKey("MemTotal"))?,
        parsed
            .mem_available
            .ok_or(LinuxMemError::MissingKey("MemAvailable"))?,
        parsed
            .mem_cached
            .ok_or(LinuxMemError::MissingKey("Cached"))?,
        parsed
            .mem_sreclaimable
            .ok_or(LinuxMemError::MissingKey("SReclaimable"))?,
        parsed.mem_shmem.ok_or(LinuxMemError::MissingKey("Shmem"))?,
        parsed.swap_total.unwrap_or(0),
        parsed.swap_free.unwrap_or(0),
    ))
}

#[derive(Default)]
pub(super) struct ParsedMeminfo {
    mem_total: Option<u64>,
    mem_available: Option<u64>,
    mem_cached: Option<u64>,
    mem_sreclaimable: Option<u64>,
    mem_shmem: Option<u64>,
    swap_total: Option<u64>,
    swap_free: Option<u64>,
}

impl ParsedMeminfo {
    fn apply_meminfo_line(&mut self, line_no: usize, line: &str) -> Result<(), LinuxMemError> {
        let Some(parsed_line) = parse_meminfo_line(line_no, line)? else {
            return Ok(());
        };

        // Core memory fields are required; swap fields are best-effort.
        match parsed_line.key {
            "MemTotal" => {
                self.mem_total = Some(kb_required_to_bytes(
                    "MemTotal",
                    parsed_line.value,
                    parsed_line.unit,
                )?)
            }
            "MemAvailable" => {
                self.mem_available = Some(kb_required_to_bytes(
                    "MemAvailable",
                    parsed_line.value,
                    parsed_line.unit,
                )?)
            }
            "Cached" => {
                self.mem_cached = Some(kb_required_to_bytes(
                    "Cached",
                    parsed_line.value,
                    parsed_line.unit,
                )?)
            }
            "SReclaimable" => {
                self.mem_sreclaimable = Some(kb_required_to_bytes(
                    "SReclaimable",
                    parsed_line.value,
                    parsed_line.unit,
                )?)
            }
            "Shmem" => {
                self.mem_shmem = Some(kb_required_to_bytes(
                    "Shmem",
                    parsed_line.value,
                    parsed_line.unit,
                )?)
            }
            "SwapTotal" => {
                self.swap_total = kb_optional_to_bytes(parsed_line.value, parsed_line.unit)
            }
            "SwapFree" => {
                self.swap_free = kb_optional_to_bytes(parsed_line.value, parsed_line.unit)
            }
            _ => {}
        }

        Ok(())
    }
}

struct ParsedLine<'a> {
    key: &'a str,
    value: u64,
    unit: Option<&'a str>,
}

fn parse_meminfo_line(line_no: usize, line: &str) -> Result<Option<ParsedLine<'_>>, LinuxMemError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let parse_line_err = || LinuxMemError::ParseLine {
        line: line_no,
        content: trimmed.to_string(),
    };

    let (key, value_part) = trimmed.split_once(':').ok_or_else(parse_line_err)?;

    let mut parts = value_part.split_whitespace();
    let value = parts
        .next()
        .ok_or_else(parse_line_err)?
        .parse::<u64>()
        .map_err(|_| LinuxMemError::ParseLine {
            line: line_no,
            content: trimmed.to_string(),
        })?;
    let unit = parts.next();

    Ok(Some(ParsedLine {
        key: key.trim(),
        value,
        unit,
    }))
}

/// Convert a required key to bytes, expecting kB.
fn kb_required_to_bytes(
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
fn kb_optional_to_bytes(value: u64, unit: Option<&str>) -> Option<u64> {
    match unit {
        Some("kB") => Some(value * 1024),
        None => Some(value),
        Some(_) => None,
    }
}
