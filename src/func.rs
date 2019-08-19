use crate::boolean::IfElse;
use crate::impl_func;
use std::marker::PhantomData;

pub trait CanApply<F> {
    type Output;
    fn apply(self) -> Self::Output;
}

pub trait Func<I> {
    type Output;
    fn call(input: I) -> Self::Output;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl<F, I> CanApply<F> for I
where
    F: Func<I>,
{
    type Output = <F as Func<I>>::Output;

    #[inline]
    fn apply(self) -> Self::Output {
        F::call(self)
    }
}

pub type Apply<F, I> = <I as CanApply<F>>::Output;
pub type ApplyIfElse<B, TrueFn, FalseFn, I> = Apply<IfElse<B, TrueFn, FalseFn>, I>;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Chain<F>(PhantomData<*const F>);

impl<F> Clone for Chain<F> {
    #[inline]
    fn clone(&self) -> Self {
        Self(PhantomData)
    }
}

impl<F> Copy for Chain<F> {}

pub fn chain<F>(_: F) -> Chain<F> {
    Chain(PhantomData)
}

impl<F, Fs, I> Func<I> for Chain<(F, Fs)>
where
    I: CanApply<F>,
    Apply<F, I>: CanApply<Chain<Fs>>,
{
    type Output = Apply<Chain<Fs>, Apply<F, I>>;

    #[inline]
    fn call(input: I) -> Self::Output {
        input.apply().apply()
    }
}

impl<I> Func<I> for Chain<()> {
    type Output = I;

    #[inline(always)]
    fn call(input: I) -> Self::Output {
        input
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Copy, Clone, Debug)]
pub struct Identity;

impl_func!(for Identity {
    #[inline]
    [T] |x: T| -> T { x }
});

////////////////////////////////////////////////////////////////////////////////////////////////////

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
