use core::slice;

pub struct Vector<T> {
    arr: Vec<T>
}

impl<T> Vector<T>
    where T: Copy + Clone
{
    pub fn new(arr: &[T]) -> Vector<T> {
        Vector { arr: arr.to_owned() }
    }

    pub fn iter(&self) -> slice::Iter<T> {
        self.arr.iter()
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
}
