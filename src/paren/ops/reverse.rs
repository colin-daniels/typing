use crate::func::Func;
use crate::paren::ops::{Fold, FoldOut};

pub struct Reverser;
impl<T, A> Func<(T, A)> for Reverser {
    type Output = (A, T);

    #[inline]
    fn call((t, a): (T, A)) -> Self::Output {
        (a, t)
    }
}

pub trait Reverse: Sized + Fold<Reverser, ()> {
    #[inline]
    fn reverse(self) -> FoldOut<Reverser, Self, ()> {
        self.fold(())
    }
}

impl<T> Reverse for T where T: Sized + Fold<Reverser, ()> {}

pub type ReverseOut<T> = FoldOut<Reverser, T, ()>;
