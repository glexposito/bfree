use bfree::render::format::fmt_short;

#[test]
fn formats_bytes_without_unit_scaling() {
    assert_eq!(fmt_short(0), "0B");
    assert_eq!(fmt_short(999), "999B");
}

#[test]
fn formats_kibibytes_with_single_decimal_and_trims_dot_zero() {
    assert_eq!(fmt_short(1024), "1K");
    assert_eq!(fmt_short(1536), "1.5K");
}

#[test]
fn rounds_to_one_decimal_for_scaled_units() {
    let gib = 1024_u64.pow(3);
    assert_eq!(fmt_short((gib as f64 * 1.24) as u64), "1.2G");
    assert_eq!(fmt_short((gib as f64 * 1.26) as u64), "1.3G");
}

#[test]
fn trims_trailing_dot_zero_for_integer_scaled_values() {
    let gib = 1024_u64.pow(3);
    assert_eq!(fmt_short(gib), "1G");
    assert_eq!(fmt_short(10 * gib), "10G");
}
