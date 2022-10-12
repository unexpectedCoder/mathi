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
    let cmp_res = Vector::compare_scalar(
        &v,
        &value,
        |a, b| Vector::isclose(a, b, None)
    );
    assert!(Vector::all(&cmp_res));
    assert_eq!(v.size(), size);
}

#[test]
fn test_zeros() {
    let size = 5;
    let zeros = Vector::zeros(size);
    assert!(zeros.is_zero());
}

#[test]
fn test_ones() {
    let size = 7;
    let ones = Vector::ones(size);
    let cmp_res = Vector::compare_scalar(
        &ones,
        &1.,
        |a, b| Vector::isclose(a, b, None)
    );
    assert!(Vector::all(&cmp_res));
}

#[test]
fn test_full_like() {
    let v = Vector::new(&[-1., 2., 3.]);
    let value = -1.5;
    let new_v = Vector::full_like(&v, value);
    let cmp_res = Vector::compare_scalar(
        &new_v,
        &value,
        |a, b| Vector::isclose(&a, &b, None)
    );
    assert!(Vector::all(&cmp_res));
    assert_eq!(new_v.size(), v.size());
}

#[test]
fn test_zeros_like() {
    let v = Vector::new(&[-1., 2., 3.]);
    let new_v = Vector::zeros_like(&v);
    assert!(new_v.is_zero());
    assert_eq!(new_v.size(), v.size());
}

#[test]
fn test_ones_like() {
    let v = Vector::new(&[-1., 2., 3.]);
    let new_v = Vector::ones_like(&v);
    let cmp_res = Vector::compare_scalar(
        &new_v,
        &1.,
        |a, b| Vector::isclose(&a, &b, None)
    );
    assert!(Vector::all(&cmp_res));
    assert_eq!(new_v.size(), v.size());
}

#[test]
fn test_size() {
    let v = Vector::new(&[1., 2., 3.]);
    assert_eq!(v.size(), 3);
}

#[test]
fn test_isclose() {
    let a = 19.1;
    let b = 19.12;
    assert!(!Vector::isclose(&a, &b, None));
    assert!(Vector::isclose(&a, &b, Some(0.1)));
    let a = 19.12;
    assert!(Vector::isclose(&a, &b, None));
}

#[test]
fn test_is_zero() {
    let zeros = Vector::zeros(5);
    assert!(zeros.is_zero());
    let non_zeros = Vector::new(&[-3.5, -7., 0.5]);
    assert!(!non_zeros.is_zero());
}

#[test]
fn test_all() {
    let mut flags = vec![true, true, true];
    assert!(Vector::all(&flags));
    flags[1] = false;
    assert!(!Vector::all(&flags));
}

#[test]
fn test_any() {
    let mut flags = vec![false, true, false];
    assert!(Vector::any(&flags));
    flags[1] = false;
    assert!(!Vector::any(&flags));
}

#[test]
fn test_compare() {
    let v1 = Vector::new(&[1., 2., -3.5]);
    let v2 = Vector::new(&[2., 5., -1.25]);
    let res = 
        Vector::compare(&v1, &v2, |a, b| a < b);
    assert!(Vector::all(&res));
    let res = 
        Vector::compare(&v1, &v2, |a, b| a == b);
    assert!(!Vector::all(&res));
    let res = 
        Vector::compare(&v1, &v2, |a, b| a > b);
    assert!(!Vector::all(&res));
}

#[test]
fn test_compare_all_less() {
    let v1 = Vector::new(&[1., 2., -3.5]);
    let v2 = Vector::new(&[2., 5., -1.25]);
    let flags =
        Vector::compare(&v1, &v2, |a, b| a < b);
    assert!(Vector::all(&flags));
    let flags =
        Vector::compare(&v1, &v2, |a, b| a == b);
    assert!(!Vector::all(&flags));
    let flags =
        Vector::compare(&v1, &v2, |a, b| a > b);
    assert!(!Vector::all(&flags));
    let flags =
        Vector::compare(&v1, &v2, |a, b| a != b);
    assert!(Vector::all(&flags));
}

#[test]
fn test_compare_all_eq() {
    let v1 = Vector::new(&[1., 2., -3.5]);
    let v2 = Vector::new(&[1., 2., -3.5]);
    let res =
        Vector::compare(&v1, &v2, |a, b| a == b);
    assert!(Vector::all(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a < b);
    assert!(!Vector::all(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a > b);
    assert!(!Vector::all(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a != b);
    assert!(!Vector::all(&res));
}

#[test]
fn test_compare_all_greater() {
    let v1 = Vector::new(&[1., 2., -3.5]);
    let v2 = Vector::new(&[0., 0., -5.7]);
    let res =
        Vector::compare(&v1, &v2, |a, b| a > b);
    assert!(Vector::all(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a != b);
    assert!(Vector::all(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a < b);
    assert!(!Vector::all(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a == b);
    assert!(!Vector::all(&res));
}

#[test]
fn test_compare_all_ne() {
    let v1 = Vector::new(&[1., 2., -3.5]);
    let v2 = Vector::new(&[0., -2.3, 5.7]);
    let res =
        Vector::compare(&v1, &v2, |a, b| a != b);
    assert!(Vector::all(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a == b);
    assert!(!Vector::all(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a > b);
    assert!(!Vector::all(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a < b);
    assert!(!Vector::all(&res));
}

#[test]
fn test_compare_any() {
    let v1 = Vector::new(&[1., 2., -3.5]);
    let v2 = Vector::new(&[0., 2., -5.7]);
    let res =
        Vector::compare(&v1, &v2, |a, b| a == b);
    assert!(Vector::any(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a > b);
    assert!(Vector::any(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a < b);
    assert!(!Vector::any(&res));
    let res =
        Vector::compare(&v1, &v2, |a, b| a != b);
    assert!(Vector::any(&res));
}

#[test]
fn test_compare_scalar_eq() {
    // Part #1
    let val = 5.;
    let v = Vector::full(3, val);
    let cmp_res =
        Vector::compare_scalar(&v, &val, |a, b| a == b);
    assert!(Vector::all(&cmp_res));
    let cmp_res =
        Vector::compare_scalar(&v, &val, |a, b| a != b);
    assert!(!Vector::all(&cmp_res));
    // Part #2
    let arr = [1., 2., 3.];
    let v = Vector::new(&arr);
    let val = 5.;
    let cmp_res =
        Vector::compare_scalar(&v, &val, |a, b| a < b);
    assert!(Vector::all(&cmp_res));
    let cmp_res =
        Vector::compare_scalar(&v, &val, |a, b| a > b);
    assert!(!Vector::all(&cmp_res));
    let cmp_res =
        Vector::compare_scalar(&v, &val, |a, b| a != b);
    assert!(Vector::all(&cmp_res));
}

#[test]
fn test_eq() {
    let arr = [1., 2., 3.];
    let v1 = Vector::new(&arr);
    let v2 = Vector::new(&arr);
    assert!(v1 == v2);
    let v3 = Vector::new(&[1., -7.2, 2.4]);
    assert!(v3 != v1);
}
