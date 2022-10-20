use super::*;
use crate::mathicore as mtc;

#[test]
fn test_new()
{
    let arr = [1., 2., 3.];
    let v = new(&arr);
    for (vi, ai) in v.iter().zip(&arr) {
        assert_eq!(vi, ai);
    }
}

#[test]
fn test_from()
{
    let arr = vec![1., 2., 3.];
    let a = new(&arr);
    let b = from(arr);
    assert_eq!(a, b);
}

#[test]
fn test_full()
{
    let size = 5;
    let value = 17.33;
    let v = full(size, value);
    let cmp_res = compare_scalar(
        &v,
        value,
        |a, b| mtc::isclose(*a, *b, None)
    );
    assert!(all(&cmp_res));
    assert_eq!(v.size(), size);
}

#[test]
fn test_zeros()
{
    let size = 5;
    let zeros = zeros(size);
    assert!(is_zero(&zeros));
    assert!(zeros.is_zero());
}

#[test]
fn test_ones()
{
    let size = 7;
    let ones = ones(size);
    let cmp_res = compare_scalar(
        &ones,
        1.,
        |a, b| mtc::isclose(*a, *b, None)
    );
    assert!(all(&cmp_res));
}

#[test]
fn test_full_like()
{
    let v = new(&[-1., 2., 3.]);
    let value = -1.5;
    let new_v = full_like(&v, value);
    let cmp_res = compare_scalar(
        &new_v,
        value,
        |a, b| mtc::isclose(*a, *b, None)
    );
    assert!(all(&cmp_res));
    assert_eq!(new_v.size(), v.size());
}

#[test]
fn test_zeros_like()
{
    let v = new(&[-1., 2., 3.]);
    let new_v = zeros_like(&v);
    assert!(new_v.is_zero());
    assert_eq!(new_v.size(), v.size());
}

#[test]
fn test_ones_like()
{
    let v = new(&[-1., 2., 3.]);
    let new_v = ones_like(&v);
    let cmp_res = compare_scalar(
        &new_v,
        1.,
        |a, b| mtc::isclose(*a, *b, None)
    );
    assert!(all(&cmp_res));
    assert_eq!(new_v.size(), v.size());
}

#[test]
fn test_size()
{
    let v = new(&[1., 2., 3.]);
    assert_eq!(v.size(), 3);
}

#[test]
fn test_is_zero()
{
    let zeros = zeros(5);
    assert!(zeros.is_zero());
    let non_zeros = new(&[-3.5, -7., 0.5]);
    assert!(!non_zeros.is_zero());
}

#[test]
fn test_all()
{
    let mut flags = vec![true, true, true];
    assert!(all(&flags));
    flags[1] = false;
    assert!(!all(&flags));
}

#[test]
fn test_any()
{
    let mut flags = vec![false, true, false];
    assert!(any(&flags));
    flags[1] = false;
    assert!(!any(&flags));
}

#[test]
fn test_compare()
{
    let v1 = new(&[1., 2., -3.5]);
    let v2 = new(&[2., 5., -1.25]);
    let res = 
        compare(&v1, &v2, |a, b| a < b);
    assert!(all(&res));
    let res = 
        compare(&v1, &v2, |a, b| a == b);
    assert!(!all(&res));
    let res = 
        compare(&v1, &v2, |a, b| a > b);
    assert!(!all(&res));
}

#[test]
fn test_compare_all_less()
{
    let v1 = new(&[1., 2., -3.5]);
    let v2 = new(&[2., 5., -1.25]);
    let flags =
        compare(&v1, &v2, |a, b| a < b);
    assert!(all(&flags));
    let flags =
        compare(&v1, &v2, |a, b| a == b);
    assert!(!all(&flags));
    let flags =
        compare(&v1, &v2, |a, b| a > b);
    assert!(!all(&flags));
    let flags =
        compare(&v1, &v2, |a, b| a != b);
    assert!(all(&flags));
}

#[test]
fn test_compare_all_eq()
{
    let v1 = new(&[1., 2., -3.5]);
    let v2 = new(&[1., 2., -3.5]);
    let res =
        compare(&v1, &v2, |a, b| a == b);
    assert!(all(&res));
    let res =
        compare(&v1, &v2, |a, b| a < b);
    assert!(!all(&res));
    let res =
        compare(&v1, &v2, |a, b| a > b);
    assert!(!all(&res));
    let res =
        compare(&v1, &v2, |a, b| a != b);
    assert!(!all(&res));
}

