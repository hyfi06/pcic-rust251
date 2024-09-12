fn main() {
    let vect = vec![String::from("Hola"), String::from("mundo"), String::from("!")];
    for &val in vect.iter() {
        println!("{}", val);
    }
}
