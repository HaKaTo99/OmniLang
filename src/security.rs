use sha2::{Sha256, Digest};
use std::fs;
use std::path::Path;

/// Memvalidasi integritas file berdasarkan checksum SHA-256.
pub fn verify_integrity(file_path: &str, expected_hash: &str) -> Result<bool, String> {
    let content = fs::read(file_path).map_err(|e| format!("Gagal membaca file: {}", e))?;
    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();
    let actual_hash = format!("{:x}", result);

    Ok(actual_hash == expected_hash)
}

/// Menghasilkan checksum SHA-256 untuk konten string.
pub fn generate_checksum(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    let result = hasher.finalize();
    format!("{:x}", result)
}

/// Contoh penggunaan dalam sistem kapabilitas: 
/// Hanya script dengan hash yang terdaftar yang boleh mengakses resource sensitif.
pub fn is_trusted_script(file_path: &str) -> bool {
    // Di masa depan, ini akan memeriksa database tanda tangan digital.
    // Untuk saat ini, fungsi ini adalah placeholder untuk arsitektur Zero-Trust.
    Path::new(file_path).exists()
}
