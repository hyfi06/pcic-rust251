pub mod generic;
pub mod u32;

pub trait Increasing
where
    Self: Iterator + 'static,
    Self::Item: PartialOrd,
{
    fn increasing(self) -> Box<dyn Iterator<Item = Self::Item>> where Self:Sized, <Self as Iterator>::Item: 'static {
        Box::new(generic::increasing_generic(self))
    }
}

impl<I> Increasing for I
where
    I: Iterator + 'static,
    I::Item: PartialOrd,
{
}
