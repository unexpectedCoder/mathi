use core::slice;

#[derive(Debug)]
pub struct Vector {
    arr: Vec<f64>,
    size: usize
}

impl Vector {
    pub fn new(arr: &[f64]) -> Vector {
        Vector { arr: arr.to_owned(), size: arr.len() }
    }

    pub fn full(size: usize, value: f64) -> Vector {
        let mut arr = Vec::new();
        arr.resize(size, value);
        let size = arr.len();
        Vector { arr, size }
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
        let size = arr.len();
        Vector { arr, size }
    }

    pub fn zeros_like(v: &Vector) -> Vector {
        Self::full_like(v, 0.)
    }

    pub fn ones_like(v: &Vector) -> Vector {
        Self::full_like(v, 1.)
    }

    pub fn size(&self) -> usize {
        self.size
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

    pub fn compare<F>(v1: &Vector, v2: &Vector, condition: F) -> Vec<bool>
        where F: Fn(&f64, &f64) -> bool
    {
        let mut res = Vec::new();
        res.resize(v1.size(), false);
        for (i, (ai, bi))
            in v1.arr.iter().zip(v2.arr.iter()).enumerate()
        {
            res[i] = condition(ai, bi);
        }
        res
    }

    pub fn all(v: &[bool]) -> bool
    {
        for flag in v {
            if !*flag {
                return false;
            }
        }
        true
    }

    pub fn any(v: &[bool]) -> bool
    {
        for flag in v {
            if *flag {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests;
