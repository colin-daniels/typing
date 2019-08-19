use crate::boolean::{AndFn, True};
use crate::paren::ops::{Fold, FoldOut, Map, MapOut};

pub trait All<F>: Map<F>
where
    MapOut<F, Self>: Fold<AndFn, True>,
{
    type Output;
}

impl<F, T> All<F> for T
where
    Self: Map<F>,
    MapOut<F, Self>: Fold<AndFn, True>,
{
    type Output = FoldOut<AndFn, MapOut<F, Self>, True>;
}

pub type AllOut<F, T> = <T as All<F>>::Output;
