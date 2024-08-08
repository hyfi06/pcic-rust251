fn main() {
    let n: i32 = 10;
    println!("{}", duplica(n as i64));
}

fn duplica(x: i64) -> i64 {
    return x + x;
}
