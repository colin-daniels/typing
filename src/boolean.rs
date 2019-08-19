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

    macro_rules! logic_fn {
        ($name:ident, $trait:ident, $alias:ident) => {
            pub struct $name;
            impl<L: $trait<R>, R> $crate::func::Func<(L, R)> for $name {
                type Output = $alias<L, R>;
                #[inline]
                fn call(_: (L, R)) -> Self::Output {
                    $alias::<L, R>::default()
                }
            }
        };
    }
    
    pub trait BoolNot: Bool { type Output: Bool; }
    impl BoolNot for True  { type Output = False; }
    impl BoolNot for False { type Output = True;  }
    pub type Not<T> = <T as BoolNot>::Output;
    
    pub struct NotFn;
    impl<T: BoolNot> Func<T> for NotFn {
        type Output = Not<T>;
        #[inline]
        fn call(_: T) -> Self::Output { Not::<T>::default() }
    }
    
    pub trait BoolAnd<R>: Bool { type Output: Bool; }
    impl BoolAnd<True>  for False { type Output = False; }
    impl BoolAnd<False> for True  { type Output = False; }
    impl BoolAnd<True>  for True  { type Output = True;  }
    impl BoolAnd<False> for False { type Output = False; }
    pub type And<L, R> = <L as BoolAnd<R>>::Output;
    logic_fn!(AndFn, BoolAnd, And);

    pub trait BoolOr<R>: Bool { type Output: Bool; }
    impl BoolOr<True>  for False { type Output = True;  }
    impl BoolOr<False> for True  { type Output = True;  }
    impl BoolOr<True>  for True  { type Output = True;  }
    impl BoolOr<False> for False { type Output = False; }
    pub type Or<L, R> = <L as BoolOr<R>>::Output;
    logic_fn!(OrFn, BoolOr, Or);
    
    pub trait BoolXor<R>: Bool { type Output: Bool; }
    impl BoolXor<True>  for False { type Output = True;  }
    impl BoolXor<False> for True  { type Output = True;  }
    impl BoolXor<True>  for True  { type Output = False; }
    impl BoolXor<False> for False { type Output = False; }
    pub type Xor<L, R> = <L as BoolXor<R>>::Output;
    logic_fn!(XorFn, BoolXor, Xor);
    
    pub trait BoolIfElse<T, F>: Bool { type Output; }
    impl<T, F> BoolIfElse<T, F> for True  { type Output = T; }
    impl<T, F> BoolIfElse<T, F> for False { type Output = F; }
    pub type IfElse<B, T, F> = <B as BoolIfElse<T, F>>::Output;
}

pub use ops::*;
