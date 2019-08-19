use crate::func::CanApply;
use crate::paren::Paren;

pub trait ZipWith<F, B> {
    type Output: Paren;

    fn zip_with(self, b: B) -> Self::Output;
}

pub type ZipWithOut<F, A, B> = <A as ZipWith<F, B>>::Output;

impl<A1, A2, B1, B2, F, Out> ZipWith<F, (B1, B2)> for (A1, A2)
where
    (A1, B1): CanApply<F, Output = Out>,
    A2: ZipWith<F, B2>,
{
    type Output = (Out, ZipWithOut<F, A2, B2>);

    #[inline]
    fn zip_with(self, (b1, b2): (B1, B2)) -> Self::Output {
        let (a1, a2) = self;
        ((a1, b1).apply(), a2.zip_with(b2))
    }
}

impl<F> ZipWith<F, ()> for () {
    type Output = ();

    #[inline(always)]
    fn zip_with(self, _: ()) -> Self::Output {}
}
