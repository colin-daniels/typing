use std::fmt::Debug;

pub type False = crate::T0;
pub type True = crate::T1;

mod private_bool {
    pub trait Sealed {}
    impl Sealed for super::True {}
    impl Sealed for super::False {}
}

pub trait Bool: Debug + Default + Clone + Copy + private_bool::Sealed {}
impl Bool for True {}
impl Bool for False {}

impl Into<bool> for True {
    #[inline(always)]
    fn into(self) -> bool {
        true
    }
}

impl Into<bool> for False {
    #[inline(always)]
    fn into(self) -> bool {
        false
    }
}

// Literally only have this module since [E0658] stops us from using skip on the module
// itself and intellij ignores rustfmt.toml...
#[rustfmt::skip]
mod ops {
    use super::{Bool, True, False};
    use crate::func::Func;

    macro_rules! logic_unop {
        ($btrait:ident, $trait:ident, $method:ident; $in:ident => $out:ident, $($rest:tt)*) => {
            impl $btrait for $in { type Output = $out; }
            impl std::ops::$trait for $in {
                type Output = $out;
                #[inline]
                fn $method(self) -> Self::Output {
                    Default::default()
                }
            }
            logic_unop!($btrait, $trait, $method; $($rest)*);
        };
        ($btrait:ident, $trait:ident, $method:ident;) => {};
    }

    macro_rules! logic_binop {
        (
            $btrait:ident, $trait:ident, $method:ident; 
            ($l:ident, $r:ident) => $out:ident, 
            $($rest:tt)*
        ) => {
            impl $btrait<$r> for $l { type Output = $out; }
            impl std::ops::$trait<$r> for $l {
                type Output = $out;
                #[inline]
                fn $method(self, _: $r) -> Self::Output {
                    Default::default()
                }
            }
            logic_binop!($btrait, $trait, $method; $($rest)*);
        };
        ($btrait:ident, $trait:ident, $method:ident;) => {};
    }

    macro_rules! binop_fn {
        ($name:ident, $trait:ident) => {
            impl<L, R> Func<(L, R)> for $name
            where
                L: $trait<R>,
            {
                type Output = <L as $trait<R>>::Output;
                #[inline]
                fn call(_: (L, R)) -> Self::Output {
                    Default::default()
                }
            }
        };
    }

    pub trait BoolNot { type Output: Bool; }
    pub type Not<T> = <T as BoolNot>::Output;
    logic_unop!(
        BoolNot, Not, not; 
        True => False,
        False => True,
    );

    pub trait BoolAnd<R> { type Output: Bool; }    
    logic_binop!(
        BoolAnd, BitAnd, bitand; 
        (False, False) => False,
        (False,  True) => False,
        ( True, False) => False,
        ( True,  True) =>  True,
    );
    
    pub type And<L, R> = <L as BoolAnd<R>>::Output;
    pub struct AndFn;
    binop_fn!(AndFn, BoolAnd);

    pub trait BoolOr<R> { type Output: Bool; }
    logic_binop!(
        BoolOr, BitOr, bitor; 
        (False, False) => False,
        (False,  True) =>  True,
        ( True, False) =>  True,
        ( True,  True) =>  True,
    );

    pub type Or<L, R> = <L as BoolOr<R>>::Output;
    pub struct OrFn;
    binop_fn!(OrFn, BoolOr);

    pub trait BoolXor<R> { type Output: Bool; }
    logic_binop!(
        BoolXor, BitXor, bitxor; 
        (False, False) => False,
        (False,  True) =>  True,
        ( True, False) =>  True,
        ( True,  True) => False,
    );
    
    pub type Xor<L, R> = <L as BoolXor<R>>::Output;
    pub struct XorFn;
    binop_fn!(XorFn, BoolXor);
    
    pub trait BoolIfElse<T, F>: Bool { type Output; }
    impl<T, F> BoolIfElse<T, F> for True  { type Output = T; }
    impl<T, F> BoolIfElse<T, F> for False { type Output = F; }
    pub type IfElse<B, T, F> = <B as BoolIfElse<T, F>>::Output;
}

pub use ops::*;
