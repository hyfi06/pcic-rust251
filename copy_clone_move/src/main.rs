fn main() {
    let n = 10;
    let r = &n;
    let m = r + 1;
    println!("{}", m);

    let v = vec![1,2,3];
    dbg!(v);
}
