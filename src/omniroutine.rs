use crate::error::OmniError;
use serde_json::Value;
use std::time::Instant;
use std::thread;

#[derive(Debug, Clone)]
pub struct RoutineTask {
    pub name: String,
    pub payload: Value,
}

#[derive(Debug)]
pub struct RoutineResult {
    #[allow(dead_code)]
    pub name: String,
    pub output: Result<Value, OmniError>,
    pub elapsed_ms: Option<u128>,
}

/// Scheduler ringan untuk eksekusi paralel terbatas.
pub struct OmniRoutine {
    max_parallel: usize,
}

impl OmniRoutine {
    /// Buat scheduler dengan batas paralel minimal 1.
    pub fn new(max_parallel: usize) -> Self {
        let limit = if max_parallel == 0 { 1 } else { max_parallel };
        OmniRoutine {
            max_parallel: limit,
        }
    }

    /// Jalankan tugas dengan worker; menjaga urutan hasil sesuai input.
    pub fn run<F>(&self, tasks: Vec<RoutineTask>, worker: F) -> Vec<RoutineResult>
    where
        F: Fn(&RoutineTask) -> Result<Value, OmniError> + Send + Sync,
    {
        if tasks.is_empty() {
            return Vec::new();
        }
        let maxp = self.max_parallel;
        let worker = &worker;
        let mut results: Vec<RoutineResult> = Vec::with_capacity(tasks.len());
        let mut idx = 0;
        while idx < tasks.len() {
            let end = (idx + maxp).min(tasks.len());
            let slice = &tasks[idx..end];
            let mut batch_results: Vec<(usize, RoutineResult)> = Vec::with_capacity(slice.len());
            thread::scope(|scope| {
                let mut handles = Vec::with_capacity(slice.len());
                for task in slice.iter().cloned() {
                    handles.push(scope.spawn(move || {
                        let start = Instant::now();
                        let out = worker(&task);
                        RoutineResult {
                            name: task.name.clone(),
                            output: out,
                            elapsed_ms: Some(start.elapsed().as_millis()),
                        }
                    }));
                    // store order alongside handle by position in handles vector
                }
                for (offset, handle) in handles.into_iter().enumerate() {
                    let order = idx + offset;
                    let res = handle.join().unwrap_or_else(|_| RoutineResult {
                        name: "panic".to_string(),
                        output: Err(OmniError::InvalidInput("task panicked".to_string())),
                        elapsed_ms: None,
                    });
                    batch_results.push((order, res));
                }
            });
            batch_results.sort_by_key(|(order, _)| *order);
            for (_, r) in batch_results {
                results.push(r);
            }
            idx = end;
        }
        results
    }
}
