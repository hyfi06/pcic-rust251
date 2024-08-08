fn main() {
    let n = 10.5f64;
    let s1 = String::from("hola mundo");
    println!("{}", s1);
    let s2 = &s1;
    println!("{}", s2);
    println!("{}", s1);
    println!("{}", duplica(n));
}

fn duplica<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
    return x + x;
}
