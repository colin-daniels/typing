use crate::boolean::Bool;

pub trait TypeIsSame<Rhs> {
    type Output: Bool;
}

pub type IsSame<L, R> = <L as TypeIsSame<R>>::Output;

/// Proxy trait to build sets of distinguishable types.
pub trait TypeId {
    type Id;
}

pub type Id<T> = <T as TypeId>::Id;

impl<L, R> TypeIsSame<R> for L
where
    L: TypeId,
    R: TypeId,
    Id<L>: TypeIsSame<Id<R>>,
{
    type Output = IsSame<Id<L>, Id<R>>;
}

#[rustfmt::skip]
mod impls {
    use crate::{T0, T1};
    use crate::boolean::{False, True, And, BoolAnd};
    use super::{TypeIsSame, IsSame};

    impl TypeIsSame<T0> for T1 { type Output = False; }
    impl TypeIsSame<T1> for T0 { type Output = False; }
    impl TypeIsSame<T1> for T1 { type Output = True;  }
    impl TypeIsSame<T0> for T0 { type Output = True;  }

    impl<R1, R2> TypeIsSame<(R1, R2)> for T1 { type Output = False; }
    impl<R1, R2> TypeIsSame<(R1, R2)> for T0 { type Output = False; }
    impl<L1, L2> TypeIsSame<T1> for (L1, L2) { type Output = False; }
    impl<L1, L2> TypeIsSame<T0> for (L1, L2) { type Output = False; }

    impl<L1, L2, R1, R2> TypeIsSame<(R1, R2)> for (L1, L2)
    where
        L1: TypeIsSame<R1>,
        L2: TypeIsSame<R2>,
        IsSame<L1, R1>: BoolAnd<IsSame<L2, R2>>,
    {
        type Output = And<
            IsSame<L1, R1>,
            IsSame<L2, R2>
        >;
    }
}
