use crate::func::{Apply, CanApply};
use crate::paren::Paren;

pub trait Map<F> {
    type Output: Paren;

    fn map(self) -> Self::Output;
}

pub type MapOut<F, T> = <T as Map<F>>::Output;

impl<F, A, B> Map<F> for (A, B)
where
    A: CanApply<F>,
    B: Map<F>,
{
    type Output = (Apply<F, A>, MapOut<F, B>);

    #[inline]
    fn map(self) -> Self::Output {
        let (a, b) = self;
        (a.apply(), b.map())
    }
}

// terminating case
impl<F> Map<F> for () {
    type Output = ();
    #[inline(always)]
    fn map(self) -> Self::Output {
        self
    }
}
