//! Minimal runtime metrics collector and exporter (OpenMetrics text).

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::runtime::DecisionMetrics;

static TOTAL_POLICIES: AtomicU64 = AtomicU64::new(0);
static TOTAL_RULES: AtomicU64 = AtomicU64::new(0);
static TOTAL_ACTIONS: AtomicU64 = AtomicU64::new(0);
static TOTAL_GUARD_HITS: AtomicU64 = AtomicU64::new(0);
static TOTAL_DURATION_MS: AtomicU64 = AtomicU64::new(0);

/// Record a decision metrics snapshot into global counters.
pub fn record_decision(metrics: &DecisionMetrics) {
    TOTAL_POLICIES.fetch_add(1, Ordering::Relaxed);
    TOTAL_RULES.fetch_add(metrics.rules_evaluated as u64, Ordering::Relaxed);
    TOTAL_ACTIONS.fetch_add(metrics.actions_triggered as u64, Ordering::Relaxed);
    TOTAL_GUARD_HITS.fetch_add(metrics.guard_hits as u64, Ordering::Relaxed);
    // Saturating add to avoid overflow; ms fits in u64 for practical ranges.
    TOTAL_DURATION_MS.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
        v.checked_add(metrics.duration_ms as u64).or(Some(u64::MAX))
    }).ok();
}

/// Export metrics in OpenMetrics text format.
pub fn export_openmetrics() -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let mut out = String::new();
    out.push_str("# TYPE omnilang_policies_total counter\n");
    out.push_str(&format!("omnilang_policies_total {} {}\n", TOTAL_POLICIES.load(Ordering::Relaxed), ts));

    out.push_str("# TYPE omnilang_rules_total counter\n");
    out.push_str(&format!("omnilang_rules_total {} {}\n", TOTAL_RULES.load(Ordering::Relaxed), ts));

    out.push_str("# TYPE omnilang_actions_total counter\n");
    out.push_str(&format!("omnilang_actions_total {} {}\n", TOTAL_ACTIONS.load(Ordering::Relaxed), ts));

    out.push_str("# TYPE omnilang_guard_hits_total counter\n");
    out.push_str(&format!("omnilang_guard_hits_total {} {}\n", TOTAL_GUARD_HITS.load(Ordering::Relaxed), ts));

    out.push_str("# TYPE omnilang_duration_ms_total counter\n");
    out.push_str(&format!("omnilang_duration_ms_total {} {}\n", TOTAL_DURATION_MS.load(Ordering::Relaxed), ts));

    out.push_str("# EOF\n");
    out
}
