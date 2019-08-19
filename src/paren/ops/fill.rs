use crate::paren::Paren;

pub trait Fill<T: Clone>: Paren {
    fn fill(init: T) -> Self;
}

impl<T: Clone, B> Fill<T> for (T, B)
where
    B: Fill<T>,
{
    #[inline]
    fn fill(init: T) -> Self {
        (init.clone(), <B as Fill<_>>::fill(init))
    }
}

impl<T: Clone> Fill<T> for () {
    #[inline(always)]
    fn fill(_: T) -> Self {}
}
