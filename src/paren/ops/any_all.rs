use crate::boolean::{AndFn, False, OrFn, True};
use crate::paren::ops::{Fold, FoldOut, Map, MapOut};

pub trait All<F> {
    type Output;
    fn all(self) -> Self::Output;
}

impl<F, T> All<F> for T
where
    Self: Map<F>,
    MapOut<F, Self>: Fold<AndFn, True>,
{
    type Output = FoldOut<AndFn, MapOut<F, Self>, True>;
    #[inline]
    fn all(self) -> Self::Output {
        self.map().fold(True::default())
    }
}

pub type AllOut<F, T> = <T as All<F>>::Output;

pub trait Any<F> {
    type Output;
    fn any(self) -> Self::Output;
}

impl<F, T> Any<F> for T
where
    Self: Map<F>,
    MapOut<F, Self>: Fold<OrFn, False>,
{
    type Output = FoldOut<OrFn, MapOut<F, Self>, False>;
    #[inline]
    fn any(self) -> Self::Output {
        self.map().fold(False::default())
    }
}

pub type AnyOut<F, T> = <T as Any<F>>::Output;
