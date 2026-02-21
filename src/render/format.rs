/// Format bytes into short human-readable units (binary/IEC style).
/// 1024 base, like Linux tools.
pub fn fmt_short(bytes: u64) -> String {
    const K: f64 = 1024.0;
    const M: f64 = 1024.0 * 1024.0;
    const G: f64 = 1024.0 * 1024.0 * 1024.0;
    const T: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;

    let b = bytes as f64;

    if b >= T {
        format!("{:.1}T", b / T)
    } else if b >= G {
        format!("{:.1}G", b / G)
    } else if b >= M {
        format!("{:.1}M", b / M)
    } else if b >= K {
        format!("{:.0}K", b / K)
    } else {
        format!("{bytes}B")
    }
}
