use core::slice;

#[derive(Debug)]
pub struct Vector {
    arr: Vec<f64>
}

impl Vector {
    pub fn new(arr: &[f64]) -> Vector {
        Vector { arr: arr.to_owned() }
    }

    pub fn full(size: &usize, value: &f64) -> Vector {
        let mut arr = Vec::new();
        arr.resize(*size, *value);
        Vector { arr }
    }

    pub fn zeros(size: &usize) -> Vector {
        Self::full(size, &0.)
    }

    pub fn ones(size: &usize) -> Vector {
        Self::full(size, &1.)
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
}

#[cfg(test)]
mod tests {
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
        let v = Vector::full(&size, &value);
        assert!(Vector::isclose(&v, value, None));
        assert_eq!(v.size(), size);
    }

    #[test]
    fn test_zeros() {
        let size = 5;
        let zv = Vector::zeros(&size);
        assert!(Vector::isclose(&zv, 0., None));
    }

    #[test]
    fn test_ones() {
        let size = 7;
        let ones = Vector::ones(&size);
        assert!(Vector::isclose(&ones, 1., None));
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
}
