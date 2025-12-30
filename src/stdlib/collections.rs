//! Collection helpers for simple map/filter/reduce over numbers and strings.

pub fn filter_numbers(values: &[f64], mut predicate: impl FnMut(f64) -> bool) -> Vec<f64> {
    values.iter().copied().filter(|v| predicate(*v)).collect()
}

pub fn map_numbers(values: &[f64], mapper: impl Fn(f64) -> f64) -> Vec<f64> {
    values.iter().cloned().map(mapper).collect()
}

pub fn reduce_numbers(values: &[f64], init: f64, reducer: impl Fn(f64, f64) -> f64) -> f64 {
    values.iter().cloned().fold(init, reducer)
}

pub fn filter_strings(values: &[String], mut predicate: impl FnMut(&str) -> bool) -> Vec<String> {
    values.iter().filter(|v| predicate(v)).cloned().collect()
}

pub fn map_strings(values: &[String], mapper: impl Fn(&str) -> String) -> Vec<String> {
    values.iter().map(|v| mapper(v)).collect()
}

pub fn reduce_strings(values: &[String], init: String, reducer: impl Fn(String, &str) -> String) -> String {
    values.iter().fold(init, |acc, v| reducer(acc, v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers_ops() {
        let v = [1.0, 2.0, 3.0];
        assert_eq!(filter_numbers(&v, |x| x > 1.5), vec![2.0, 3.0]);
        assert_eq!(map_numbers(&v, |x| x * 2.0), vec![2.0, 4.0, 6.0]);
        assert_eq!(reduce_numbers(&v, 0.0, |a, b| a + b), 6.0);
    }

    #[test]
    fn strings_ops() {
        let v = vec!["a".into(), "b".into(), "abc".into()];
        assert_eq!(filter_strings(&v, |s| s.contains('a')), vec!["a", "abc"]);
        assert_eq!(map_strings(&v, |s| s.to_uppercase()), vec!["A", "B", "ABC"]);
        assert_eq!(reduce_strings(&v, String::new(), |acc, s| acc + s), "ababc");
    }
}
