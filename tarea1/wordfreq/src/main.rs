use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &str = parse_args(&args);
    let text: String = read_file(file_path);
    let frequencies: HashMap<&str, u32> = word_count(&text);
    for (&word, freq) in &frequencies {
        println!("{word}: {freq}");
    }
}

fn parse_args(args: &Vec<String>) -> &str {
    if args.len() < 2 {
        println!("Using: {} [file_path]", &args[0]);
        panic!("Not enough arguments");
    }
    let file_path: &String = &args[1];
    file_path
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


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[should_panic]
    fn test_parse_args_panic() {
        let args: Vec<String> = vec![String::from("wordfreq")];
        parse_args(&args);
    }

    #[test]
    fn test_parse_args() {
        let args: Vec<String> = vec![String::from("wordfreq"), String::from("poem.txt")];
        let file_path = parse_args(&args);
        assert_eq!(file_path, "poem.txt")
    }

    #[test]
    fn test_word_count() {
        let text = String::from("hello world wonderful world");
        let map = word_count(&text);
        assert_eq!(map["world"],2)
    }
}
