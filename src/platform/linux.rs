use std::fs;

use crate::core::memory_stats::MemoryStats;

const MEMINFO_PATH: &str = "/proc/meminfo";

mod error;
mod parser;

pub use error::LinuxMemError;
use parser::{build_memory_stats, parse_meminfo_content};

/// Read memory stats from Linux `/proc/meminfo` and return a typed domain model.
///
/// This is intentionally Linux-specific. Callers get `MemoryStats` and do not
/// need to know anything about `/proc` keys or parsing.
pub fn read_memory_stats() -> Result<MemoryStats, LinuxMemError> {
    let content = load_meminfo()?;
    parse_meminfo(&content)
}

fn parse_meminfo(content: &str) -> Result<MemoryStats, LinuxMemError> {
    let parsed = parse_meminfo_content(content)?;
    build_memory_stats(parsed)
}

fn load_meminfo() -> Result<String, LinuxMemError> {
    Ok(fs::read_to_string(MEMINFO_PATH)?)
}
