struct Fibo {
    a: u128,
    b: u128,
}

impl Iterator for Fibo {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.a;
        let temp = self.a;
        self.a = self.b;
        self.b += temp;
        Some(res)
    }
}

fn main() {
    let fibo = Fibo { a: 0, b: 1 };
    for f in fibo.take(10) {
        println!("{}", f)
    }
}
