use crate::OmniError;
use time::format_description::well_known::Rfc3339;
use time::{Duration, OffsetDateTime, Time, UtcOffset};

/// Mengembalikan waktu UTC sekarang dalam format RFC3339 (ISO-8601).
pub fn now_iso8601() -> String {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .unwrap_or_else(|_| "".to_string())
}

/// Mengembalikan waktu epoch dalam milidetik (UTC) sebagai i128.
pub fn now_unix_millis() -> i128 {
    let dt = OffsetDateTime::now_utc();
    dt.unix_timestamp_nanos() / 1_000_000
}

/// Parse string ISO-8601 menjadi OffsetDateTime.
pub fn parse_iso8601(s: &str) -> Result<OffsetDateTime, OmniError> {
    OffsetDateTime::parse(s, &Rfc3339).map_err(|e| OmniError::InvalidInput(format!("parse iso8601: {}", e)))
}

/// Selisih antara dua ISO-8601 (b2 - a1) dalam milidetik.
pub fn duration_between_ms(a: &str, b: &str) -> Result<i128, OmniError> {
    let t1 = parse_iso8601(a)?;
    let t2 = parse_iso8601(b)?;
    Ok((t2 - t1).whole_milliseconds())
}

/// Menambah milidetik ke ISO-8601, mengembalikan ISO-8601 baru.
pub fn add_millis(base: &str, delta_ms: i128) -> Result<String, OmniError> {
    let t = parse_iso8601(base)?;
    let dt = Duration::milliseconds(delta_ms as i64);
    let new = t.checked_add(dt).ok_or_else(|| OmniError::InvalidInput("overflow add_millis".to_string()))?;
    new.format(&Rfc3339)
        .map_err(|e| OmniError::InvalidInput(format!("format iso8601: {}", e)))
}

/// Ubah offset ke UTC dan kembalikan ISO-8601.
pub fn to_utc_iso8601(s: &str) -> Result<String, OmniError> {
    let t = parse_iso8601(s)?;
    let utc = t.to_offset(UtcOffset::UTC);
    utc.format(&Rfc3339)
        .map_err(|e| OmniError::InvalidInput(format!("format iso8601: {}", e)))
}

/// Potong ke awal hari (00:00:00) mempertahankan offset asli.
pub fn truncate_to_date_iso8601(s: &str) -> Result<String, OmniError> {
    let t = parse_iso8601(s)?;
    let midnight = Time::MIDNIGHT;
    let truncated = t
        .replace_time(midnight);
    truncated
        .format(&Rfc3339)
        .map_err(|e| OmniError::InvalidInput(format!("format iso8601: {}", e)))
}

/// Potong ke awal jam (mm:ss -> 00:00) mempertahankan offset asli.
pub fn truncate_to_hour_iso8601(s: &str) -> Result<String, OmniError> {
    let t = parse_iso8601(s)?;
    let replaced = t
        .replace_minute(0)
        .and_then(|v| v.replace_second(0))
        .and_then(|v| v.replace_nanosecond(0))
        .map_err(|e| OmniError::InvalidInput(format!("truncate_to_hour: {}", e)))?;
    replaced
        .format(&Rfc3339)
        .map_err(|e| OmniError::InvalidInput(format!("format iso8601: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_conversion_and_truncation() {
        let src = "2024-01-02T15:30:45+07:00";

        let utc = to_utc_iso8601(src).unwrap();
        assert_eq!(utc, "2024-01-02T08:30:45Z");

        let day = truncate_to_date_iso8601(src).unwrap();
        assert_eq!(day, "2024-01-02T00:00:00+07:00");

        let hour = truncate_to_hour_iso8601(src).unwrap();
        assert_eq!(hour, "2024-01-02T15:00:00+07:00");
    }
}
