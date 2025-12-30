//! JSON path helpers (dot notation with array indices like a.b[0].c).

use serde_json::{Value};

pub fn get_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = value;
    for segment in parse_path(path)? {
        match segment {
            PathSeg::Key(k) => current = current.get(k)?,
            PathSeg::Index(i) => current = current.get(i)?,
        }
    }
    Some(current)
}

pub fn set_path(value: &mut Value, path: &str, new_value: Value) -> Result<(), String> {
    let mut current = value;
    let segments = parse_path(path).ok_or_else(|| "invalid path".to_string())?;
    if segments.is_empty() {
        return Err("empty path".to_string());
    }
    let last = segments.len() - 1;
    for seg in &segments[..last] {
        match seg {
            PathSeg::Key(k) => {
                let obj = current.as_object_mut().ok_or_else(|| "expected object".to_string())?;
                current = obj.entry(k.clone()).or_insert_with(|| Value::Object(Default::default()));
            }
            PathSeg::Index(i) => {
                let arr = current.as_array_mut().ok_or_else(|| "expected array".to_string())?;
                if *i >= arr.len() {
                    return Err("index out of bounds".to_string());
                }
                current = &mut arr[*i];
            }
        }
    }
    match &segments[last] {
        PathSeg::Key(k) => {
            let obj = current.as_object_mut().ok_or_else(|| "expected object".to_string())?;
            obj.insert(k.clone(), new_value);
            Ok(())
        }
        PathSeg::Index(i) => {
            let arr = current.as_array_mut().ok_or_else(|| "expected array".to_string())?;
            if *i >= arr.len() {
                return Err("index out of bounds".to_string());
            }
            arr[*i] = new_value;
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
enum PathSeg {
    Key(String),
    Index(usize),
}

fn parse_path(path: &str) -> Option<Vec<PathSeg>> {
    if path.is_empty() {
        return Some(vec![]);
    }
    let mut segments = Vec::new();
    for raw in path.split('.') {
        let mut rest = raw;
        if rest.is_empty() {
            return None;
        }
        // collect key before indices
        let mut key_end = rest.find('[').unwrap_or(rest.len());
        if key_end > 0 {
            segments.push(PathSeg::Key(rest[..key_end].to_string()));
        }
        while let Some(start) = rest[key_end..].find('[') {
            let global_start = key_end + start;
            let end = rest[global_start + 1..].find(']')? + global_start + 1;
            let idx_str = &rest[global_start + 1..end];
            let idx = idx_str.parse().ok()?;
            segments.push(PathSeg::Index(idx));
            rest = &rest[end + 1..];
            key_end = 0;
        }
    }
    Some(segments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn get_and_set_paths() {
        let mut v = json!({"a": {"b": [{"c": 1}, {"c": 2}]}});
        assert_eq!(get_path(&v, "a.b[1].c").unwrap(), &json!(2));
        set_path(&mut v, "a.b[0].c", json!(10)).unwrap();
        assert_eq!(get_path(&v, "a.b[0].c").unwrap(), &json!(10));
    }
}
