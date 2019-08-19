pub trait Deriv<Tag> {
    type Output;
    fn deriv(&self) -> Self::Output;
}

pub type Dv<T, Tag> = <T as Deriv<Tag>>::Output;
