use core::slice;

#[derive(Debug)]
pub struct Vector {
    arr: Vec<f64>
}

impl Vector {
    pub fn new(arr: &[f64]) -> Vector {
        Vector { arr: arr.to_owned() }
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
