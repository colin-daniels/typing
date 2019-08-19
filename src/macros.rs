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

#[macro_export]
macro_rules! named_func {
    ($name:ident {
            $($t:tt)*
    }) => {
        struct $name;
        $crate::impl_func!(for $name { $($t)* });
        $name
    };
}

/// Assert at compile-time that two types are the same.
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
