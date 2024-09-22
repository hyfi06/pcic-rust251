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

impl Complex<f64> {
    pub fn norm64(&self) -> f64 {
        (self.re.powi(2) + self.im.powi(2)).powf(0.5)
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
