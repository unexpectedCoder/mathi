use core::slice;
use std::ops::Index;

use crate::mathicore as mtc;


/// This is the math vector struct.
#[derive(Debug)]
pub struct Vector
{
    arr: Vec<f64>,
    size: usize
}

/// Creates a new instance of the `Vector`
/// using an initializing array `arr`.
pub fn new(arr: &[f64]) -> Vector
{
    Vector { arr: arr.to_owned(), size: arr.len() }
}

/// Creates a new instance of the `Vector`
/// using an initializing array `arr` **that is moved**.
pub fn from(arr: Vec<f64>) -> Vector
{
    let size = arr.len();
    Vector { arr, size }
}

/// Creates a vector with size `size`
/// and filled with the given `value`.
pub fn full(size: usize, value: f64) -> Vector
{
    let mut arr = Vec::new();
    arr.resize(size, value);
    let size = arr.len();
    Vector { arr, size }
}

/// Creates a zeros-filled vector with size `size`.
pub fn zeros(size: usize) -> Vector
{
    full(size, 0.)
}

/// Creates a ones-filled vector with size `size`.
pub fn ones(size: usize) -> Vector
{
    full(size, 1.)
}

/// Creates a `value`-filled vector with the same size
/// as the given `v` vector.
pub fn full_like(v: &Vector, value: f64) -> Vector
{
    let mut arr = Vec::new();
    arr.resize(v.size(), value);
    let size = arr.len();
    Vector { arr, size }
}

/// Creates a zeros-filled vector with the same size
/// as the given `v` vector.
pub fn zeros_like(v: &Vector) -> Vector
{
    full_like(v, 0.)
}

/// Creates a ones-filled vector with the same size
/// as the given `v` vector.
pub fn ones_like(v: &Vector) -> Vector
{
    full_like(v, 1.)
}

/// Compares a vector `v` components with a scalar `val`
/// according to a `condition` closure.
/// Returns an array of booleans.
/// 
/// # Example
/// 
/// ```
/// use mathi::vector;
/// 
/// let init_arr = vec![1., 2., 3.];
/// let v = vector::new(&init_arr);
/// let val = 5.;
/// let cmp_res = vector::compare_scalar(&v, val, |a, b| a < b);
/// assert!(vector::all(&cmp_res));
/// let cmp_res = vector::compare_scalar(&v, val, |a, b| a == b);
/// assert!(!vector::all(&cmp_res));
/// ```
pub fn compare_scalar<F>(v: &Vector, val: f64, condition: F) -> Vec<bool>
where
    F: Fn(&f64, &f64) -> bool
{
    let mut res = Vec::new();
    res.resize(v.size(), false);
    for (i, vi) in v.iter().enumerate() {
        res[i] = condition(vi, &val);
    }
    res
}

/// Compares two vectors `v1` and `v2` using pairs of their
/// components for the `condition` closure.
/// Returns an array of booleans.
/// 
/// # Example
/// 
/// ```
/// use mathi::vector;
/// 
/// let init_arr = vec![1., 2., 3.];
/// let v1 = vector::new(&init_arr);
/// let v2 = vector::new(&init_arr);
/// let cmp_res = vector::compare(&v1, &v2, |a, b| a == b);
/// assert!(vector::all(&cmp_res));
/// ```
pub fn compare<F>(v1: &Vector, v2: &Vector, condition: F) -> Vec<bool>
where
    F: Fn(&f64, &f64) -> bool
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

/// Returns `true` if all components of the `v` array are `true`.
/// 
/// # Example
/// 
/// ```
/// use mathi::vector;
/// 
/// let mut array = [true, true, true];
/// assert!(vector::all(&array));
/// array[1] = false;
/// assert!(!vector::all(&array));
/// ```
pub fn all(v: &[bool]) -> bool
{
    v.iter().all(|&flag| flag)
}

/// Returns `true` if any of the `v` array components is `true`.
/// 
/// # Example
/// 
/// ```
/// use mathi::vector;
/// 
/// let mut array = [false, true, false];
/// assert!(vector::any(&array));
/// array[1] = false;
/// assert!(!vector::any(&array));
/// ```
pub fn any(v: &[bool]) -> bool
{
    v.iter().any(|&flag| flag)
}

