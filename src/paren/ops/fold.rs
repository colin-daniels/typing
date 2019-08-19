use crate::func::{Apply, CanApply};

/// Left fold.
pub trait Fold<F, Acc> {
    type Output;

    fn fold(self, acc: Acc) -> Self::Output;
}

pub type FoldOut<F, T, Acc> = <T as Fold<F, Acc>>::Output;

impl<F, Acc, A, B> Fold<F, Acc> for (A, B)
where
    (Acc, A): CanApply<F>,
    B: Fold<F, Apply<F, (Acc, A)>>,
{
    type Output = FoldOut<F, B, Apply<F, (Acc, A)>>;

    #[inline]
    fn fold(self, acc: Acc) -> Self::Output {
        let (a, b) = self;
        b.fold((acc, a).apply())
    }
}

impl<F, Acc> Fold<F, Acc> for () {
    type Output = Acc;

    #[inline(always)]
    fn fold(self, acc: Acc) -> Self::Output {
        acc
    }
}
