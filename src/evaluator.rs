use std::collections::HashMap;

/// Mengevaluasi kondisi string sederhana
/// Format didukung: "Variable Operator Value" (e.g., "Distance < 1", "Temperature > 100")
pub fn evaluate_condition(condition: &str, data: &HashMap<String, f64>) -> bool {
    // 1. Pecah string menjadi token
    let parts: Vec<&str> = condition.split_whitespace().collect();
    
    // Kita harapkan format: [Variable, Operator, Value]
    // Contoh: ["Distance", "<", "1m"]
    if parts.len() < 3 {
        // Fallback: Jika format salah, anggap false (aman fail-safe)
        eprintln!("Warning: Invalid condition format '{}'", condition);
        return false;
    }

    let var_name = parts[0];
    let op = parts[1];
    let value_str = parts[2];

    // 2. Ambil nilai Variable dari Data Context
    // Jika data tidak ada, kita asumsikan 0.0 atau return false. Mari return false untuk safety.
    let var_value = match data.get(var_name) {
        Some(v) => *v,
        None => {
            eprintln!("Warning: Variable type '{}' not found in context data", var_name);
            return false;
        }
    };

    // 3. Parse Value (Bersihkan unit seperti 'm', 'C', dll)
    let clean_value_str: String = value_str.chars()
        .filter(|c| c.is_digit(10) || *c == '.')
        .collect();
        
    let reference_value: f64 = match clean_value_str.parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Warning: Could not parse reference value '{}'", value_str);
            return false;
        }
    };

    // 4. Bandingkan
    match op {
        "<" => var_value < reference_value,
        ">" => var_value > reference_value,
        "<=" => var_value <= reference_value,
        ">=" => var_value >= reference_value,
        "==" => var_value == reference_value,
        "!=" => var_value != reference_value,
        _ => {
            eprintln!("Warning: Unknown operator '{}'", op);
            return false;
        }
    }
}
