use core::slice;

#[derive(Debug)]
pub struct Vector {
    arr: Vec<f64>
}

impl Vector {
    pub fn new(arr: &[f64]) -> Vector {
        Vector { arr: arr.to_owned() }
    }

    pub fn full(size: usize, value: f64) -> Vector {
        let mut arr = Vec::new();
        arr.resize(size, value);
        Vector { arr }
    }

    pub fn zeros(size: usize) -> Vector {
        Self::full(size, 0.)
    }

    pub fn ones(size: usize) -> Vector {
        Self::full(size, 1.)
    }

    pub fn full_like(v: &Vector, value: f64) -> Vector {
        let mut arr = Vec::new();
        arr.resize(v.size(), value);
        Vector { arr }
    }

    pub fn zeros_like(v: &Vector) -> Vector {
        Self::full_like(v, 0.)
    }

    pub fn ones_like(v: &Vector) -> Vector {
        Self::full_like(v, 1.)
    }

    pub fn size(&self) -> usize {
        self.arr.len()
    }

    pub fn iter(&self) -> slice::Iter<f64> {
        self.arr.iter()
    }

    pub fn isclose(v: &Vector, val: f64, tol: Option<f64>) -> bool {
        let tol = tol.unwrap_or(1e-6);
        for vi in v.iter() {
            if (*vi - val).abs() > tol {
                return false;
            }
        }
        true
    }

    pub fn is_zero(&self) -> bool {
        Self::isclose(self, 0., None)
    }

    pub fn all<F>(v1: &Vector, v2: &Vector, expr: F) -> bool
        where F: Fn(f64, f64) -> bool
    {
        for (a, b) in v1.arr.iter().zip(v2.arr.iter()) {
            if !expr(*a, *b) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod vector_tests {
    use super::*;

    #[test]
    fn test_new() {
        let arr = [1., 2., 3.];
        let v = Vector::new(&arr);
        for (vi, ai) in v.iter().zip(&arr) {
            assert_eq!(vi, ai);
        }
    }

    #[test]
    fn test_full() {
        let size = 5;
        let value = 17.33;
        let v = Vector::full(size, value);
        assert!(Vector::isclose(&v, value, None));
        assert_eq!(v.size(), size);
    }

    #[test]
    fn test_zeros() {
        let size = 5;
        let zv = Vector::zeros(size);
        assert!(Vector::isclose(&zv, 0., None));
    }

    #[test]
    fn test_ones() {
        let size = 7;
        let ones = Vector::ones(size);
        assert!(Vector::isclose(&ones, 1., None));
    }

    #[test]
    fn test_full_like() {
        let v = Vector::new(&[-1., 2., 3.]);
        let value = -1.5;
        let new_v = Vector::full_like(&v, value);
        assert!(Vector::isclose(&new_v, value, None));
        assert_eq!(new_v.size(), v.size());
    }

    #[test]
    fn test_zeros_like() {
        let v = Vector::new(&[-1., 2., 3.]);
        let new_v = Vector::zeros_like(&v);
        assert!(Vector::isclose(&new_v, 0., None));
        assert_eq!(new_v.size(), v.size());
    }

    #[test]
    fn test_ones_like() {
        let v = Vector::new(&[-1., 2., 3.]);
        let new_v = Vector::ones_like(&v);
        assert!(Vector::isclose(&new_v, 1., None));
        assert_eq!(new_v.size(), v.size());
    }

    #[test]
    fn test_size() {
        let v = Vector::new(&[1., 2., 3.]);
        assert_eq!(v.size(), 3);
    }

    #[test]
    fn test_isclose() {
        let val = 19.1;
        let v = Vector::new(&[1., 2., 3.]);
        assert!(!Vector::isclose(&v, val, None));
        let v = Vector::new(&[1., 2., 3. * (val / 3.)]);
        assert!(!Vector::isclose(&v, val, Some(1e-3)));
        let v = Vector::new(&[val, val, 3. * (val / 3.)]);
        assert!(Vector::isclose(&v, val, None));
    }

    #[test]
    fn test_is_zero() {
        let zeros = Vector::zeros(5);
        assert!(zeros.is_zero());
        let non_zeros = Vector::new(&[-3.5, -7., 0.5]);
        assert!(!non_zeros.is_zero());
    }

    #[test]
    fn test_all_less() {
        let v1 = Vector::new(&[1., 2., -3.5]);
        let v2 = Vector::new(&[2., 5., -1.25]);
        assert!(Vector::all(&v1, &v2, |a, b| a < b));   // true
        assert!(!Vector::all(&v1, &v2, |a, b| a == b));
        assert!(!Vector::all(&v1, &v2, |a, b| a > b));
    }

    #[test]
    fn test_all_eq() {
        let v1 = Vector::new(&[1., 2., -3.5]);
        let v2 = Vector::new(&[1., 2., -3.5]);
        assert!(Vector::all(&v1, &v2, |a, b| a == b));   // true
        assert!(!Vector::all(&v1, &v2, |a, b| a < b));
        assert!(!Vector::all(&v1, &v2, |a, b| a > b));
    }

    #[test]
    fn test_all_greater() {
        let v1 = Vector::new(&[1., 2., -3.5]);
        let v2 = Vector::new(&[0., 0., -5.7]);
        assert!(Vector::all(&v1, &v2, |a, b| a > b));   // true
        assert!(!Vector::all(&v1, &v2, |a, b| a == b));
        assert!(!Vector::all(&v1, &v2, |a, b| a < b));
    }

    #[test]
    fn test_all_ne() {
        let v1 = Vector::new(&[1., 2., -3.5]);
        let v2 = Vector::new(&[0., -2.3, 5.7]);
        assert!(Vector::all(&v1, &v2, |a, b| a != b));   // true
        assert!(!Vector::all(&v1, &v2, |a, b| a < b));
        assert!(!Vector::all(&v1, &v2, |a, b| a > b));
    }
}
