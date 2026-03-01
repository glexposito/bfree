/// Format bytes into short human-readable IEC units.
/// 1024 base: KiB, MiB, GiB, TiB.
pub fn fmt_short(bytes: u64) -> String {
    const K: f64 = 1024.0;
    const M: f64 = 1024.0 * 1024.0;
    const G: f64 = 1024.0 * 1024.0 * 1024.0;
    const T: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

    let b = bytes as f64;

    fn fmt(value: f64, unit: &str) -> String {
        let mut s = format!("{value:.1}");
        if s.ends_with(".0") {
            s.truncate(s.len() - 2);
        }
        format!("{s}{unit}")
    }

    if b >= T {
        fmt(b / T, "TiB")
    } else if b >= G {
        fmt(b / G, "GiB")
    } else if b >= M {
        fmt(b / M, "MiB")
    } else if b >= K {
        fmt(b / K, "KiB")
    } else {
        format!("{bytes}B")
    }
}
