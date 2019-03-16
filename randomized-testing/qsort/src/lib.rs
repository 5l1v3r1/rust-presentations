/// Fast sorting routine.
///
/// This sorts a slice of integers in ascending order.
///
/// # Examples
///
/// ```rust
/// # use qsort::qsort;
/// let mut d = [3, 2, 1];
/// qsort(&mut d);
/// assert_eq!([1, 2, 3], d);
/// ```
pub fn qsort(data: &mut [u8]) {
    if data.len() <= 1 {
        return;
    }

    let mut l = 0;
    let mut r = data.len() - 1;
    let pivot = data[(l + r) / 2];

    while l < r {
        if data[l] < pivot {
            l += 1;
        } else if data[r] > pivot {
            r -= 1;
        } else {
            data.swap(l, r);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_empty() {
        let mut d = [];
        qsort(&mut d);
        assert!(d.is_empty());
    }

    #[test]
    fn sort_singleton() {
        let mut d = [42];
        qsort(&mut d);
        assert_eq!([42], d);
    }

    #[test]
    fn sort_presorted() {
        let mut d = [1, 2, 3];
        qsort(&mut d);
        assert_eq!([1, 2, 3], d);
    }

    #[test]
    fn sort_reversed() {
        let mut d = [3, 2, 1];
        qsort(&mut d);
        assert_eq!([1, 2, 3], d);
    }

    #[test]
    fn sort_random() {
        let mut d = [4, 2, 3, 1];
        qsort(&mut d);
        assert_eq!([1, 2, 3, 4], d);
    }

    #[test]
    fn sort_random_long() {
        let mut d = [1, 8, 3, 7, 5, 6, 4, 2, 9];
        qsort(&mut d);
        assert_eq!([1, 2, 3, 4, 5, 6, 7, 8, 9], d);
    }
}
