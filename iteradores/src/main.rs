
#![feature(coroutines)]
#![feature(coroutine_trait)]
#![feature(iter_from_coroutine)]
#![feature(stmt_expr_attributes)]

use std::u128;

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

fn mkfibo() -> impl Iterator<Item = u128> {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    std::iter::from_fn(move || {
        let res = a;
        let temp = a;
        a = b;
        b += temp;
        Some(res)
    })
}

fn mkfibocoroutine() -> impl Iterator<Item = u128> {
    std::iter::form_coroutine(
        #[coroutine]
        || {
            let mut a: u128 = 0;
            let mut b: u128 = 1;
            loop {
                yield a;
                (a, b) = (b, a + b)
            }
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mkfibo() {
        let fibo_iter = mkfibo();
        for f in fibo_iter.take(8) {
            println!("{}", f);
        }
    }

    #[test]
    fn test_mkfibo_coroutine() {
        let fibo_iter = mkfibocoroutine();
        for f in fibo_iter.take(8) {
            println!("{}", f);
        }
    }

}