/// Checks if a vector is a null vector.
pub fn is_zero(v: &Vector) -> bool
{
    let res = compare_scalar(
        v,
        0.,
        |a, b| mtc::isclose(*a, *b, None)
    );
    all(&res)
}

/// Calculates the dot product of two vectors.
/// 
/// # Errors
/// 
/// Returns an `Err` instance if sizes of the vectors are not equal.
/// 
/// # Example
/// 
/// ```
/// use mathi::vector;
/// use mathi::mathicore as mtc;
/// 
/// let a = vector::new(&[1., 2., 3.]);
/// let b = vector::new(&[-1., 1., -2.]);
/// let res = vector::dot(&a, &b).unwrap();
/// assert!(mtc::isclose(res, -5., None));
/// ```
/// 
/// Math is [here](https://en.wikipedia.org/wiki/Dot_product).
pub fn dot(a: &Vector, b: &Vector) -> Result<f64, String>
{
    if a.size() != b.size() {
        return Err(
            String::from("Vectors sizes must be equal to each other for the dot")
        );
    }
    let mut res = 0.;
    for (ai, bi) in a.iter().zip(b.iter()) {
        res += ai * bi;
    }
    Ok(res)
}

/// Calculates the cross product of two 3D vectors.
/// 
/// # Errors
/// 
/// Returns an `Err` instance when
/// 1. sizes of the vectors are not equal to 3 or
/// 2. sizes of the vectors are not equal to each other.
/// 
/// # Example
/// 
/// ```
/// use mathi::vector;
/// 
/// let a = vector::new(&[-1., 2., 0.]);
/// let b = vector::new(&[1., 2., 3.]);
/// let test_res = vector::new(&[6., 3., -4.]);
/// let res = vector::cross(&a, &b).unwrap();
/// assert_eq!(res, test_res);
/// ```
/// 
/// See math [here](https://en.wikipedia.org/wiki/Cross_product).
pub fn cross(a: &Vector, b: &Vector) -> Result<Vector, String>
{
    if a.size() != 3 || a.size() != b.size() {
        return Err(
            String::from("Invalid vectors sizes for the cross product")
        );
    }
    let s1 = a[1] * b[2] - a[2] * b[1];
    let s2 = a[2] * b[0] - a[0] * b[2];
    let s3 = a[0] * b[1] - a[1] * b[0];
    Ok(Vector { arr: vec![s1, s2, s3], size: 3 })
}

impl Vector {
    /// Returns the vector size.
    pub fn size(&self) -> usize
    {
        self.size
    }

    /// Returns the vector data iterator.
    pub fn iter(&self) -> slice::Iter<f64>
    {
        self.arr.iter()
    }

    /// See [is_zero](is_zero).
    pub fn is_zero(&self) -> bool
    {
        is_zero(self)
    }

    /// See [compare](compare).
    pub fn compare<F>(&self, other: &Vector, condition: F) -> Vec<bool>
    where
        F: Fn(&f64, &f64) -> bool
    {
        compare(self, other, condition)
    }

    /// See documentation [compare_scalar](compare_scalar).
    pub fn compare_scalar<F>(&self, val: f64, condition: F) -> Vec<bool>
    where
        F: Fn(&f64, &f64) -> bool
    {
        compare_scalar(self, val, condition)
    }

    /// See [dot](dot).
    pub fn dot(&self, other: &Vector) -> Result<f64, String>
    {
        dot(self, other)
    }

    /// See [cross](cross).
    pub fn cross(&self, other: &Vector) -> Result<Self, String>
    {
        cross(self, other)
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        let v = compare(
            self,
            other,
            |a, b| mtc::isclose(*a, *b, None)
        );
        all(&v)
    }
}

impl Eq for Vector {}

impl Index<usize> for Vector
{
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output
    {
        &self.arr[i]
    }
}

#[cfg(test)]
mod tests;
