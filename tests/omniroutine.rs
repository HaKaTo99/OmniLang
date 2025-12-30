use omnilang_core::{OmniError, OmniRoutine, RoutineTask};
use serde_json::json;

#[test]
fn run_tasks_preserves_order_and_runs_in_parallel_caps() {
    let routine = OmniRoutine::new(2);
    let tasks = vec![
        RoutineTask { name: "a".into(), payload: json!({"v":1}) },
        RoutineTask { name: "b".into(), payload: json!({"v":2}) },
        RoutineTask { name: "c".into(), payload: json!({"v":3}) },
    ];

    let results = routine.run(tasks, |task| {
        let v = task.payload["v"].as_i64().unwrap_or(0);
        Ok(json!({"out": v * 2}))
    });

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].name, "a");
    assert_eq!(results[1].name, "b");
    assert_eq!(results[2].name, "c");
    assert_eq!(results[0].output.as_ref().unwrap()["out"], 2);
    assert_eq!(results[2].output.as_ref().unwrap()["out"], 6);
}

#[test]
fn run_tasks_propagates_errors() {
    let routine = OmniRoutine::new(2);
    let tasks = vec![
        RoutineTask { name: "ok".into(), payload: json!({"v":1}) },
        RoutineTask { name: "fail".into(), payload: json!({"v":-1}) },
    ];

    let results = routine.run(tasks, |task| {
        let v = task.payload["v"].as_i64().unwrap_or(0);
        if v < 0 {
            Err(OmniError::InvalidInput("negative".into()))
        } else {
            Ok(json!({"out": v}))
        }
    });

    assert_eq!(results.len(), 2);
    assert!(results[0].output.is_ok());
    assert!(results[1].output.is_err());
}
