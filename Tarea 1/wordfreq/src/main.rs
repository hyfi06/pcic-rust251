use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = parse_args(&args);
    let text = open_file(file_path);
    let frequencies = word_count(&text);
    for (&word, freq) in &frequencies {
        println!("{word}: {freq}");
    }
}

fn parse_args(args: &Vec<String>) -> &str {
    if args.len() < 2 {
        println!("Using: {args[0]} [file_path]");
        panic!("Not enough arguments");
    }
    &args[1]
}

fn open_file(path: &str) -> String {
    let file = fs::read_to_string(path);
    file.unwrap()
}

fn word_count(text: &str) -> HashMap<&str, u32> {
    let mut map: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    map
}
