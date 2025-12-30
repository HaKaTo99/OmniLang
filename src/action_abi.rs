use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Capability hints for action execution in the host.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[allow(dead_code)]
pub enum ActionCapability {
	Read,
	Write,
	Network,
	FileSystem,
	Execute,
}

/// Optional metadata for action execution.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[allow(dead_code)]
pub struct ActionMetadata {
	pub timeout_ms: Option<u64>,
	pub retry: Option<u32>,
	pub priority: Option<u8>,
}

/// Payload contract between policy decisions and host adapters.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ActionPayload {
	pub action: String,
	#[serde(default)]
	pub params: HashMap<String, serde_json::Value>,
	#[serde(default)]
	pub required_capabilities: Vec<ActionCapability>,
	#[serde(default)]
	pub metadata: ActionMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ActionResult {
	Success {
		output: Option<serde_json::Value>,
		elapsed_ms: Option<u64>,
	},
	Failed {
		error: String,
		elapsed_ms: Option<u64>,
	},
}
