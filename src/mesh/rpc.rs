use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use crate::program_evaluator::Value;

/// Representasi subset dari tipe data OmniLang yang bisa diserialisasi via TCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RpcValue {
    Number(f64),
    String(String),
    Bool(bool),
    Unit,
    List(Vec<RpcValue>),
    Object(BTreeMap<String, RpcValue>),
}

impl RpcValue {
    /// Konversi dari core Value ke RpcValue
    pub fn from_value(val: &Value) -> Result<Self, String> {
        match val {
            Value::Number(n) => Ok(RpcValue::Number(*n)),
            Value::String(s) => Ok(RpcValue::String(s.clone())),
            Value::Bool(b) => Ok(RpcValue::Bool(*b)),
            Value::Unit => Ok(RpcValue::Unit),
            Value::List(l) => {
                let mut rp_list = Vec::new();
                for item in l {
                    rp_list.push(RpcValue::from_value(item)?);
                }
                Ok(RpcValue::List(rp_list))
            },
            Value::Object(o) => {
                let mut rp_obj = BTreeMap::new();
                for (k, v) in o {
                    rp_obj.insert(k.clone(), RpcValue::from_value(v)?);
                }
                Ok(RpcValue::Object(rp_obj))
            },
            _ => Err(format!("Cannot serialize type {:?} for mesh transport", val)),
        }
    }

    /// Konversi kembali ke core Value
    pub fn to_value(self) -> Value {
        match self {
            RpcValue::Number(n) => Value::Number(n),
            RpcValue::String(s) => Value::String(s),
            RpcValue::Bool(b) => Value::Bool(b),
            RpcValue::Unit => Value::Unit,
            RpcValue::List(l) => {
                Value::List(l.into_iter().map(|item| item.to_value()).collect())
            },
            RpcValue::Object(o) => {
                Value::Object(o.into_iter().map(|(k, v)| (k, v.to_value())).collect())
            }
        }
    }
}

/// Struktur request JSON yang menembus kabel TCP
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshRequest {
    pub function_name: String,
    pub args: Vec<RpcValue>,
    pub capability_token: Option<String>,
}

/// Struktur response JSON balik dari node worker
#[derive(Debug, Serialize, Deserialize)]
pub struct MeshResponse {
    pub result: Result<RpcValue, String>,
}
