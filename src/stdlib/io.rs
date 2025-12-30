use crate::OmniError;
use serde_json::Value;
use std::fs;
use std::path::Path;

/// Membaca file JSON dan mengembalikan serde_json::Value.
pub fn read_json_file<P: AsRef<Path>>(path: P) -> Result<Value, OmniError> {
    let path_ref = path.as_ref();
    let content = fs::read_to_string(path_ref)
        .map_err(|e| OmniError::Io(format!("{}: {}", path_ref.display(), e)))?;
    serde_json::from_str(&content)
        .map_err(|e| OmniError::Json(format!("{}: {}", path_ref.display(), e)))
}

/// Menulis Value ke file dalam bentuk JSON yang terformat.
pub fn write_json_pretty<P: AsRef<Path>>(path: P, value: &Value) -> Result<(), OmniError> {
    let path_ref = path.as_ref();
    let data = serde_json::to_string_pretty(value)
        .map_err(|e| OmniError::Json(format!("serialize: {}", e)))?;
    if let Some(parent) = path_ref.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .map_err(|e| OmniError::Io(format!("{}: {}", parent.display(), e)))?;
        }
    }
    fs::write(path_ref, data)
        .map_err(|e| OmniError::Io(format!("{}: {}", path_ref.display(), e)))
}
