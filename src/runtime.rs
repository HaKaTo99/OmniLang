use crate::action_abi::ActionResult;
use crate::ast::Policy;
use crate::evaluator::evaluate_condition;
use crate::error::OmniError;
use crate::observability::format_log;
use crate::omniroutine::{OmniRoutine, RoutineTask};
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fs;
use std::time::Instant;

// Safety guards for loop execution
const MAX_LOOP_ITERATIONS: usize = 50; // Lowered for better demo flow
const MAX_LOOP_TIME_MS: u128 = 1_000; // 1 second max per loop

pub struct Runtime {
    context_data: Value,
}

fn resolve_loop_elements(data: &Value, collection_path: &str) -> Vec<Value> {
    if let Some(arr) = resolve_array_path(data, collection_path) {
        let capped: Vec<Value> = arr.into_iter().take(MAX_LOOP_ITERATIONS).collect();
        return capped;
    }
    // fallback to 3 null iterations to preserve legacy behavior
    vec![Value::Null, Value::Null, Value::Null]
}

fn resolve_array_path(root: &Value, path: &str) -> Option<Vec<Value>> {
    let mut current = root;
    for segment in path.split('.') {
        if segment.is_empty() {
            return None;
        }
        // handle base[index]
        let mut base = segment;
        let mut indexes: Vec<usize> = Vec::new();
        while let Some(start) = base.find('[') {
            let end = base[start + 1..].find(']')? + start + 1;
            let idx_str = &base[start + 1..end];
            let idx = idx_str.parse::<usize>().ok()?;
            indexes.push(idx);
            base = &base[..start];
        }

        if !base.is_empty() {
            current = current.get(base)?;
        }
        for idx in indexes {
            current = current.get(idx)?;
        }
    }
    current.as_array().cloned()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub actions: Vec<String>,
    pub logs: Vec<String>,
    pub guard_triggered: bool,
    pub metrics: DecisionMetrics,
    pub traces: Vec<TraceEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DecisionMetrics {
    pub rules_evaluated: usize,
    pub actions_triggered: usize,
    pub guard_hits: usize,
    pub duration_ms: u128,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEvent {
    pub step: usize,
    pub phase: String,
    pub message: String,
    pub elapsed_ms: u128,
    pub context_snapshot: Option<Value>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            context_data: Value::Object(serde_json::Map::new()),
        }
    }

    /// Memuat konteks dari file JSON (map key -> number)
    pub fn load_context_from_file(&mut self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| format!("cannot read context file: {}", e))?;
        let value: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| format!("invalid JSON context: {}", e))?;
        if !value.is_object() {
            return Err("context JSON must be an object".to_string());
        }
        self.context_data = value;
        Ok(())
    }

    pub fn context_data_as_string(&self) -> String {
        serde_json::to_string(&self.context_data).unwrap_or_else(|_| "{}".to_string())
    }

    pub fn context_snapshot(&self) -> Value {
        self.context_data.clone()
    }

    #[allow(dead_code)]
    /// Memasukkan data simulasi/sensor ke dalam runtime
    pub fn update_data(&mut self, key: &str, value: Value) {
        if let Some(obj) = self.context_data.as_object_mut() {
            obj.insert(key.to_string(), value);
        }
    }

    /// Mengeksekusi Kebijakan terhadap data saat ini
    /// Mengembalikan daftar aksi yang harus dilakukan (Triggered Actions)
    pub fn execute_policy(&self, policy: &Policy) -> Decision {
        let start = Instant::now();
        let mut ctx = self.context_data.clone();
        let mut trace_step: usize = 0;
        let mut decision = Decision {
            actions: Vec::new(),
            logs: Vec::new(),
            guard_triggered: false,
            metrics: DecisionMetrics::default(),
            traces: Vec::new(),
        };

        decision
            .logs
            .push(format_log(&format!("Context Data: {}", ctx)));
        Self::push_trace(
            &mut decision.traces,
            &mut trace_step,
            "start",
            "Context loaded".to_string(),
            &ctx,
            start.elapsed().as_millis(),
        );

        for rule in &policy.rules {
            self.execute_rule(rule, &mut ctx, &mut decision, &mut trace_step, &start);
        }

        decision.metrics.duration_ms = start.elapsed().as_millis();
        Self::push_trace(
            &mut decision.traces,
            &mut trace_step,
            "end",
            "Policy execution finished".to_string(),
            &ctx,
            decision.metrics.duration_ms,
        );
        decision
    }

    fn execute_rule(
        &self,
        rule: &crate::ast::Rule,
        data: &mut Value,
        decision: &mut Decision,
        trace_step: &mut usize,
        start: &Instant,
    ) {
        match rule {
            crate::ast::Rule::Standard(r) => {
                decision.metrics.rules_evaluated += 1;
                let is_triggered = evaluate_condition(&r.condition, data);

                decision
                    .logs
                    .push(format_log(&format!("Checking Rule: IF {} ...", r.condition)));
                Self::push_trace(
                    &mut decision.traces,
                    trace_step,
                    "rule",
                    format!("IF {}", r.condition),
                    data,
                    start.elapsed().as_millis(),
                );

                if is_triggered {
                    decision.metrics.actions_triggered += 1;
                    decision
                        .logs
                        .push(format_log(&format!("MATCH -> THEN {}", r.action)));
                    decision.actions.push(r.action.clone());
                    Self::push_trace(
                        &mut decision.traces,
                        trace_step,
                        "action",
                        format!("Trigger action: {}", r.action),
                        data,
                        start.elapsed().as_millis(),
                    );
                } else {
                    decision.logs.push(format_log("No match"));
                    Self::push_trace(
                        &mut decision.traces,
                        trace_step,
                        "rule",
                        "Condition not met".to_string(),
                        data,
                        start.elapsed().as_millis(),
                    );
                }
            }
            crate::ast::Rule::For(loop_data) => {
                decision.metrics.rules_evaluated += 1;
                decision.logs.push(format_log(&format!(
                    "[LOOP] FOR {} IN {}",
                    loop_data.iterator, loop_data.collection
                )));
                Self::push_trace(
                    &mut decision.traces,
                    trace_step,
                    "loop",
                    format!("FOR {} IN {}", loop_data.iterator, loop_data.collection),
                    data,
                    start.elapsed().as_millis(),
                );
                let start_time = Instant::now();
                let mut iter_count = 0usize;
                let elements = resolve_loop_elements(data, &loop_data.collection);
                for element in elements {
                    iter_count += 1;
                    if iter_count > MAX_LOOP_ITERATIONS {
                        decision.guard_triggered = true;
                        decision.metrics.guard_hits += 1;
                        decision.logs.push(format_log(&format!(
                            "Guard hit: loop iteration limit exceeded ({})",
                            MAX_LOOP_ITERATIONS
                        )));
                        Self::push_trace(
                            &mut decision.traces,
                            trace_step,
                            "guard",
                            "Loop iteration limit hit".to_string(),
                            data,
                            start.elapsed().as_millis(),
                        );
                        break;
                    }
                    if start_time.elapsed().as_millis() > MAX_LOOP_TIME_MS {
                        decision.guard_triggered = true;
                        decision.metrics.guard_hits += 1;
                        decision.logs.push(format_log(&format!(
                            "Guard hit: loop time exceeded ({} ms)",
                            MAX_LOOP_TIME_MS
                        )));
                        Self::push_trace(
                            &mut decision.traces,
                            trace_step,
                            "guard",
                            "Loop time limit hit".to_string(),
                            data,
                            start.elapsed().as_millis(),
                        );
                        break;
                    }

                    if let Some(obj) = data.as_object_mut() {
                        obj.insert(loop_data.iterator.clone(), element.clone());
                    }

                    decision
                        .logs
                        .push(format_log(&format!("Iteration {} for {}", iter_count, loop_data.iterator)));
                    Self::push_trace(
                        &mut decision.traces,
                        trace_step,
                        "loop-iter",
                        format!("Iter {} set {}", iter_count, loop_data.iterator),
                        data,
                        start.elapsed().as_millis(),
                    );
                    for sub_rule in &loop_data.body {
                        self.execute_rule(sub_rule, data, decision, trace_step, start);
                    }
                }
            }
            crate::ast::Rule::While(loop_data) => {
                decision.metrics.rules_evaluated += 1;
                decision
                    .logs
                    .push(format_log(&format!("[LOOP] WHILE {}", loop_data.condition)));
                Self::push_trace(
                    &mut decision.traces,
                    trace_step,
                    "loop",
                    format!("WHILE {}", loop_data.condition),
                    data,
                    start.elapsed().as_millis(),
                );
                let start_time = Instant::now();
                let mut iter_count = 0usize;
                
                // Guarded while loop: continue while condition holds but respect limits
                while evaluate_condition(&loop_data.condition, data) {
                    iter_count += 1;
                    if iter_count > MAX_LOOP_ITERATIONS {
                        decision.guard_triggered = true;
                        decision.metrics.guard_hits += 1;
                        decision.logs.push(format_log(&format!(
                            "Guard hit: WHILE iteration limit exceeded ({})",
                            MAX_LOOP_ITERATIONS
                        )));
                        Self::push_trace(
                            &mut decision.traces,
                            trace_step,
                            "guard",
                            "WHILE iteration limit hit".to_string(),
                            data,
                            start.elapsed().as_millis(),
                        );
                        break;
                    }
                    if start_time.elapsed().as_millis() > MAX_LOOP_TIME_MS {
                        decision.guard_triggered = true;
                        decision.metrics.guard_hits += 1;
                        decision.logs.push(format_log(&format!(
                            "Guard hit: WHILE time exceeded ({} ms)",
                            MAX_LOOP_TIME_MS
                        )));
                        Self::push_trace(
                            &mut decision.traces,
                            trace_step,
                            "guard",
                            "WHILE time limit hit".to_string(),
                            data,
                            start.elapsed().as_millis(),
                        );
                        break;
                    }
                    decision
                        .logs
                        .push(format_log(&format!("Condition met. Iteration {}", iter_count)));
                    Self::push_trace(
                        &mut decision.traces,
                        trace_step,
                        "loop-iter",
                        format!("WHILE iter {}", iter_count),
                        data,
                        start.elapsed().as_millis(),
                    );
                    for sub_rule in &loop_data.body {
                        self.execute_rule(sub_rule, data, decision, trace_step, start);
                    }
                }
            }
        }
    }

    fn push_trace(
        traces: &mut Vec<TraceEvent>,
        step: &mut usize,
        phase: &str,
        message: String,
        ctx: &Value,
        elapsed_ms: u128,
    ) {
        traces.push(TraceEvent {
            step: *step,
            phase: phase.to_string(),
            message,
            elapsed_ms,
            context_snapshot: Some(ctx.clone()),
        });
        *step += 1;
    }

    /// Jalankan aksi menggunakan OmniRoutine; worker menerima string aksi dan konteks Value.
    /// Mengembalikan hasil berurutan sesuai daftar aksi.
    pub fn execute_actions_with_routine<F>(
        &self,
        actions: &[String],
        max_parallel: usize,
        worker: F,
    ) -> Vec<ActionResult>
    where
        F: Fn(&str, &Value) -> Result<Value, OmniError> + Send + Sync,
    {
        if actions.is_empty() {
            return Vec::new();
        }
        let routine = OmniRoutine::new(max_parallel);
        let ctx = self.context_data.clone();
        let tasks: Vec<RoutineTask> = actions
            .iter()
            .map(|a| RoutineTask {
                name: a.clone(),
                payload: ctx.clone(),
            })
            .collect();
        let results = routine.run(tasks, |task| worker(&task.name, &task.payload));
        results
            .into_iter()
            .map(|r| match r.output {
                Ok(val) => ActionResult::Success {
                    output: Some(val),
                    elapsed_ms: r.elapsed_ms.map(|m| m as u64),
                },
                Err(err) => ActionResult::Failed {
                    error: err.to_string(),
                    elapsed_ms: r.elapsed_ms.map(|m| m as u64),
                },
            })
            .collect()
    }
}
