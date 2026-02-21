use bfree::platform::linux;
use std::io;

#[test]
fn read_memory_stats_returns_plausible_values() {
    let stats = linux::read_memory_stats().expect("failed to read linux memory stats");

    assert!(stats.mem_total > 0, "mem_total should be positive");
    assert!(
        stats.mem_available <= stats.mem_total,
        "mem_available should be <= mem_total"
    );
    assert!(
        stats.mem_cached <= stats.mem_total,
        "mem_cached should be <= mem_total"
    );
    assert!(
        stats.mem_sreclaimable <= stats.mem_total,
        "mem_sreclaimable should be <= mem_total"
    );
    assert!(
        stats.mem_shmem <= stats.mem_total,
        "mem_shmem should be <= mem_total"
    );
    assert!(
        stats.swap_free <= stats.swap_total,
        "swap_free should be <= swap_total"
    );
}

#[test]
fn read_memory_stats_is_consistent_with_derived_methods() {
    let stats = linux::read_memory_stats().expect("failed to read linux memory stats");

    assert_eq!(
        stats.mem_used(),
        stats.mem_total.saturating_sub(stats.mem_available)
    );
    assert_eq!(
        stats.mem_cache_effective(),
        stats
            .mem_cached
            .saturating_add(stats.mem_sreclaimable)
            .saturating_sub(stats.mem_shmem)
    );
    assert_eq!(
        stats.swap_used(),
        stats.swap_total.saturating_sub(stats.swap_free)
    );
}

#[test]
fn linux_error_display_includes_context() {
    let io_err = linux::LinuxMemError::Io(io::Error::new(io::ErrorKind::NotFound, "boom"));
    let io_msg = io_err.to_string();
    assert!(io_msg.contains("failed to read /proc/meminfo"));
    assert!(io_msg.contains("boom"));

    let parse = linux::LinuxMemError::ParseLine {
        line: 42,
        content: "MemTotal abc".to_string(),
    };
    let parse_msg = parse.to_string();
    assert!(parse_msg.contains("failed to parse /proc/meminfo line 42"));
    assert!(parse_msg.contains("MemTotal abc"));

    let missing = linux::LinuxMemError::MissingKey("MemTotal");
    assert!(missing.to_string().contains("missing key"));

    let unsupported = linux::LinuxMemError::UnsupportedUnit {
        key: "MemTotal",
        unit: "MB".to_string(),
    };
    assert!(unsupported.to_string().contains("unsupported unit"));
}

#[test]
fn linux_mem_error_from_io_maps_to_io_variant() {
    let src = io::Error::new(io::ErrorKind::PermissionDenied, "nope");
    let err: linux::LinuxMemError = src.into();

    match err {
        linux::LinuxMemError::Io(e) => {
            assert_eq!(e.kind(), io::ErrorKind::PermissionDenied);
            assert_eq!(e.to_string(), "nope");
        }
        other => panic!("expected Io variant, got {other:?}"),
    }
}
