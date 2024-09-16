fn main() {}

fn incresing_u32(mut input: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    let mut max_value = 0;
    std::iter::from_fn(move || {
        while let Some(curr_val) = input.next() {
            if curr_val > max_value {
                max_value = curr_val.clone();
                return Some(curr_val);
            }
        }
        None
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_incresing_u32() {
        let v = vec![1, 2, 4, 2, 1, 5, 0];
        let res: Vec<u32> = incresing_u32(v.into_iter()).collect();
        assert_eq!(res, vec![1,2,4,5]);
    }
}
