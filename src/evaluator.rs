use serde_json::Value;

#[derive(Debug, Clone, PartialEq)]
enum Atom {
    Number(f64),
    Bool(bool),
    Str(String),
}

fn parse_number_with_unit(raw: &str) -> Option<f64> {
    let lower = raw.to_lowercase();

    if lower.ends_with('%') {
        let num: String = lower
            .trim_end_matches('%')
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '.')
            .collect();
        return num.parse::<f64>().ok().map(|v| v / 100.0);
    }

    if lower.ends_with("km") {
        let num = lower.trim_end_matches("km");
        return num.parse::<f64>().ok().map(|v| v * 1000.0);
    }
    if lower.ends_with("cm") {
        let num = lower.trim_end_matches("cm");
        return num.parse::<f64>().ok().map(|v| v / 100.0);
    }
    if lower.ends_with("mm") {
        let num = lower.trim_end_matches("mm");
        return num.parse::<f64>().ok().map(|v| v / 1000.0);
    }
    if lower.ends_with('m') {
        let num = lower.trim_end_matches('m');
        return num.parse::<f64>().ok();
    }

    if lower.ends_with("ms") {
        let num = lower.trim_end_matches("ms");
        return num.parse::<f64>().ok().map(|v| v / 1000.0);
    }
    if lower.ends_with('s') {
        let num = lower.trim_end_matches('s');
        return num.parse::<f64>().ok();
    }
    if lower.ends_with('h') {
        let num = lower.trim_end_matches('h');
        return num.parse::<f64>().ok().map(|v| v * 3600.0);
    }

    let numeric_part: String = raw
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    numeric_part.parse().ok()
}

/// Evaluasi kondisi dengan dukungan OR/AND/NOT, dot-path, IN array, dan literal array JSON.
pub fn evaluate_condition(expr: &str, data: &Value) -> bool {
    eval_or(expr, data)
}

fn eval_or(expr: &str, data: &Value) -> bool {
    let parts = split_ci(expr, "OR");
    if parts.len() > 1 {
        return parts.iter().any(|p| eval_and(p, data));
    }
    eval_and(expr, data)
}

fn eval_and(expr: &str, data: &Value) -> bool {
    let parts = split_ci(expr, "AND");
    if parts.len() > 1 {
        return parts.iter().all(|p| eval_not(p, data));
    }
    eval_not(expr, data)
}

fn eval_not(expr: &str, data: &Value) -> bool {
    let trimmed = expr.trim();
    if trimmed.to_uppercase().starts_with("NOT ") {
        return !eval_simple(&trimmed[4..], data);
    }
    eval_simple(trimmed, data)
}

fn eval_simple(condition: &str, data: &Value) -> bool {
    let parts: Vec<&str> = condition.split_whitespace().collect();
    if parts.len() < 3 {
        eprintln!("Warning: Invalid condition format '{}'", condition);
        return false;
    }

    let var_path = parts[0];
    let op = parts[1];
    let value_str = parts[2];

    let left_atom = resolve_atom(var_path, data);
    if left_atom.is_none() {
        // Suppress noisy warnings for nested/indexed paths; keep warning for top-level misses
        if !(var_path.contains('.') || var_path.contains('[')) {
            eprintln!("Warning: Variable '{}' not found in context", var_path);
        }
        return false;
    }
    let left_atom = left_atom.unwrap();

    let ref_atom = resolve_reference_atom(value_str, data);
    let ref_array = resolve_array(value_str, data);

    match op {
        "<" | ">" | "<=" | ">=" | "==" | "=" | "!=" => {
            if let Some(atom) = ref_atom {
                compare_atoms(op, &left_atom, &atom)
            } else {
                eprintln!("Warning: Reference '{}' not found or unsupported", value_str);
                false
            }
        }
        "IN" | "in" => {
            if let Some(arr) = ref_array {
                arr.iter().any(|v| atom_eq_json(&left_atom, v))
            } else if let Some(atom) = ref_atom {
                compare_atoms("==", &left_atom, &atom)
            } else {
                eprintln!("Warning: IN expects array reference or literal, got '{}'", value_str);
                false
            }
        }
        _ => {
            eprintln!("Warning: Unknown operator '{}' in condition '{}'", op, condition);
            false
        }
    }
}

fn resolve_atom(path: &str, root: &Value) -> Option<Atom> {
    let v = resolve_path(root, path)?;
    to_atom(v)
}

fn resolve_reference_atom(raw: &str, root: &Value) -> Option<Atom> {
    let trimmed = raw.trim_matches('"');
    match trimmed.to_lowercase().as_str() {
        "true" => Some(Atom::Bool(true)),
        "false" => Some(Atom::Bool(false)),
        _ => {
            if let Some(num) = parse_number_with_unit(trimmed) {
                return Some(Atom::Number(num));
            }
            resolve_atom(raw, root).or_else(|| Some(Atom::Str(trimmed.to_string())))
        }
    }
}

fn resolve_array(path: &str, root: &Value) -> Option<Vec<Value>> {
    if let Some(arr) = parse_array_literal(path) {
        return Some(arr);
    }
    let v = resolve_path(root, path)?;
    v.as_array().cloned()
}

fn parse_array_literal(raw: &str) -> Option<Vec<Value>> {
    let t = raw.trim();
    if t.starts_with('[') && t.ends_with(']') {
        if let Ok(val) = serde_json::from_str::<Value>(t) {
            return val.as_array().cloned();
        }
    }
    None
}

fn resolve_path<'a>(root: &'a Value, path: &str) -> Option<&'a Value> {
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
    Some(current)
}

fn to_atom(v: &Value) -> Option<Atom> {
    if let Some(n) = v.as_f64() {
        return Some(Atom::Number(n));
    }
    if let Some(b) = v.as_bool() {
        return Some(Atom::Bool(b));
    }
    if let Some(s) = v.as_str() {
        if let Some(n) = parse_number_with_unit(s) {
            return Some(Atom::Number(n));
        }
        return Some(Atom::Str(s.to_string()));
    }
    None
}

fn compare_atoms(op: &str, left: &Atom, right: &Atom) -> bool {
    match (left, right) {
        (Atom::Number(l), Atom::Number(r)) => match op {
            "<" => l < r,
            ">" => l > r,
            "<=" => l <= r,
            ">=" => l >= r,
            "==" | "=" => l == r,
            "!=" => l != r,
            _ => false,
        },
        (Atom::Bool(l), Atom::Bool(r)) => match op {
            "==" | "=" => l == r,
            "!=" => l != r,
            _ => false,
        },
        (Atom::Str(l), Atom::Str(r)) => match op {
            "==" | "=" => l == r,
            "!=" => l != r,
            _ => false,
        },
        _ => false,
    }
}

fn atom_eq_json(atom: &Atom, v: &Value) -> bool {
    match (atom, v) {
        (Atom::Number(l), Value::Number(r)) => r.as_f64().map(|rv| *l == rv).unwrap_or(false),
        (Atom::Bool(l), Value::Bool(r)) => l == r,
        (Atom::Str(l), Value::String(r)) => l == r,
        _ => false,
    }
}

fn split_ci(expr: &str, sep: &str) -> Vec<String> {
    let mut chunks = Vec::new();
    let mut current = Vec::new();
    for tok in expr.split_whitespace() {
        if tok.eq_ignore_ascii_case(sep) {
            if !current.is_empty() {
                chunks.push(current.join(" "));
                current.clear();
            }
        } else {
            current.push(tok.to_string());
        }
    }
    if !current.is_empty() {
        chunks.push(current.join(" "));
    }
    chunks.retain(|s| !s.trim().is_empty());
    chunks
}
