/// Checks all float pointing numbers of the `v` vector
/// are equal to a scalar `value`.
/// 
/// You must give the `Option`-wrapped `tol` value (`Some(value)`)
/// to set tolerance value or `None` to set the default tolerance.
/// The tolerance parameter `tol` is 1e-6 by default.
/// 
/// # Example
/// 
/// ```
/// use mathi::mathicore as mtc;
/// 
/// let a = 19.1;
/// let b = 19.12;
/// // Default tolerance => a != b
/// assert!(!mtc::isclose(a, b, None));
/// // Low tolerance value => a == b
/// assert!(mtc::isclose(a, b, Some(0.1)));
/// // a == b and high tolerance
/// let a = 19.12;
/// assert!(mtc::isclose(a, b, None));
/// ```
pub fn isclose(a: f64, b: f64, tol: Option<f64>) -> bool
{
    let tol = tol.unwrap_or(1e-6);
    (a - b).abs() < tol
}
