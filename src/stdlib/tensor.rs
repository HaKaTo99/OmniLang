//! Minimal tensor helpers for vector dot and small matrix multiplication.

use crate::OmniError;

/// Dot product of two equal-length vectors.
pub fn dot(a: &[f64], b: &[f64]) -> Result<f64, OmniError> {
    if a.len() != b.len() {
        return Err(OmniError::InvalidInput("dot: dimension mismatch".to_string()));
    }
    Ok(a.iter().zip(b.iter()).map(|(x, y)| x * y).sum())
}

/// Matrix multiplication C = A x B for small dense matrices.
/// A is m x k, B is k x n.
pub fn matmul(a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, OmniError> {
    if a.is_empty() || b.is_empty() {
        return Ok(Vec::new());
    }
    let k = a[0].len();
    if a.iter().any(|row| row.len() != k) {
        return Err(OmniError::InvalidInput("matmul: ragged A".to_string()));
    }
    if b.iter().any(|row| row.len() != b[0].len()) {
        return Err(OmniError::InvalidInput("matmul: ragged B".to_string()));
    }
    if b.len() != k {
        return Err(OmniError::InvalidInput("matmul: inner dimension mismatch".to_string()));
    }
    let n = b[0].len();
    let m = a.len();

    let mut result = vec![vec![0.0; n]; m];
    for i in 0..m {
        for j in 0..n {
            let mut acc = 0.0;
            for t in 0..k {
                acc += a[i][t] * b[t][j];
            }
            result[i][j] = acc;
        }
    }
    Ok(result)
}

/// Transpose a matrix (rows -> columns).
pub fn transpose(a: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, OmniError> {
    if a.is_empty() {
        return Ok(Vec::new());
    }
    let cols = a[0].len();
    if a.iter().any(|row| row.len() != cols) {
        return Err(OmniError::InvalidInput("transpose: ragged matrix".to_string()));
    }
    let rows = a.len();
    let mut out = vec![vec![0.0; rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            out[c][r] = a[r][c];
        }
    }
    Ok(out)
}

/// Matrix-vector multiplication y = A x for A (m x n) and x (n).
pub fn matvec(a: &[Vec<f64>], x: &[f64]) -> Result<Vec<f64>, OmniError> {
    if a.is_empty() {
        return Ok(Vec::new());
    }
    let n = a[0].len();
    if x.len() != n {
        return Err(OmniError::InvalidInput("matvec: dimension mismatch".to_string()));
    }
    if a.iter().any(|row| row.len() != n) {
        return Err(OmniError::InvalidInput("matvec: ragged matrix".to_string()));
    }
    let m = a.len();
    let mut y = vec![0.0; m];
    for i in 0..m {
        let mut acc = 0.0;
        for j in 0..n {
            acc += a[i][j] * x[j];
        }
        y[i] = acc;
    }
    Ok(y)
}

/// L2 norm of a vector.
pub fn norm_l2(x: &[f64]) -> f64 {
    x.iter().map(|v| v * v).sum::<f64>().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product_matches_expected() {
        assert_eq!(dot(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]).unwrap(), 32.0);
        assert!(dot(&[1.0], &[]).is_err());
    }

    #[test]
    fn matmul_works_for_small_matrices() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]]; // 2x2
        let b = vec![vec![5.0, 6.0, 7.0], vec![8.0, 9.0, 10.0]]; // 2x3
        let expected = vec![vec![21.0, 24.0, 27.0], vec![47.0, 54.0, 61.0]];
        assert_eq!(matmul(&a, &b).unwrap(), expected);

        // Dimension mismatch
        let bad = matmul(&a, &vec![vec![1.0]]);
        assert!(bad.is_err());
    }

    #[test]
    fn transpose_and_matvec_and_norm() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]; // 2x3
        let t = transpose(&a).unwrap(); // 3x2
        assert_eq!(t, vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]);

        let x = vec![1.0, 1.0, 1.0];
        let y = matvec(&a, &x).unwrap();
        assert_eq!(y, vec![6.0, 15.0]);

        let n = norm_l2(&y);
        assert!((n - (6.0_f64.hypot(15.0))).abs() < 1e-9);
    }
}
