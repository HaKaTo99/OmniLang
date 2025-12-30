use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use omnilang_core::runtime::Decision;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CacheEntry {
    ts_millis: u128,
    decision: Decision,
}

#[derive(Debug, Clone)]
pub struct EvalCache {
    file: PathBuf,
    ttl: Duration,
    enabled: bool,
}

impl EvalCache {
    pub fn new(file: PathBuf, ttl: Duration, enabled: bool) -> Self {
        Self { file, ttl, enabled }
    }

    pub fn get(&self, key: &str) -> Option<Decision> {
        if !self.enabled {
            return None;
        }
        let map = self.read_all().ok()?;
        let entry = map.get(key)?;
        if Self::expired(entry, self.ttl) {
            return None;
        }
        Some(entry.decision.clone())
    }

    pub fn put(&self, key: &str, decision: &Decision) {
        if !self.enabled {
            return;
        }
        let mut map = self.read_all().unwrap_or_default();
        let entry = CacheEntry {
            ts_millis: Self::now_millis(),
            decision: decision.clone(),
        };
        map.insert(key.to_string(), entry);
        let _ = self.write_all(&map);
    }

    fn expired(entry: &CacheEntry, ttl: Duration) -> bool {
        if ttl.as_millis() == 0 {
            return false;
        }
        let now = Self::now_millis();
        now.saturating_sub(entry.ts_millis) > ttl.as_millis()
    }

    fn now_millis() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0)
    }

    fn read_all(&self) -> Result<HashMap<String, CacheEntry>, ()> {
        if !self.file.exists() {
            return Ok(HashMap::new());
        }
        let raw = fs::read_to_string(&self.file).map_err(|_| ())?;
        serde_json::from_str(&raw).map_err(|_| ())
    }

    fn write_all(&self, map: &HashMap<String, CacheEntry>) -> Result<(), ()> {
        if let Some(parent) = self.file.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let blob = serde_json::to_string_pretty(map).map_err(|_| ())?;
        fs::write(&self.file, blob).map_err(|_| ())
    }
}
