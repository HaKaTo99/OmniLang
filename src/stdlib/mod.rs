//! Stdlib inti v1.3 (expanded): modul time, io, web, math, string, collections, json.

pub mod io;
pub mod time;
pub mod web;
pub mod math;
pub mod string;
pub mod collections;
pub mod json;
pub mod crypto;
pub mod tensor;

pub const VERSION: &str = "1.0.0-stable";

pub use io::{read_json_file, write_json_pretty};
pub use time::{add_millis, duration_between_ms, now_iso8601, now_unix_millis, parse_iso8601, to_utc_iso8601, truncate_to_date_iso8601, truncate_to_hour_iso8601};
pub use web::{clear_mocks, get_json, register_mock_json};
pub use math::{avg, clamp, round, min, max, sum, stddev, abs, pow, sqrt, ceil, floor};
pub use string::{contains, split, format, lowercase, uppercase, trim, length, starts_with, ends_with, matches_regex, replace, replace_first};
pub use collections::{filter_numbers, map_numbers, reduce_numbers, filter_strings, map_strings, reduce_strings};
pub use json::{get_path, set_path};
pub use crypto::{hash_sha256, hmac_sha256, base64_encode, base64_decode, random_bytes, random_hex};
pub use tensor::{dot, matmul, transpose, matvec, norm_l2};
