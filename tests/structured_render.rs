use bfree::core::memory_stats::MemoryStats;
use bfree::render::structured::{StructuredFormat, StructuredView, render};
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

#[test]
fn renders_compact_json_shape_and_values() {
    let gib = 1024_u64.pow(3);
    let stats = MemoryStats::new(10 * gib, 6 * gib, 2 * gib, 1024, 512, 2 * gib, gib);

    let out = render(&stats, StructuredFormat::Json, StructuredView::Compact)
        .expect("compact json should render");
    let json: JsonValue = serde_json::from_str(&out).expect("json should parse");

    assert_eq!(json["memory"]["total_bytes"], 10 * gib);
    assert_eq!(json["memory"]["used_bytes"], 4 * gib);
    assert_eq!(json["memory"]["available_bytes"], 6 * gib);
    assert_eq!(json["memory"]["used"]["percent"], 40.0);
    assert_eq!(json["memory"]["available"]["percent"], 60.0);

    assert_eq!(json["swap"]["total_bytes"], 2 * gib);
    assert_eq!(json["swap"]["used_bytes"], gib);
    assert_eq!(json["swap"]["used"]["percent"], 50.0);
}

#[test]
fn renders_extended_json_includes_cache_breakdown_and_swap_free() {
    let gib = 1024_u64.pow(3);
    let stats = MemoryStats::new(
        10 * gib,
        6 * gib,
        2 * gib,
        gib,
        512 * 1024 * 1024,
        2 * gib,
        gib,
    );

    let out = render(&stats, StructuredFormat::Json, StructuredView::Extended)
        .expect("extended json should render");
    let json: JsonValue = serde_json::from_str(&out).expect("json should parse");

    assert_eq!(json["memory"]["cache_bytes"], 2684354560_u64);
    assert_eq!(json["memory"]["cache"]["percent"], 25.0);
    assert_eq!(json["swap"]["free_bytes"], gib);
    assert_eq!(json["swap"]["free"]["percent"], 50.0);
    assert_eq!(json["cache_breakdown"]["cached_bytes"], 2 * gib);
    assert_eq!(json["cache_breakdown"]["sreclaimable_bytes"], gib);
    assert_eq!(json["cache_breakdown"]["shmem_bytes"], 512 * 1024 * 1024);
}

#[test]
fn renders_compact_yaml_shape_and_values() {
    let mib = 1024_u64.pow(2);
    let stats = MemoryStats::new(900 * mib, 300 * mib, 0, 0, 0, 0, 0);

    let out = render(&stats, StructuredFormat::Yaml, StructuredView::Compact)
        .expect("compact yaml should render");
    let yaml: YamlValue = serde_yaml::from_str(&out).expect("yaml should parse");

    assert_eq!(yaml["memory"]["total_bytes"], 900 * mib);
    assert_eq!(yaml["memory"]["used_bytes"], 600 * mib);
    assert_eq!(yaml["memory"]["used"]["percent"], 66.66666666666667_f64);
    assert_eq!(yaml["swap"]["total_bytes"], 0);
    assert_eq!(yaml["swap"]["used"]["percent"], 0.0_f64);
}

#[test]
fn renders_extended_yaml_with_zero_swap_free_percent() {
    let stats = MemoryStats::new(1024, 512, 256, 128, 64, 0, 0);

    let out = render(&stats, StructuredFormat::Yaml, StructuredView::Extended)
        .expect("extended yaml should render");
    let yaml: YamlValue = serde_yaml::from_str(&out).expect("yaml should parse");

    assert_eq!(yaml["swap"]["free"]["percent"], 0.0_f64);
    assert_eq!(yaml["cache_breakdown"]["cached_bytes"], 256);
}
