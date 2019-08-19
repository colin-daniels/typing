use crate::boolean::Bool;
use crate::func::Func;
use std::marker::PhantomData;

pub trait TypeEq<Rhs> {
    type Output: Bool;
}

pub type IsEq<L, R> = <L as TypeEq<R>>::Output;

/// Proxy trait to build sets of distinguishable types.
pub trait TypeId {
    type Id;
}

pub type Id<T> = <T as TypeId>::Id;

impl<L, R> TypeEq<R> for L
where
    L: TypeId,
    R: TypeId,
    Id<L>: TypeEq<Id<R>>,
{
    type Output = IsEq<Id<L>, Id<R>>;
}

#[rustfmt::skip]
mod impls {
    use crate::{T0, T1};
    use crate::boolean::{False, True, And, BoolAnd};
    use super::{TypeEq, IsEq};

    impl TypeEq<T0> for T1 { type Output = False; }
    impl TypeEq<T1> for T0 { type Output = False; }
    impl TypeEq<T1> for T1 { type Output = True;  }
    impl TypeEq<T0> for T0 { type Output = True;  }

    impl<R1, R2> TypeEq<(R1, R2)> for T1 { type Output = False; }
    impl<R1, R2> TypeEq<(R1, R2)> for T0 { type Output = False; }
    impl<L1, L2> TypeEq<T1> for (L1, L2) { type Output = False; }
    impl<L1, L2> TypeEq<T0> for (L1, L2) { type Output = False; }

    impl<L1, L2, R1, R2> TypeEq<(R1, R2)> for (L1, L2)
    where
        L1: TypeEq<R1>,
        L2: TypeEq<R2>,
        IsEq<L1, R1>: BoolAnd<IsEq<L2, R2>>,
    {
        type Output = And<
            IsEq<L1, R1>,
            IsEq<L2, R2>
        >;
    }
}

#[derive(Debug)]
pub struct TypeEqFn<T>(PhantomData<*const T>);

impl<L, R> Func<R> for TypeEqFn<L>
where
    L: TypeEq<R>,
{
    type Output = IsEq<L, R>;
    #[inline]
    fn call(_: R) -> Self::Output {
        IsEq::<L, R>::default()
    }
}
