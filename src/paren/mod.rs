pub mod array;
pub mod ops;

mod macros;

use ops::*;

pub mod prelude {
    pub use super::Paren;
    pub use crate::{impl_func, named_func, paren, paren_pat, Paren};
}

pub trait Paren: Sized {
    const LEN: usize;

    #[inline]
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Apply a function to each element.
    /// # Examples
    /// ```
    /// use typing::paren::prelude::*;
    /// named_func!(AddTwo {
    ///     |x: f64| -> f64 { x + 2.0 },
    ///     |x: i32| -> i32 { x + 2 },
    ///     |x: &str| -> String { String::from(x) + " " + x },
    /// });
    ///
    /// let p = paren!(1.0, 2, "Hello");
    /// assert_eq!(p.map::<AddTwo>(), paren!(3.0, 4, String::from("Hello Hello")));
    /// ```
    #[inline]
    fn map<F>(self) -> MapOut<F, Self>
    where
        Self: Map<F>,
    {
        <Self as Map<_>>::map(self)
    }

    /// Zip two parens.
    /// # Examples
    /// ```
    /// use typing::paren::prelude::*;
    ///
    /// let a = paren!(1.0, 2, "Hello");
    /// let b = paren!(false, (), vec![3.0]);
    /// let c = a.zip(b);
    /// assert_eq!(c, paren!(
    ///     (1.0, false),
    ///     (2, ()),
    ///     ("Hello", vec![3.0]),
    /// ));
    /// ```
    #[inline]
    fn zip<B>(self, other: B) -> ZipOut<Self, B>
    where
        Self: Zip<B>,
    {
        <Self as Zip<_>>::zip(self, other)
    }

    /// Zip two parens with a given function that accepts a tuple of each pair of elements
    /// and returns a new type. E.g. implements `Func<(A, B)>` for every pair of elements.
    /// # Examples
    /// ```
    /// use typing::paren::prelude::*;
    /// use typing::func::AddFn;
    ///
    /// let a = paren!(1.0, 2, String::from("Hello"));
    /// let b = paren!(2.0, 2, "World");
    ///
    /// let c = a.zip_with::<AddFn, _>(b);
    /// assert_eq!(c, paren!(3.0, 4, String::from("HelloWorld")));
    /// ```
    #[inline]
    fn zip_with<F, B>(self, other: B) -> ZipWithOut<F, Self, B>
    where
        Self: ZipWith<F, B>,
    {
        <Self as ZipWith<_, _>>::zip_with(self, other)
    }

    /// Construct a paren with elements cloned from a given value.
    /// # Examples
    /// ```
    /// use typing::paren::prelude::*;
    /// let p: (_, _) = Paren::fill(false);
    /// assert_eq!(p, paren!(false, false, false, false));
    /// ```
    #[inline]
    fn fill<T>(init: T) -> Self
    where
        T: Clone,
        Self: Fill<T>,
    {
        Fill::fill(init)
    }

    #[inline]
    fn filter<F>(self) -> FilterOut<F, Self>
    where
        Self: Filter<F>,
    {
        <Self as Filter<_>>::filter(self)
    }

    #[inline]
    fn filter_map<F>(self) -> FilterMapOut<F, Self>
    where
        Self: FilterMap<F>,
    {
        <Self as FilterMap<_>>::filter_map(self)
    }

    #[inline]
    fn fold<F, B>(self, init: B) -> FoldOut<F, Self, B>
    where
        Self: Fold<F, B>,
    {
        <Self as Fold<_, _>>::fold(self, init)
    }

    #[inline]
    fn reverse(self) -> ReverseOut<Self>
    where
        Self: Reverse,
    {
        <Self as Reverse>::reverse(self)
    }

    #[inline]
    fn scan<F, S>(self, state: S) -> ScanOut<F, S, Self>
    where
        Self: Scan<F, S>,
    {
        <Self as Scan<_, _>>::scan(self, state)
    }
}

impl<A, B: Paren> Paren for (A, B) {
    const LEN: usize = 1 + <B as Paren>::LEN;
}

impl Paren for () {
    const LEN: usize = 0;
}
