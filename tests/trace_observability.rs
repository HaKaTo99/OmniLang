use omnilang_core::ast::Policy;
use omnilang_core::observability::{init_global_logger, set_global_trace, TraceId};
use omnilang_core::runtime::Runtime;

fn empty_policy() -> Policy {
    Policy {
        intent: None,
        actors: Vec::new(),
        context: None,
        assumptions: Vec::new(),
        rules: Vec::new(),
        constraints: Vec::new(),
        impacts: Vec::new(),
        traces: Vec::new(),
        reviews: Vec::new(),
    }
}

#[test]
fn decision_logs_are_trace_prefixed() {
    init_global_logger();
    set_global_trace(TraceId::new());

    let runtime = Runtime::new();
    let policy = empty_policy();
    let decision = runtime.execute_policy(&policy);

    assert!(
        !decision.logs.is_empty(),
        "expected runtime to emit at least one log entry"
    );
    assert!(
        decision
            .logs
            .iter()
            .all(|line| line.contains("[trace:")),
        "log lines should include trace id, got {:?}",
        decision.logs
    );
}
