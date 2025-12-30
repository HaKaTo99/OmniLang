use omnilang_core::stdlib::{add_millis, clear_mocks, duration_between_ms, get_json, now_iso8601, now_unix_millis, parse_iso8601, read_json_file, register_mock_json, write_json_pretty};
use serde_json::json;
use std::fs;
use std::path::PathBuf;

#[test]
fn time_functions_return_values() {
    let ts1 = now_unix_millis();
    let ts2 = now_unix_millis();
    assert!(ts2 >= ts1);

    let iso = now_iso8601();
    assert!(!iso.is_empty());
    // Format parse check
    parse_iso8601(&iso).expect("iso8601 harus valid");
}

#[test]
fn io_read_write_json() {
    let tmp = std::env::temp_dir().join("omnilang_stdlib_io_test.json");
    let value = json!({ "hello": "world", "n": 7 });
    write_json_pretty(&tmp, &value).expect("write gagal");
    let back = read_json_file(&tmp).expect("read gagal");
    assert_eq!(back["hello"], "world");
    fs::remove_file(&tmp).ok();
}

#[test]
fn web_get_json_supports_file_scheme() {
    let tmp = std::env::temp_dir().join("omnilang_stdlib_web.json");
    fs::write(&tmp, "{\"ok\":true}").expect("write fixture");

    let url = file_url(&tmp);
    let val = get_json(&url).expect("get_json file:// gagal");
    assert_eq!(val["ok"], true);
    fs::remove_file(&tmp).ok();
}

#[test]
fn web_mock_http() {
    clear_mocks(None);
    register_mock_json("https://api.example.com/data", json!({"x": 42}));
    let val = get_json("https://api.example.com/data").expect("mock harus tersedia");
    assert_eq!(val["x"], 42);
    clear_mocks(Some("https://api.example.com/data"));
}

#[test]
fn time_duration_helpers() {
    let base = "2024-01-01T00:00:00Z";
    let later = add_millis(base, 1500).expect("add_millis");
    let diff = duration_between_ms(base, &later).expect("duration_between_ms");
    assert_eq!(diff, 1500);
}

fn file_url(path: &PathBuf) -> String {
    let abs = fs::canonicalize(path).unwrap_or_else(|_| path.clone());
    let mut s = abs.to_string_lossy().to_string();
    if cfg!(windows) && s.starts_with(r"\\?\") {
        s = s.trim_start_matches(r"\\?\").to_string();
    }
    s = s.replace('\\', "/");
    if cfg!(windows) {
        format!("file:///{}", s)
    } else {
        if !s.starts_with('/') {
            s.insert(0, '/');
        }
        format!("file://{}", s)
    }
}
