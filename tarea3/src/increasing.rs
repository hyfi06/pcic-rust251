pub mod generic;
pub mod u32;

pub trait Increasing: Iterator {
    fn increasing(self) -> impl Iterator<Item = Self::Item>;
}

impl<I> Increasing for I
where
    I: Iterator,
    I::Item: PartialOrd,
{
    fn increasing(self) -> impl Iterator<Item = Self::Item> {
        generic::increasing_generic(self)
    }
}
