use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (file_path, order) = parse_args(&args);
    let text = read_file(file_path);
    let frequencies: HashMap<&str, u32> = word_count(&text);
    match order {
        "w" => {
            let mut sorted: Vec<(&str, &u32)> = frequencies
                .iter()
                .map(|(key, value)| (*key, value))
                .collect();
            merge_sort(&mut sorted);
            sorted
                .iter()
                .for_each(|(key, &value)| println!("{key}: {value}"));
        }
        &_ => {
            let mut sorted: Vec<(&u32, &str)> = frequencies
                .iter()
                .map(|(key, value)| (value, *key))
                .collect();
            merge_sort(&mut sorted);
            sorted
                .iter()
                .for_each(|(&value, key)| println!("{key}: {value}"));
        }
    }
}

fn parse_args(args: &Vec<String>) -> (&str, &str) {
    if args.len() < 2 {
        println!("Using: {} [file_path] [w|c]", &args[0]);
        panic!("Not enough arguments");
    }
    let path = &args[1];
    let order = if args.len() > 2 { &args[2] } else { "c" };
    (path, order)
}

fn read_file(path: &str) -> String {
    let content = fs::read_to_string(&path);
    content.unwrap()
}

fn word_count(text: &str) -> HashMap<&str, u32> {
    let mut map: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map
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
    fn test_sort() {}
}
