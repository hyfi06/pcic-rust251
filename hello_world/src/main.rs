fn main() {
    let n = 10.5f64;
    println!("{}", duplica(n));
}

fn duplica<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
    return x + x;
}
