use crate::core::memory_stats::MemoryStats;

use super::LinuxMemError;

pub(super) fn parse_meminfo_content(content: &str) -> Result<ParsedMeminfo, LinuxMemError> {
    let mut parsed = ParsedMeminfo::default();
    for line in content.lines() {
        parsed.process_line(line)?;
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
    /// Parses a single line and updates the struct if the key matches a known metric.
    fn process_line(&mut self, line: &str) -> Result<(), LinuxMemError> {
        let line = line.trim();
        if line.is_empty() {
            return Ok(());
        }

        // Robustness: Ignore lines that don't look like "Key: Value"
        let Some((key, value_part)) = line.split_once(':') else {
            return Ok(());
        };

        let key = key.trim();
        let value_part = value_part.trim();

        // Dispatch to specific field parsers
        match key {
            "MemTotal" => {
                self.mem_total = Some(self.parse_required("MemTotal", value_part, line)?);
            }
            "MemAvailable" => {
                self.mem_available = Some(self.parse_required("MemAvailable", value_part, line)?);
            }
            "Cached" => {
                self.mem_cached = Some(self.parse_required("Cached", value_part, line)?);
            }
            "SReclaimable" => {
                self.mem_sreclaimable =
                    Some(self.parse_required("SReclaimable", value_part, line)?);
            }
            "Shmem" => {
                self.mem_shmem = Some(self.parse_required("Shmem", value_part, line)?);
            }
            "SwapTotal" => {
                self.swap_total = self.parse_optional(value_part, line)?;
            }
            "SwapFree" => {
                self.swap_free = self.parse_optional(value_part, line)?;
            }
            _ => {} // Ignore unknown keys
        }

        Ok(())
    }

    /// Parses a required field. Errors if value is invalid or unit is not "kB".
    fn parse_required(
        &self,
        key: &'static str,
        value_str: &str,
        full_line: &str,
    ) -> Result<u64, LinuxMemError> {
        let (value, unit) = self.parse_numeric_part(value_str, full_line)?;

        // Enforce units for required fields
        match unit {
            Some("kB") => Ok(value * 1024),
            None => Ok(value),
            Some(other_unit) => Err(LinuxMemError::UnsupportedUnit {
                key,
                unit: other_unit.to_string(),
            }),
        }
    }

    /// Parses an optional field (like Swap). Returns None if unit is unsupported.
    fn parse_optional(
        &self,
        value_str: &str,
        full_line: &str,
    ) -> Result<Option<u64>, LinuxMemError> {
        let (value, unit) = self.parse_numeric_part(value_str, full_line)?;

        // Best-effort for optional fields
        match unit {
            Some("kB") => Ok(Some(value * 1024)),
            None => Ok(Some(value)),
            Some(_) => Ok(None), // Ignore unsupported units for optional fields
        }
    }

    /// Helper to parse "123 kB" into (123, Some("kB")).
    fn parse_numeric_part<'a>(
        &self,
        s: &'a str,
        full_line: &str,
    ) -> Result<(u64, Option<&'a str>), LinuxMemError> {
        let mut parts = s.split_whitespace();

        let val_str = parts.next().ok_or_else(|| LinuxMemError::ParseLine {
            content: full_line.to_string(),
        })?;

        let value = val_str
            .parse::<u64>()
            .map_err(|_| LinuxMemError::ParseLine {
                content: full_line.to_string(),
            })?;

        let unit = parts.next();
        Ok((value, unit))
    }
}

#[cfg(test)]
mod tests {
    use super::{build_memory_stats, parse_meminfo_content};
    use crate::platform::linux::LinuxMemError;

    fn parse_to_stats(
        content: &str,
    ) -> Result<crate::core::memory_stats::MemoryStats, LinuxMemError> {
        let parsed = parse_meminfo_content(content)?;
        build_memory_stats(parsed)
    }

    #[test]
    fn rejects_wrong_unit_on_required_key() {
        let meminfo = "\
MemTotal:       1000 MB
MemAvailable:    500 kB
Cached:          100 kB
SReclaimable:     20 kB
Shmem:            10 kB
SwapTotal:      2048 kB
SwapFree:       1024 kB
";

        let err = parse_to_stats(meminfo).expect_err("required key with wrong unit must fail");
        assert!(matches!(
            err,
            LinuxMemError::UnsupportedUnit {
                key: "MemTotal",
                ..
            }
        ));
    }

    #[test]
    fn ignores_wrong_unit_on_optional_swap_keys() {
        let meminfo = "\
MemTotal:       1000 kB
MemAvailable:    500 kB
Cached:          100 kB
SReclaimable:     20 kB
Shmem:            10 kB
SwapTotal:      2048 MB
SwapFree:       1024 MB
";

        let stats = parse_to_stats(meminfo).expect("optional swap parse should be best-effort");
        assert_eq!(stats.swap_total, 0);
        assert_eq!(stats.swap_free, 0);
    }

    #[test]
    fn fails_when_required_key_is_missing() {
        let meminfo = "\
MemTotal:       1000 kB
Cached:          100 kB
SReclaimable:     20 kB
Shmem:            10 kB
";

        let err = parse_to_stats(meminfo).expect_err("missing required key must fail");
        assert!(matches!(err, LinuxMemError::MissingKey("MemAvailable")));
    }

    #[test]
    fn ignores_malformed_lines_and_unknown_keys() {
        let meminfo = "\
MemTotal:       1000 kB
MemAvailable:    500 kB
bad line without separator
UnknownKey:      123 kB
Cached:          100 kB
SReclaimable:     20 kB
Shmem:            10 kB
";

        // This used to fail, now it should succeed
        parse_to_stats(meminfo).expect("malformed lines should be ignored");
    }

    #[test]
    fn reports_parse_error_for_non_numeric_required_value() {
        let meminfo = "\
MemTotal:       not-a-number kB
MemAvailable:    500 kB
Cached:          100 kB
SReclaimable:     20 kB
Shmem:            10 kB
";

        let err = parse_to_stats(meminfo).expect_err("invalid numeric value should fail");
        assert!(matches!(err, LinuxMemError::ParseLine { .. }));
    }

    #[test]
    fn accepts_required_values_without_unit_as_bytes() {
        let meminfo = "\
MemTotal:       1024
MemAvailable:    512
Cached:          128
SReclaimable:     64
Shmem:            32
";

        let stats = parse_to_stats(meminfo).expect("unitless required values should parse");
        assert_eq!(stats.mem_total, 1024);
        assert_eq!(stats.mem_available, 512);
        assert_eq!(stats.mem_cached, 128);
        assert_eq!(stats.mem_sreclaimable, 64);
        assert_eq!(stats.mem_shmem, 32);
    }
}
