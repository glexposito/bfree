use crate::core::memory_stats::MemoryStats;
use serde::Serialize;

mod compact;
mod extended;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructuredFormat {
    Json,
    Yaml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructuredView {
    Compact,
    Extended,
}

#[derive(Serialize)]
pub(super) struct PercentValue {
    percent: f64,
}

impl PercentValue {
    pub(super) fn new(percent: f64) -> Self {
        Self { percent }
    }
}

pub fn render(
    s: &MemoryStats,
    format: StructuredFormat,
    view: StructuredView,
) -> Result<String, String> {
    match view {
        StructuredView::Compact => serialize(&compact::CompactOutput::from_stats(s), format),
        StructuredView::Extended => serialize(&extended::ExtendedOutput::from_stats(s), format),
    }
}

fn serialize<T: Serialize>(value: &T, format: StructuredFormat) -> Result<String, String> {
    match format {
        StructuredFormat::Json => serde_json::to_string_pretty(value)
            .map_err(|e| format!("json serialization failed: {e}")),
        StructuredFormat::Yaml => serde_yaml::to_string(value)
            .map(|s| s.strip_suffix('\n').unwrap_or(&s).to_string())
            .map_err(|e| format!("yaml serialization failed: {e}")),
    }
}
