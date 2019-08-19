use crate::func::{Apply, CanApply};
use crate::option::{NoneType, SomeType};
use crate::paren::Paren;

// same deal as Filter, need to do a two stage thing where we process the results in another trait
pub trait FilterMap<F> {
    type Output: Paren;

    fn filter_map(self) -> Self::Output;
}

#[doc(hidden)]
pub trait FilterMap2<F, B> {
    type Output;

    fn filter_map2(self, rest: B) -> Self::Output;
}

pub type FilterMapOut<F, T> = <T as FilterMap<F>>::Output;
type FilterMapOut2<F, A, B> = <Apply<F, A> as FilterMap2<F, B>>::Output;

impl<F, A, B> FilterMap<F> for (A, B)
where
    A: CanApply<F>,
    Apply<F, A>: FilterMap2<F, B>,
    FilterMapOut2<F, A, B>: Paren,
{
    type Output = FilterMapOut2<F, A, B>;

    #[inline(always)]
    fn filter_map(self) -> Self::Output {
        let (a, b) = self;
        a.apply().filter_map2(b)
    }
}

impl<F, A, B> FilterMap2<F, B> for SomeType<A>
where
    B: FilterMap<F>,
{
    type Output = (A, FilterMapOut<F, B>);

    #[inline]
    fn filter_map2(self, b: B) -> Self::Output {
        (self.0, b.filter_map())
    }
}

impl<F, B> FilterMap2<F, B> for NoneType
where
    B: FilterMap<F>,
{
    type Output = FilterMapOut<F, B>;

    #[inline]
    fn filter_map2(self, b: B) -> Self::Output {
        b.filter_map()
    }
}

impl<F> FilterMap<F> for () {
    type Output = ();
    #[inline(always)]
    fn filter_map(self) -> Self::Output {
        self
    }
}
