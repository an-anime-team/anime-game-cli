pub mod config;
pub mod output;

/// Convert bytes to gigabytes with 2 digits round
pub fn format_size(bytes: u64) -> f64 {
    (bytes as f64 / 1024.0 / 1024.0 / 1024.0 * 100.0).ceil() / 100.0
}
