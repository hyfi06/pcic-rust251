use std::{iter::zip, path::Iter};

struct Ones {}

impl Iterator for Ones {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(1);
    }
}

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
    let ones = Ones {};
    let ones2 = Ones {};
    for s in ones.take(5) {
        println!("{}", s);
    }

    for (a, b) in zip(ones, ones2).skip(3).step_by(2).take(6) {
        println!("({}, {})", a, b);
    }
    for (a, b) in zip(ones.take(10), ones2.take(8)) {
        println!("({}, {})", a, b);
    }

    // let fibo = Fibo { a: 0, b: 1 };
    // for f in fibo.take(10) {
    //     println!("{}", f)
    // }
}
