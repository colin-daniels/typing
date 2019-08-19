use super::Func;

macro_rules! impl_std_binop_fn {
    ($name:ident, $trait:ident, $method:ident) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name;
        impl<L, R> Func<(L, R)> for $name
        where
            L: std::ops::$trait<R>,
        {
            type Output = <L as std::ops::$trait<R>>::Output;
            #[inline]
            fn call((l, r): (L, R)) -> Self::Output {
                std::ops::$trait::$method(l, r)
            }
        }
    };
}

impl_std_binop_fn!(AddFn, Add, add);
impl_std_binop_fn!(BitAndFn, BitAnd, bitand);
impl_std_binop_fn!(BitOrFn, BitOr, bitor);
impl_std_binop_fn!(BitXorFn, BitXor, bitxor);
impl_std_binop_fn!(DivFn, Div, div);
impl_std_binop_fn!(MulFn, Mul, mul);
impl_std_binop_fn!(RemFn, Rem, rem);
impl_std_binop_fn!(ShlFn, Shl, shl);
impl_std_binop_fn!(ShrFn, Shr, shr);
impl_std_binop_fn!(SubFn, Sub, sub);

macro_rules! impl_std_unop_fn {
    ($name:ident, $trait:ident, $method:ident) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name;
        impl<T> Func<T> for $name
        where
            T: std::ops::$trait,
        {
            type Output = <T as std::ops::$trait>::Output;
            #[inline]
            fn call(t: T) -> Self::Output {
                std::ops::$trait::$method(t)
            }
        }
    };
}

impl_std_unop_fn!(NotFn, Not, not);
impl_std_unop_fn!(NegFn, Neg, neg);

// NOTE: Not really sure if this should return a reference to the Lhs or not...
macro_rules! impl_std_assignop_fn {
    ($name:ident, $trait:ident, $method:ident) => {
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name;
        impl<'a, L, R> Func<(&'a mut L, R)> for $name
        where
            L: std::ops::$trait<R>,
        {
            type Output = &'a mut L;
            #[inline]
            fn call((l, r): (&'a mut L, R)) -> Self::Output {
                std::ops::$trait::$method(l, r);
                l
            }
        }
    };
}

impl_std_assignop_fn!(AddAssignFn, AddAssign, add_assign);
impl_std_assignop_fn!(BitAndAssignFn, BitAndAssign, bitand_assign);
impl_std_assignop_fn!(BitOrAssignFn, BitOrAssign, bitor_assign);
impl_std_assignop_fn!(BitXorAssignFn, BitXorAssign, bitxor_assign);
impl_std_assignop_fn!(DivAssignFn, DivAssign, div_assign);
impl_std_assignop_fn!(MulAssignFn, MulAssign, mul_assign);
impl_std_assignop_fn!(RemAssignFn, RemAssign, rem_assign);
impl_std_assignop_fn!(ShlAssignFn, ShlAssign, shl_assign);
impl_std_assignop_fn!(ShrAssignFn, ShrAssign, shr_assign);
impl_std_assignop_fn!(SubAssignFn, SubAssign, sub_assign);
// TODO: Drop, Index, IndexMut, Deref, DerefMut, others (e.g. Into?)
