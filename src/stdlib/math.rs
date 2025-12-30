//! Math helpers for policy evaluation.

/// Clamp a value between min and max (inclusive).
pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Round a float to the nearest integer (banker’s rounding from f64::round).
pub fn round(value: f64) -> f64 {
    value.round()
}

/// Average of a slice; returns 0.0 for empty input.
pub fn avg(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    let sum: f64 = values.iter().sum();
    sum / values.len() as f64
}

pub fn min(values: &[f64]) -> f64 {
    values.iter().cloned().fold(f64::INFINITY, f64::min)
}

pub fn max(values: &[f64]) -> f64 {
    values.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
}

pub fn sum(values: &[f64]) -> f64 {
    values.iter().sum()
}

pub fn stddev(values: &[f64]) -> f64 {
    if values.len() < 2 {
        return 0.0;
    }
    let mean = avg(values);
    let var = values
        .iter()
        .map(|v| (v - mean).powi(2))
        .sum::<f64>()
        / (values.len() as f64);
    var.sqrt()
}

pub fn abs(v: f64) -> f64 {
    v.abs()
}

pub fn pow(base: f64, exp: f64) -> f64 {
    base.powf(exp)
}

pub fn sqrt(v: f64) -> f64 {
    v.sqrt()
}

pub fn ceil(v: f64) -> f64 {
    v.ceil()
}

pub fn floor(v: f64) -> f64 {
    v.floor()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clamp_bounds() {
        assert_eq!(clamp(5.0, 0.0, 10.0), 5.0);
        assert_eq!(clamp(-1.0, 0.0, 10.0), 0.0);
        assert_eq!(clamp(11.0, 0.0, 10.0), 10.0);
    }

    #[test]
    fn round_basic() {
        assert_eq!(round(1.2), 1.0);
        assert_eq!(round(1.5), 2.0);
    }

    #[test]
    fn avg_empty_and_values() {
        assert_eq!(avg(&[]), 0.0);
        assert!((avg(&[1.0, 2.0, 3.0]) - 2.0).abs() < 1e-9);
    }

    #[test]
    fn min_max_sum_stddev() {
        let vals = [1.0, 2.0, 3.0];
        assert_eq!(min(&vals), 1.0);
        assert_eq!(max(&vals), 3.0);
        assert_eq!(sum(&vals), 6.0);
        assert!((stddev(&vals) - 0.8164965809).abs() < 1e-6);
    }
}
