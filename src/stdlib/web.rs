use crate::OmniError;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

static MOCKS: LazyLock<Mutex<HashMap<String, Value>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// Mendapatkan JSON dari URL sederhana. Saat ini hanya mendukung skema file://.
///
/// - "file:///C:/path/to/file.json" (Windows) atau "file:///home/user/file.json" (Unix)
/// - "file://C:/path/to/file.json" juga diterima
pub fn get_json(url: &str) -> Result<Value, OmniError> {
    if let Some(mock) = lookup_mock(url) {
        return Ok(mock);
    }

    if url.starts_with("file://") {
        let path = file_path_from_url(url)?;
        let content = fs::read_to_string(&path)
            .map_err(|e| OmniError::Io(format!("{}: {}", path.display(), e)))?;
        return serde_json::from_str(&content)
            .map_err(|e| OmniError::Json(format!("{}: {}", path.display(), e)));
    }

    Err(OmniError::Unsupported(
        "HTTP/HTTPS harus dimock dengan register_mock_json atau gunakan file://".to_string(),
    ))
}

fn file_path_from_url(url: &str) -> Result<PathBuf, OmniError> {
    let prefix = "file://";
    if !url.starts_with(prefix) {
        return Err(OmniError::Unsupported(
            "hanya skema file:// yang didukung untuk get_json".to_string(),
        ));
    }
    let mut rest = &url[prefix.len()..];
    if rest.starts_with('/') {
        // Windows: file:///C:/path -> drop satu '/' di depan drive letter.
        let bytes = rest.as_bytes();
        if bytes.len() > 2 && (bytes[1] as char).is_ascii_alphabetic() && bytes[2] == b':' {
            rest = &rest[1..];
        }
    }
    Ok(PathBuf::from(rest))
}

/// Mendaftarkan respon mock untuk URL tertentu (http/https/file). Menimpa jika sudah ada.
pub fn register_mock_json(url: &str, value: Value) {
    let mut map = MOCKS.lock().unwrap();
    map.insert(url.to_string(), value);
}

/// Menghapus respon mock; jika url None, bersihkan semua.
pub fn clear_mocks(url: Option<&str>) {
    let mut map = MOCKS.lock().unwrap();
    if let Some(u) = url {
        map.remove(u);
    } else {
        map.clear();
    }
}

fn lookup_mock(url: &str) -> Option<Value> {
    let map = MOCKS.lock().unwrap();
    map.get(url).cloned()
}

#[cfg(test)]
mod tests {
    use super::file_path_from_url;

    #[test]
    fn file_url_handles_drive_prefix() {
        let path = file_path_from_url("file:///C:/tmp/data.json").unwrap();
        assert!(path.to_string_lossy().contains("C:"));
        let path2 = file_path_from_url("file://C:/tmp/data.json").unwrap();
        assert!(path2.to_string_lossy().contains("C:"));
    }
}
