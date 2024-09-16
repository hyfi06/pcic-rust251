fn main() {}

fn incresing_u32(mut input: impl Iterator<Item = u32>) -> impl Iterator<Item = u32> {
    let mut max_value = 0;
    std::iter::from_fn(move || {
        while let Some(curr_val) = input.next() {
            if curr_val > max_value {
                max_value = curr_val;
                return Some(max_value);
            }
        }
        None
    })
}

fn incresing_generic<T: PartialOrd>(mut input: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let mut max_value = input.next();
    let mut first = true;
    std::iter::from_fn(move ||{
        if first {
            first = false;
            return max_value;
        } else {
            match max_value {
                Some(max_val) => {
                    while let Some(curr) = input.next() {
                        if curr > max_val {
                            max_value = Some(max_val);
                            return Some(curr);
                        }
                    }
                    return None;
                },
                None => return max_value,
            }
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_incresing_u32() {
        let v = vec![1, 2, 4, 2, 1, 5, 0];
        let res: Vec<u32> = incresing_u32(v.into_iter()).collect();
        assert_eq!(res, vec![1, 2, 4, 5]);
    }

    #[test]
    fn test_incresing_genertic() {
        let v = vec![1, 2, 4, 2, 1, 5, 0];
        let iter = incresing_generic(v.into_iter());
        for x in iter {
            println!("{x}")
        }
    }
}
