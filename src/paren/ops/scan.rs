use crate::func::CanApply;
use crate::paren::Paren;

pub trait Scan<F, S> {
    type Output: Paren;

    fn scan(self, state: S) -> Self::Output;
}

pub type ScanOut<F, S, T> = <T as Scan<F, S>>::Output;

impl<A, B, F, S, AOut> Scan<F, S> for (A, B)
where
    for<'r> (&'r mut S, A): CanApply<F, Output = AOut>,
    B: Scan<F, S>,
{
    type Output = (AOut, ScanOut<F, S, B>);

    #[inline]
    fn scan(self, mut state: S) -> Self::Output {
        ((&mut state, self.0).apply(), self.1.scan(state))
    }
}

impl<F, S> Scan<F, S> for () {
    type Output = ();
    #[inline(always)]
    fn scan(self, _: S) -> Self::Output {
        self
    }
}
