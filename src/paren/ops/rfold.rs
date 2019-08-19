use crate::func::{Apply, CanApply};

/// Right fold.
pub trait RFold<F, Acc> {
    type Output;

    fn fold(self, acc: Acc) -> Self::Output;
}

pub type RFoldOut<F, T, Acc> = <T as RFold<F, Acc>>::Output;

impl<F, Acc, A, B> RFold<F, Acc> for (A, B)
where
    B: RFold<F, Acc>,
    (RFoldOut<F, B, Acc>, A): CanApply<F>,
{
    type Output = Apply<F, (RFoldOut<F, B, Acc>, A)>;

    #[inline]
    fn fold(self, acc: Acc) -> Self::Output {
        let (a, b) = self;
        (b.fold(acc), a).apply()
    }
}

impl<F, Acc> RFold<F, Acc> for () {
    type Output = Acc;

    #[inline(always)]
    fn fold(self, acc: Acc) -> Self::Output {
        acc
    }
}
