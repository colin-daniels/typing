pub mod array;
pub mod ops;

mod macros;

use crate::ident::{TypeEqFn, TypeId};
use ops::*;

pub mod prelude {
    pub use super::Paren;
    pub use crate::{declare_func, impl_func, paren, paren_pat, Paren};
}

pub trait Paren: Sized {
    /// Number of elements/length of the `Paren`.
    /// # Examples
    /// ```
    /// use typing::prelude::*;
    /// assert_eq!(<Paren!(i32, bool, char)>::LEN, 3);
    /// assert_eq!(<Paren!(bool)>::LEN, 1);
    /// assert_eq!(<Paren!()>::LEN, 0);
    ///
    /// ```
    const LEN: usize;

    /// Number of elements/length of the `Paren`.
    /// # Examples
    /// ```
    /// use typing::prelude::*;
    /// assert_eq!(paren!(1, false, 'c').len(), 3);
    /// assert_eq!(paren!(true).len(), 1);
    /// assert_eq!(paren!().len(), 0);
    ///
    /// ```
    #[inline]
    fn len(&self) -> usize {
        Self::LEN
    }

    /// Returns `true` if there are no elements in the `Paren`.
    /// # Examples
    /// ```
    /// use typing::prelude::*;
    /// assert_eq!(paren!("hello", 3.0, 'z').is_empty(), false);
    /// assert_eq!(paren!().is_empty(), true);
    /// ```
    #[inline]
    fn is_empty(&self) -> bool {
        Self::LEN == 0
    }

    /// Apply a function to each element.
    /// # Examples
    /// ```
    /// use typing::paren::prelude::*;
    /// declare_func!(AddTwo {
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
        Map::map(self)
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
        Zip::zip(self, other)
    }

    /// Zip two parens with a given function that accepts a tuple of each pair of elements
    /// and returns a new type. E.g. implements `Func<(A, B)>` for every pair of elements.
    /// # Examples
    /// ```
    /// use typing::paren::prelude::*;
    /// use typing::func::ops::AddFn;
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
        ZipWith::zip_with(self, other)
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
        Filter::filter(self)
    }

    #[inline]
    fn filter_type<T: TypeId>(self) -> FilterOut<TypeEqFn<T>, Self>
    where
        Self: Filter<TypeEqFn<T>>,
    {
        Filter::filter(self)
    }

    #[inline]
    fn filter_map<F>(self) -> FilterMapOut<F, Self>
    where
        Self: FilterMap<F>,
    {
        FilterMap::filter_map(self)
    }

    /// Perform a left fold over all elements of the paren.
    /// # Examples
    /// ```
    /// use typing::prelude::*;
    /// declare_func!(FoldFunc {
    ///     |(acc, v): (u8,  u16)| -> u16 { acc as u16 + v },
    ///     |(acc, v): (u16, u32)| -> u32 { acc as u32 + v },
    ///     |(acc, v): (u32, u64)| -> u64 { acc as u64 + v },
    /// });
    ///
    /// let p: (u16, (u32, (u64, ()))) = paren!(1, 2, 3);
    /// assert_eq!(p.fold::<FoldFunc, _>(1u8), 7u64);
    /// ```
    #[inline]
    fn fold<F, B>(self, init: B) -> FoldOut<F, Self, B>
    where
        Self: Fold<F, B>,
    {
        Fold::fold(self, init)
    }

    /// Reverse the order of the elements in the paren.
    /// # Examples
    /// ```
    /// use typing::prelude::*;
    /// let a = paren!(true, [5, 10], "foo", "bar");
    /// let b = paren!("bar", "foo", [5, 10], true);
    /// assert_eq!(a.reverse(), b);
    /// ```
    #[inline]
    fn reverse(self) -> ReverseOut<Self>
    where
        Self: Reverse,
    {
        Reverse::reverse(self)
    }

    /// Scan over all elements of a paren with a mutable state.
    /// # Examples
    /// ```
    /// use typing::prelude::*;
    /// declare_func!(Accumulate {
    ///     |(state, v): (&mut u64, u16)| -> u64 { *state += v as u64; *state },
    ///     |(state, v): (&mut u64, u32)| -> u64 { *state += v as u64; *state },
    ///     |(state, v): (&mut u64, u64)| -> u64 { *state += v as u64; *state },
    /// });
    ///
    /// let p: (u16, (u32, (u64, (u64, ())))) = paren!(1, 2, 3, 4);
    /// assert_eq!(p.scan::<Accumulate, _>(0u64), paren!(1, 3, 6, 10));
    /// ```
    #[inline]
    fn scan<F, S>(self, state: S) -> ScanOut<F, S, Self>
    where
        Self: Scan<F, S>,
    {
        Scan::scan(self, state)
    }

    #[inline]
    fn any<F>(self) -> AnyOut<F, Self>
    where
        Self: Any<F>,
    {
        Any::any(self)
    }

    #[inline]
    fn all<F>(self) -> AllOut<F, Self>
    where
        Self: All<F>,
    {
        All::all(self)
    }
}

impl<A, B: Paren> Paren for (A, B) {
    const LEN: usize = 1 + <B as Paren>::LEN;
}

impl Paren for () {
    const LEN: usize = 0;
}
