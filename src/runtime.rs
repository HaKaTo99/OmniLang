use crate::ast::Policy;
use crate::evaluator::evaluate_condition;
use std::collections::HashMap;

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
            // Evaluasi kondisi Rule
            let is_triggered = evaluate_condition(&rule.condition, &self.context_data);
            
            print!("Checking Rule: IF {} ... ", rule.condition);
            
            if is_triggered {
                println!("✅ MATCH! -> THEN {}", rule.action);
                triggered_actions.push(rule.action.clone());
            } else {
                println!("❌ (False)");
            }
        }
        println!("-------------------------");
        
        triggered_actions
    }
}
