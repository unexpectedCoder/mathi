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
