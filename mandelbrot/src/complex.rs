use std::ops;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    pub fn new(re: T, im: T) -> Complex<T> {
        Complex { re, im }
    }
}

impl<T: ops::Add<Output = T>> ops::Add for Complex<T> {
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::<T>::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl<T> ops::Mul for Complex<T>
where
    T: ops::Mul<Output = T> + ops::Add<Output = T> + ops::Sub<Output = T> + Copy,
{
    type Output = Complex<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex::<T>::new(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> Complex<T> {
    pub fn norm_sqrt_f64(&self) -> T {
        self.re * self.re + self.im * self.im
    }
}
