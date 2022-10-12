use core::slice;

/// This is the math vector data struct.
#[derive(Debug)]
pub struct Vector {
    arr: Vec<f64>,
    size: usize
}

impl Vector {
    /// Creates a new instance of the ``Vector``
    /// using an initializing array ``arr``.
    pub fn new(arr: &[f64]) -> Vector {
        Vector { arr: arr.to_owned(), size: arr.len() }
    }

    /// Creates a vector with size ``size``
    /// and filled with the given ``value``.
    pub fn full(size: usize, value: f64) -> Vector {
        let mut arr = Vec::new();
        arr.resize(size, value);
        let size = arr.len();
        Vector { arr, size }
    }

    /// Creates a zeros-filled vector with size ``size``.
    pub fn zeros(size: usize) -> Vector {
        Self::full(size, 0.)
    }

    /// Creates a ones-filled vector with size ``size``.
    pub fn ones(size: usize) -> Vector {
        Self::full(size, 1.)
    }

    /// Creates a ``value``-filled vector with the same size
    /// as the given ``v`` vector.
    pub fn full_like(v: &Vector, value: f64) -> Vector {
        let mut arr = Vec::new();
        arr.resize(v.size(), value);
        let size = arr.len();
        Vector { arr, size }
    }

    /// Creates a zeros-filled vector with the same size
    /// as the given ``v`` vector.
    pub fn zeros_like(v: &Vector) -> Vector {
        Self::full_like(v, 0.)
    }

    /// Creates a ones-filled vector with the same size
    /// as the given ``v`` vector.
    pub fn ones_like(v: &Vector) -> Vector {
        Self::full_like(v, 1.)
    }

    /// Returns the vector size.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns the vector data iterator.
    pub fn iter(&self) -> slice::Iter<f64> {
        self.arr.iter()
    }

    /// Checks all float pointing numbers of the ``v`` vector
    /// are equal to a scalar ``value``.
    /// 
    /// You must give the ``Option``-wrapped ``tol`` value (``Some(value)``)
    /// to set tolerance value or ``None`` to set the default tolerance.
    /// The tolerance parameter ``tol`` is ``1e-6`` by default.
    pub fn isclose(v: &Vector, val: f64, tol: Option<f64>) -> bool {
        let tol = tol.unwrap_or(1e-6);
        for vi in v.iter() {
            if (*vi - val).abs() > tol {
                return false;
            }
        }
        true
    }

    /// Checks if a vector is a null vector.
    pub fn is_zero(&self) -> bool {
        Self::isclose(self, 0., None)
    }

    /// Compares two vectors ``v1`` and ``v2`` using pairs of their
    /// components for the ``condition`` closure.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use mathi::vector::Vector;
    /// 
    /// let init_arr = vec![1., 2., 3.];
    /// let v1 = Vector::new(&init_arr);
    /// let v2 = Vector::new(&init_arr);
    /// let cmp_res = Vector::compare(&v1, &v2, |a, b| a == b);
    /// assert!(Vector::all(&cmp_res));
    /// ```
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

    /// Returns ``true`` if all components of the ``v`` array are ``true``.
    /// 
    /// # Example
    /// 
    /// ```
    /// use mathi::vector::Vector;
    /// 
    /// let mut array = [true, true, true];
    /// assert!(Vector::all(&array));
    /// array[1] = false;
    /// assert!(!Vector::all(&array));
    /// ```
    pub fn all(v: &[bool]) -> bool
    {
        for flag in v {
            if !*flag {
                return false;
            }
        }
        true
    }
    
    /// Returns ``true`` if any of the ``v`` array components is ``true``.
    /// 
    /// # Example
    /// 
    /// ```
    /// use mathi::vector::Vector;
    /// 
    /// let mut array = [false, true, false];
    /// assert!(Vector::any(&array));
    /// array[1] = false;
    /// assert!(!Vector::any(&array));
    /// ```
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
