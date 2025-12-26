use crate::ast::Policy;
use crate::evaluator::evaluate_condition;
use std::collections::HashMap;
use std::time::Instant;

// Safety guards for loop execution
const MAX_LOOP_ITERATIONS: usize = 10_000; // Upper bound to avoid infinite loops
const MAX_LOOP_TIME_MS: u128 = 1_000; // 1 second max per loop

pub struct Runtime {
    context_data: HashMap<String, f64>,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            context_data: HashMap::new(),
        }
    }

    /// Memasukkan data simulasi/sensor ke dalam runtime
    pub fn update_data(&mut self, key: &str, value: f64) {
        self.context_data.insert(key.to_string(), value);
    }

    /// Mengeksekusi Kebijakan terhadap data saat ini
    /// Mengembalikan daftar aksi yang harus dilakukan (Triggered Actions)
    pub fn execute_policy(&self, policy: &Policy) -> Vec<String> {
        let mut triggered_actions = Vec::new();

        println!("--- RUNTIME EXECUTION ---");
        println!("Current Data Context: {:?}", self.context_data);

        for rule in &policy.rules {
            self.execute_rule(rule, &self.context_data, &mut triggered_actions);
        }
        println!("-------------------------");
        
        triggered_actions
    }

    fn execute_rule(&self, rule: &crate::ast::Rule, data: &HashMap<String, f64>, triggered_actions: &mut Vec<String>) {
        match rule {
            crate::ast::Rule::Standard(r) => {
                let is_triggered = evaluate_condition(&r.condition, data);
                
                print!("Checking Rule: IF {} ... ", r.condition);
                
                if is_triggered {
                    println!("✅ MATCH! -> THEN {}", r.action);
                    triggered_actions.push(r.action.clone());
                } else {
                    println!("❌ (False)");
                }
            }
            crate::ast::Rule::For(loop_data) => {
                println!("  [LOOP] Starting FOR loop: {} IN {}", loop_data.iterator, loop_data.collection);
                // Guarded iteration: simulate over collection size if known, otherwise use a safe default
                let start_time = Instant::now();
                let mut iter_count = 0usize;
                // For now we simulate a fixed number of iterations (e.g., 3) but enforce limits
                for i in 1..=3 {
                    iter_count += 1;
                    if iter_count > MAX_LOOP_ITERATIONS {
                        println!("⚠️ Loop iteration limit exceeded ({}). Aborting loop.", MAX_LOOP_ITERATIONS);
                        break;
                    }
                    if start_time.elapsed().as_millis() > MAX_LOOP_TIME_MS {
                        println!("⚠️ Loop execution time exceeded ({} ms). Aborting loop.", MAX_LOOP_TIME_MS);
                        break;
                    }
                    println!("    > Iteration {} for {}", i, loop_data.iterator);
                    for sub_rule in &loop_data.body {
                        self.execute_rule(sub_rule, data, triggered_actions);
                    }
                }
            }
            crate::ast::Rule::While(loop_data) => {
                println!("  [LOOP] Starting WHILE loop: {}", loop_data.condition);
                let start_time = Instant::now();
                let mut iter_count = 0usize;
                // Guarded while loop: continue while condition holds but respect limits
                while evaluate_condition(&loop_data.condition, data) {
                    iter_count += 1;
                    if iter_count > MAX_LOOP_ITERATIONS {
                        println!("⚠️ WHILE loop iteration limit exceeded ({}). Breaking.", MAX_LOOP_ITERATIONS);
                        break;
                    }
                    if start_time.elapsed().as_millis() > MAX_LOOP_TIME_MS {
                        println!("⚠️ WHILE loop execution time exceeded ({} ms). Breaking.", MAX_LOOP_TIME_MS);
                        break;
                    }
                    println!("    > Condition met. Executing body iteration {}", iter_count);
                    for sub_rule in &loop_data.body {
                        self.execute_rule(sub_rule, data, triggered_actions);
                    }
                }
            }
        }
    }
}
