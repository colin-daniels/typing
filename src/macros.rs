/// Implement the [`Func`] trait for a given type using syntax similar to closures.
/// # Examples
/// ```
/// use typing::{impl_func, func::Func};
/// struct Double;
/// impl_func!(for Double {
///     |x: f64| -> f64 { x * 2.0 },
///     // lifetimes or generic parameters can be put before the function in brackets
///     ['a] |y: &'a mut f32| { *y *= 2.0; },
///     [T: Copy] |y: [T; 1]| -> [T; 2] { [y[0], y[0]] },
///     // attributes are allowed as well, and will be applied to the function definition
///     #[inline]
///     |x: usize| -> (usize, usize) { (x, x) },
/// });
///
/// let mut x = 1.0;
/// Double::call(&mut x);
/// assert_eq!(x, 2.0);
///
/// assert_eq!(Double::call(2.0), 4.0);
/// assert_eq!(Double::call(2), (2, 2));
/// assert_eq!(Double::call(["hello"]), ["hello", "hello"]);
/// ```
/// [`Func`]: func/trait.Func.html
#[macro_export]
macro_rules! impl_func {
    (for $typ:ty {
            // ignore any commas
            $(,)?
            // optional function meta
            $(#[$($meta:meta)*])*
            // lifetimes/generic params
            $([$($c:tt $(: $c0:ident $(+ $cN:ident)*)?),*])?
            // function arg
            |$arg:tt: $arg_type:ty|
            // optional return type
            $(-> $ret:ty)?
            // function body
            $body:block
            // rest of the functions
            $($rest:tt)*
    }) => {
        impl<$($($c $(: $c0 $(+ $cN)*)?,)*)?> $crate::func::Func<$arg_type> for $typ
        {
            // note: we can put the return type in parentheses since there shouldn't be a comma
            type Output = ($($ret)?);

            $(#[$($meta)*])*
            fn call($arg: $arg_type) -> Self::Output $body
        }

        $crate::impl_func!(for $typ { $($rest)* });
    };
    (for $typ:ty {$(,)?}) => {};
}
/// Declare an empty struct type with an optional name and implement the [`Func`] trait for it.
///
/// Also implements a member function `call` that can be used for the case of anonymous structs.
/// Function syntax is equivalent to [`impl_func`].
/// # Examples
/// ```
/// use typing::{declare_func, func::Func, paren, paren::Paren};
/// declare_func!(Double {
///     |x: f64| -> f64 { x * 2.0 },
///     |x: i32| -> i32 { x * 2 },
/// });
///
/// let p = paren!(1, 2.0, 3, 4.0, 5);
/// assert_eq!(p.map::<Double>(), paren!(2, 4.0, 6, 8.0, 10));
///
/// // if no name is specified, the macro returns an instance of an anonymous struct that
/// // implements Func.
/// let f = declare_func!({
///     [T] |(mut a, mut b): (Vec<T>, Vec<T>)| -> Vec<T> { a.extend(b); a }
/// });
/// assert_eq!(vec![true, false], f.call((vec![true], vec![false])));
/// assert_eq!(vec![0, 0, 1, 1], f.call((vec![0, 0], vec![1, 1])));
/// ```
/// [`Func`]: func/trait.Func.html
/// [`impl_func`]: macro.impl_func.html
#[macro_export]
macro_rules! declare_func {
    ($name:ident { $($t:tt)* }) => {
        struct $name;
        $crate::impl_func!(for $name { $($t)* });
        impl $name {
            #[inline]
            pub fn call<T, R>(&self, arg: T) -> R
            where
                T: $crate::func::CanApply<Self, Output = R>,
            {
                T::apply(arg)
            }
        }
    };
    ({ $($t:tt)* }) => {{
        $crate::declare_func!( F { $($t)* });
        F
    }};
}

/// Assert at compile-time that two types are the same.
/// # Examples
/// ```
/// use typing::{assert_type_eq, Paren};
/// use typing::boolean::{True, False, And};
///
/// assert_type_eq!(f64, <Vec<f64> as IntoIterator>::Item);
///
/// assert_type_eq!(And<True, False>, False);
/// assert_type_eq!(And<True, True>, True);
///
/// assert_type_eq!(
///     Paren!(bool, char, [i32; 1]),
///     (bool, (char, ([i32; 1], ())))
/// );
/// ```
#[macro_export]
macro_rules! assert_type_eq {
    ($a:ty, $b:ty) => {
        let _ = |x: $a| -> $b { x };
    };
}

/// Replace first argument by the second, interprets as tts.
#[doc(hidden)]
#[macro_export]
macro_rules! __rep {
    ($a:tt by $b:tt) => {
        $b
    };
}

/// Counts number of tts recursively.
#[doc(hidden)]
#[macro_export]
macro_rules! __count_tts {
    () => {0usize};
    ($_a:tt $_b:tt $_c:tt $_d:tt $($t:tt)*) => {4usize + $crate::__count_tts!($($t)*)};
    ($_a:tt $_b:tt $_c:tt $($t:tt)*) => {3usize + $crate::__count_tts!($($t)*)};
    ($_a:tt $_b:tt $($t:tt)*) => {2usize + $crate::__count_tts!($($t)*)};
    ($_a:tt $($t:tt)*) => {1usize + $crate::__count_tts!($($t)*)};
}
