use crate::boolean::{False, True};
use crate::func::{Apply, CanApply};
use crate::paren::Paren;

// we do a two-stage thing where Filter evaluates whether we should keep the next element,
// then Filter2 actually drops or keeps it
pub trait Filter<F>: Sized {
    type Output: Paren;

    fn filter(self) -> Self::Output;
}

#[doc(hidden)]
pub trait Filter2<F, Marker>: Sized {
    type Output: Paren;

    fn filter2(self) -> Self::Output;
}

pub type FilterOut<F, T> = <T as Filter<F>>::Output;
type FilterOut2<F, A, B> = <(A, B) as Filter2<F, Apply<F, A>>>::Output;

impl<F, A, B> Filter<F> for (A, B)
where
    A: CanApply<F>,
    Self: Filter2<F, Apply<F, A>>,
{
    type Output = FilterOut2<F, A, B>;

    // we have the always here just because it's an intermediate step
    #[inline(always)]
    fn filter(self) -> Self::Output {
        self.filter2()
    }
}

impl<F, A, B> Filter2<F, True> for (A, B)
where
    B: Filter<F>,
{
    type Output = (A, FilterOut<F, B>);

    #[inline]
    fn filter2(self) -> Self::Output {
        (self.0, self.1.filter())
    }
}

impl<F, A, B> Filter2<F, False> for (A, B)
where
    B: Filter<F>,
{
    type Output = FilterOut<F, B>;

    #[inline]
    fn filter2(self) -> Self::Output {
        self.1.filter()
    }
}

impl<F> Filter<F> for () {
    type Output = ();
    #[inline(always)]
    fn filter(self) -> Self::Output {
        self
    }
}
