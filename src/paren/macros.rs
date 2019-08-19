/// Helper macro to define a nested tuple pattern.
/// # Examples
/// ```
/// use typing::{paren, paren_pat};
/// let p: (char, (i32, (f64, (Vec<bool>, ()))))
///     = paren!('c', 2, std::f64::consts::PI, vec![false, true]);
///
/// let paren_pat!(a, b, c, d) = p;
/// assert_eq!((a, b, c, d), ('c', 2, std::f64::consts::PI, vec![false, true]));
/// ```
#[macro_export]
macro_rules! paren_pat {
    ($(,)?) => { () };
    ($a:pat $(,)?) => { ($a, ()) };
    ($a:pat, $($tok:tt)+) => {
        ($a, $crate::paren_pat!($($tok)+))
    };
}

/// Helper macro to define a nested tuple.
/// # Examples
/// ```
/// use typing::{paren, Paren, paren_pat};
/// let p: (char, (i32, (f64, (Vec<bool>, ()))))
///     = paren!('c', 2, std::f64::consts::PI, vec![false, true]);
/// // the Paren macro can also be used to help with defining types
/// let p: Paren!(char, i32, f64, Vec<bool>) = p;
/// // and paren_pat can be used to pattern match
/// let paren_pat!(a, b, c, d) = p;
/// assert_eq!((a, b, c, d), ('c', 2, std::f64::consts::PI, vec![false, true]));
/// ```
#[macro_export]
macro_rules! paren {
    ($(,)?) => { () };
    ($a:expr $(,)?) => { ($a, ()) };
    ($a:expr, $($tok:tt)+) => {
        ($a, $crate::paren!($($tok)*))
    };
}

/// Helper macro to define a nested tuple type.
/// # Examples
/// ```
/// use typing::{paren, Paren, assert_type_eq};
/// let _: Paren!(char, i32, f64, Vec<bool>)
///     = paren!('c', 2, std::f64::consts::PI, vec![false, true]);
///
/// assert_type_eq!((), Paren!());
/// assert_type_eq!((), Paren!(,));
/// assert_type_eq!((i32, ()), Paren!(i32));
/// assert_type_eq!((i32, (f64, ())), Paren!(i32, f64));
/// ```
#[macro_export]
macro_rules! Paren {
    ($(,)?) => { () };
    ($a:ty $(,)?) => { ($a, ()) };
    ($a:ty, $($tok:tt)+) => {
        ($a, $crate::Paren!($($tok)*))
    };
}
