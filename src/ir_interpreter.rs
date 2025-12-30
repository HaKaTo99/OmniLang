use std::time::Instant;

use serde_json::Value;

use crate::evaluator::evaluate_condition;
use crate::ir::{PolicyIR, RuleIR};
use crate::runtime::{Decision, DecisionMetrics};

/// Execute a PolicyIR against a JSON context using the existing evaluator semantics.
pub fn execute_ir(policy_ir: &PolicyIR, context: Value) -> Decision {
    let start = Instant::now();
    let mut ctx = context;
    let mut decision = Decision {
        actions: Vec::new(),
        logs: Vec::new(),
        guard_triggered: false,
        metrics: DecisionMetrics::default(),
        traces: Vec::new(),
    };

    for rule in &policy_ir.rules {
        eval_rule(rule, &mut ctx, &mut decision);
    }

    decision.metrics.duration_ms = start.elapsed().as_millis();
    decision
}

fn eval_rule(rule: &RuleIR, ctx: &mut Value, decision: &mut Decision) {
    match rule {
        RuleIR::Standard(r) => {
            decision.metrics.rules_evaluated += 1;
            decision
                .logs
                .push(format!("Checking Rule: IF {} ...", r.condition));
            let triggered = evaluate_condition(&r.condition, ctx);
            if triggered {
                decision.logs.push(format!("MATCH -> THEN {}", r.action));
                decision.actions.push(r.action.clone());
                decision.metrics.actions_triggered += 1;
            } else {
                decision.logs.push("No match".to_string());
            }
        }
        RuleIR::For(loop_ir) => {
            decision.metrics.rules_evaluated += 1;
            decision
                .logs
                .push(format!("[LOOP] FOR {} IN {}", loop_ir.iterator, loop_ir.collection));
            let start_time = Instant::now();
            let mut iter_count: usize = 0;
            let elements = resolve_loop_elements(ctx, &loop_ir.collection);
            for element in elements {
                iter_count += 1;
                if iter_count > loop_ir.guard.max_iterations {
                    decision.guard_triggered = true;
                    decision.metrics.guard_hits += 1;
                    decision.logs.push(format!(
                        "Guard hit: loop iteration limit exceeded ({})",
                        loop_ir.guard.max_iterations
                    ));
                    break;
                }
                if start_time.elapsed().as_millis() > loop_ir.guard.max_time_ms {
                    decision.guard_triggered = true;
                    decision.metrics.guard_hits += 1;
                    decision.logs.push(format!(
                        "Guard hit: loop time exceeded ({} ms)",
                        loop_ir.guard.max_time_ms
                    ));
                    break;
                }

                if let Some(obj) = ctx.as_object_mut() {
                    obj.insert(loop_ir.iterator.clone(), element.clone());
                }

                decision
                    .logs
                    .push(format!("Iteration {} for {}", iter_count, loop_ir.iterator));
                for sub_rule in &loop_ir.body {
                    eval_rule(sub_rule, ctx, decision);
                }
            }
        }
        RuleIR::While(loop_ir) => {
            decision.metrics.rules_evaluated += 1;
            decision
                .logs
                .push(format!("[LOOP] WHILE {}", loop_ir.condition));
            let start_time = Instant::now();
            let mut iter_count: usize = 0;

            while evaluate_condition(&loop_ir.condition, ctx) {
                iter_count += 1;
                if iter_count > loop_ir.guard.max_iterations {
                    decision.guard_triggered = true;
                    decision.metrics.guard_hits += 1;
                    decision.logs.push(format!(
                        "Guard hit: WHILE iteration limit exceeded ({})",
                        loop_ir.guard.max_iterations
                    ));
                    break;
                }
                if start_time.elapsed().as_millis() > loop_ir.guard.max_time_ms {
                    decision.guard_triggered = true;
                    decision.metrics.guard_hits += 1;
                    decision.logs.push(format!(
                        "Guard hit: WHILE time exceeded ({} ms)",
                        loop_ir.guard.max_time_ms
                    ));
                    break;
                }
                decision
                    .logs
                    .push(format!("Condition met. Iteration {}", iter_count));
                for sub_rule in &loop_ir.body {
                    eval_rule(sub_rule, ctx, decision);
                }
            }
        }
        RuleIR::Match(match_ir) => {
            decision.metrics.rules_evaluated += 1;
            decision
                .logs
                .push(format!("[MATCH] {}", match_ir.scrutinee));
            
            for arm in &match_ir.arms {
                let condition = format!("{} == {}", match_ir.scrutinee, arm.pattern);
                if evaluate_condition(&condition, ctx) {
                    decision.logs.push(format!("-> Match arm: {} => {}", arm.pattern, arm.action));
                    decision.actions.push(arm.action.clone());
                    decision.metrics.actions_triggered += 1;
                    return;
                }
            }
            decision.logs.push("-> No match found".to_string());
        }
    }
}

fn resolve_loop_elements(data: &Value, collection_path: &str) -> Vec<Value> {
    if let Some(arr) = resolve_array_path(data, collection_path) {
        return arr.into_iter().take(50).collect();
    }
    vec![Value::Null, Value::Null, Value::Null]
}

fn resolve_array_path(root: &Value, path: &str) -> Option<Vec<Value>> {
    let mut current = root;
    for segment in path.split('.') {
        if segment.is_empty() {
            return None;
        }
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
