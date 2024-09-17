#[allow(dead_code)]
pub fn increasing_u32(input: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    let mut input = input;
    let mut max_value: Option<u32> = None;
    return std::iter::from_fn(move || {
        while let Some(curr) = input.next() {
            match max_value {
                Some(max) => {
                    if max < curr {
                        max_value = Some(curr);
                        return max_value;
                    }
                }
                None => {
                    max_value = Some(curr);
                    return max_value;
                }
            }
        }
        None
    });
}

#[cfg(test)]
mod test {
    use crate::increasing::u32::increasing_u32;

    #[test]
    fn increasing_u32_tarea() {
        let v = vec![1, 2, 4, 2, 1, 5, 0];
        let res: Vec<u32> = increasing_u32(v.into_iter()).collect();
        assert_eq!(res, vec![1, 2, 4, 5]);
    }

    #[test]
    fn increasing_u32_empty_iter() {
        let v = vec![];
        let res: Vec<u32> = increasing_u32(v.into_iter()).collect();
        assert_eq!(res, vec![]);
    }

    #[test]
    fn increasing_u32_last_max() {
        let v = vec![1, 2, 4, 2, 1, 5];
        let res: Vec<u32> = increasing_u32(v.into_iter()).collect();
        assert_eq!(res, vec![1, 2, 4, 5]);
    }
}