#[test]
fn test_compare_all_greater()
{
    let v1 = new(&[1., 2., -3.5]);
    let v2 = new(&[0., 0., -5.7]);
    let res =
        compare(&v1, &v2, |a, b| a > b);
    assert!(all(&res));
    let res =
        compare(&v1, &v2, |a, b| a != b);
    assert!(all(&res));
    let res =
        compare(&v1, &v2, |a, b| a < b);
    assert!(!all(&res));
    let res =
        compare(&v1, &v2, |a, b| a == b);
    assert!(!all(&res));
}

#[test]
fn test_compare_all_ne()
{
    let v1 = new(&[1., 2., -3.5]);
    let v2 = new(&[0., -2.3, 5.7]);
    let res =
        compare(&v1, &v2, |a, b| a != b);
    assert!(all(&res));
    let res =
        compare(&v1, &v2, |a, b| a == b);
    assert!(!all(&res));
    let res =
        compare(&v1, &v2, |a, b| a > b);
    assert!(!all(&res));
    let res =
        compare(&v1, &v2, |a, b| a < b);
    assert!(!all(&res));
}

#[test]
fn test_compare_any()
{
    let v1 = new(&[1., 2., -3.5]);
    let v2 = new(&[0., 2., -5.7]);
    let res =
        compare(&v1, &v2, |a, b| a == b);
    assert!(any(&res));
    let res =
        compare(&v1, &v2, |a, b| a > b);
    assert!(any(&res));
    let res =
        compare(&v1, &v2, |a, b| a < b);
    assert!(!any(&res));
    let res =
        compare(&v1, &v2, |a, b| a != b);
    assert!(any(&res));
}

#[test]
fn test_compare_scalar_eq()
{
    // Part #1
    let val = 5.;
    let v = full(3, val);
    let cmp_res =
        compare_scalar(&v, val, |a, b| a == b);
    assert!(all(&cmp_res));
    let cmp_res =
        compare_scalar(&v, val, |a, b| a != b);
    assert!(!all(&cmp_res));
    // Part #2
    let arr = [1., 2., 3.];
    let v = new(&arr);
    let val = 5.;
    let cmp_res =
        compare_scalar(&v, val, |a, b| a < b);
    assert!(all(&cmp_res));
    let cmp_res =
        compare_scalar(&v, val, |a, b| a > b);
    assert!(!all(&cmp_res));
    let cmp_res =
        compare_scalar(&v, val, |a, b| a != b);
    assert!(all(&cmp_res));
}

#[test]
fn test_eq()
{
    let arr = [1., 2., 3.];
    let v1 = new(&arr);
    let v2 = new(&arr);
    assert!(v1 == v2);
    let v3 = new(&[1., -7.2, 2.4]);
    assert!(v3 != v1);
}

#[test]
fn test_dot()
{
    let a = new(&[1., 2., 3.]);
    let b = new(&[-1., 1., -2.]);
    let test_res = -5.;
    let res = dot(&a, &b).unwrap();
    assert!(mtc::isclose(res, test_res, None));
    let res = a.dot(&b).unwrap();
    assert!(mtc::isclose(res, test_res, None));
    // Square norm
    assert!(mtc::isclose(a.dot(&a).unwrap(), 14., None));
}

#[test]
fn test_invalid_dot()
{
    let a = new(&[1., 2., 3.]);
    let b = new(&[-1., 1.]);
    // Sizes of a and b are not equal to each other
    assert_ne!(a.size(), b.size());
    assert!(a.dot(&b).is_err());
}

#[test]
fn test_index()
{
    let a = new(&[1., 2., 3.]);
    assert_eq!(a[0], 1.);
    assert_eq!(a[1], 2.);
    assert_ne!(a[2], 0.);
}

#[test]
#[should_panic]
fn test_invalid_index()
{
    let a = new(&[1., 2., 3.]);
    assert_eq!(a[100], 0.);
}

#[test]
fn test_cross()
{
    let a = new(&[-1., 2., 0.]);
    let b = new(&[1., 2., 3.]);
    let test_res = new(&[6., 3., -4.]);
    let res = cross(&a, &b).unwrap();
    assert_eq!(res, test_res);
    let res = a.cross(&b).unwrap();
    assert_eq!(res, test_res);
}

#[test]
fn test_invalid_cross()
{
    let a = new(&[-1., 2.]);
    let b = new(&[1., 2.]);
    assert!(cross(&a, &b).is_err());
}

#[test]
fn test_add()
{
    let a = from(vec![1., 2., 3.]);
    let b = from(vec![2., 5., -1.5]);
    let test_res = from(vec![3., 7., 1.5]);
    assert_eq!(a + b, test_res);
}

#[test]
fn test_add_assign()
{
    let mut a = from(vec![1., 2., 3.]);
    let b = from(vec![2., 5., -1.5]);
    a += b;
    let test_res = from(vec![3., 7., 1.5]);
    assert_eq!(a, test_res);
}
