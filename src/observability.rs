use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceId(u64);

impl TraceId {
	pub fn new() -> Self {
		static COUNTER: AtomicU64 = AtomicU64::new(1);
		TraceId(COUNTER.fetch_add(1, Ordering::Relaxed))
	}

	pub fn as_u64(&self) -> u64 {
		self.0
	}
}

impl Default for TraceId {
	fn default() -> Self {
		Self::new()
	}
}

#[derive(Debug, Default)]
pub struct Logger;

impl Logger {
	#[allow(dead_code)]
	pub fn log(&self, _msg: &str) {
		// placeholder structured logger
	}
}

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn init_global_logger() {
	LOGGER.get_or_init(|| Logger);
}

#[allow(dead_code)]
pub fn global_logger() -> &'static Logger {
	LOGGER.get_or_init(|| Logger)
}

thread_local! {
	static TRACE: std::cell::RefCell<Option<TraceId>> = const { std::cell::RefCell::new(None) };
}

pub fn set_global_trace(id: TraceId) {
	TRACE.with(|t| {
		*t.borrow_mut() = Some(id);
	});
}

pub fn current_trace() -> Option<TraceId> {
	TRACE.with(|t| *t.borrow())
}

/// Prefix log lines with timestamp, level, and trace id if available.
pub fn format_log(msg: &str) -> String {
	format_log_level(msg, "INFO")
}

pub fn format_log_level(msg: &str, level: &str) -> String {
	let ts = OffsetDateTime::now_utc()
		.format(&Rfc3339)
		.unwrap_or_else(|_| "".to_string());
	let trace = current_trace().map(|t| format!("[trace:{}]", t.as_u64()));
	match trace {
		Some(tr) => format!("[{}][{}]{} {}", ts, level, tr, msg),
		None => format!("[{}][{}] {}", ts, level, msg),
	}
}
