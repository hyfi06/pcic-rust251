fn main() {}

fn mysort<T: Ord + Copy>(arr: &mut [T]) {
    merge_sort(arr);
    arr.to_vec();
}

fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    let mut merge_arr: Vec<T> = arr.to_vec();
    merge(&arr[..mid], &arr[mid..], &mut merge_arr);
    arr.copy_from_slice(&merge_arr);
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T], merge_arr: &mut [T]) {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut left_item = left_iter.next();
    let mut right_item = right_iter.next();
    for i in merge_arr.iter_mut() {
        match (left_item, right_item) {
            (Some(l), Some(r)) => {
                if l <= r {
                    *i = *l;
                    left_item = left_iter.next();
                } else {
                    *i = *r;
                    right_item = right_iter.next();
                }
            }
            (Some(l), None) => {
                *i = *l;
                left_item = left_iter.next();
            }
            (None, Some(r)) => {
                *i = *r;
                right_item = right_iter.next();
            }
            (None, None) => break,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_merge() {
        let l = [1, 3, 5, 6];
        let r = [2, 4, 7];
        let mut c = [0; 7];
        merge(&l, &r, &mut c);
        println!("{:?}", c);
        assert_eq!(c, [1, 2, 3, 4, 5, 6, 7])
    }

    #[test]
    fn test_merge_sort() {
        let mut a = [3, 4, 5, 6, 1, 4, 5, 1, 6];
        merge_sort(&mut a);
        println!("{:?}", a);
        assert_eq!(a, [1, 1, 3, 4, 4, 5, 5, 6, 6])
    }
}
