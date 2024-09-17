use std::mem::swap;

#[allow(dead_code)]
pub fn increasing_generic<T: PartialOrd>(
    mut input: impl Iterator<Item = T>,
) -> impl Iterator<Item = T> {
    let mut max_value: Option<T> = None;
    return std::iter::from_fn(move || {
        while let Some(current_value) = input.next() {
            let mut curr = Some(current_value);
            if max_value == None || curr > max_value {
                swap(&mut max_value, &mut curr);
                if curr.is_none() && max_value.is_some() {
                    continue;
                } else {
                    return curr;
                }
            }
        }
        let mut last = None;
        swap(&mut max_value, &mut last);
        return last;
    });
}

#[cfg(test)]
mod test {
    use crate::increasing::generic::*;
    #[test]
    fn increasing_generic_tarea() {
        let v = vec![1, 2, 4, 2, 1, 5, 0];
        let res: Vec<u32> = increasing_generic(v.into_iter()).collect();
        assert_eq!(res, vec![1, 2, 4, 5]);
    }

    #[test]
    fn increasing_generic_empty_iter() {
        let v = vec![];
        let res: Vec<u32> = increasing_generic(v.into_iter()).collect();
        assert_eq!(res, vec![]);
    }

    #[test]
    fn increasing_generic_last_max() {
        let v = vec![1, 2, 4, 2, 1, 5];
        let res: Vec<u32> = increasing_generic(v.into_iter()).collect();
        assert_eq!(res, vec![1, 2, 4, 5]);
    }
}
