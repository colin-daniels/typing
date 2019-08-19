use crate::func::Identity;
use crate::paren::ops::{ZipWith, ZipWithOut};

pub trait Zip<B>: Sized + ZipWith<Identity, B> {
    #[inline]
    fn zip(self, b: B) -> ZipWithOut<Identity, Self, B> {
        self.zip_with(b)
    }
}

impl<A, B> Zip<B> for A where A: Sized + ZipWith<Identity, B> {}

pub type ZipOut<A, B> = ZipWithOut<Identity, A, B>;
